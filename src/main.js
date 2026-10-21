import { t, maybeT, translateDocument } from './i18n.js';

const { invoke } = window.__TAURI__.core;

const STATUS_POLL_MS = 500;
const NOTICE_MS = 6000;
const RESET_CONFIRM_MS = 4000;

// Magnifier crops into the controller drawing (viewBox units), one per region.
const CROPS = {
    face: { x: 230, y: 82, size: 74 },
    pad: { x: 124, y: 128, size: 74 },
    leftShoulder: { x: 92, y: 44, size: 112 },
    rightShoulder: { x: 228, y: 44, size: 112 },
    leftStick: { x: 90, y: 82, size: 66 },
    rightStick: { x: 196, y: 128, size: 62 },
    middle: { x: 160, y: 76, size: 78 }
};

// Rows of the Buttons page, grouped by controller region.
const BUTTONS = [
    { key: 'right_trigger', crop: 'rightShoulder' },
    { key: 'left_trigger', crop: 'leftShoulder' },
    { key: 'right_bumper', crop: 'rightShoulder' },
    { key: 'left_bumper', crop: 'leftShoulder' },
    { key: 'south', crop: 'face' },
    { key: 'east', crop: 'face' },
    { key: 'west', crop: 'face' },
    { key: 'north', crop: 'face' },
    { key: 'left_thumb', crop: 'leftStick' },
    { key: 'right_thumb', crop: 'rightStick' },
    { key: 'dpad_up', crop: 'pad' },
    { key: 'dpad_down', crop: 'pad' },
    { key: 'dpad_left', crop: 'pad' },
    { key: 'dpad_right', crop: 'pad' },
    { key: 'start', crop: 'middle' },
    { key: 'select', crop: 'middle' }
];

const KEY_GROUPS = [
    { label: 'group.keyboard', values: ['Space', 'Escape', 'Tab', 'Enter', 'Backspace', 'Shift', 'Control', 'Alt'] },
    { label: 'group.letters', values: 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'.split('') },
    { label: 'group.numbers', values: '0123456789'.split('') },
    { label: 'group.function', values: Array.from({ length: 12 }, (_, index) => `F${index + 1}`) },
    { label: 'group.arrows', values: ['Up', 'Down', 'Left', 'Right'] },
    { label: 'group.mouse', values: ['LeftClick', 'RightClick', 'MiddleClick'] },
    { label: 'group.scroll', values: ['ScrollUp', 'ScrollDown'] }
];

const KNOWN_KEYS = new Set(KEY_GROUPS.flatMap((group) => group.values));
const MOUSE_KEYS = new Set(['LeftClick', 'RightClick', 'MiddleClick', 'ScrollUp', 'ScrollDown']);

const elements = {
    tabs: [...document.querySelectorAll('.tab')],
    pages: [...document.querySelectorAll('.page')],
    sheet: document.getElementById('sheet'),
    stage: document.getElementById('stage'),
    panels: [...document.querySelectorAll('.stage__panel')],
    chips: [...document.querySelectorAll('.step-chip')],
    deviceName: document.getElementById('deviceName'),
    startButton: document.getElementById('startButton'),
    stopButton: document.getElementById('stopButton'),
    diagram: document.getElementById('diagram'),
    partsList: document.getElementById('partsList'),
    resetButton: document.getElementById('resetButton'),
    resetLabel: document.getElementById('resetLabel'),
    notice: document.getElementById('notice'),
    sliders: {
        sensitivity: document.getElementById('sensitivity'),
        threshold: document.getElementById('threshold'),
        deadzone: document.getElementById('deadzone')
    },
    sliderValues: {
        sensitivity: document.getElementById('sensitivityValue'),
        threshold: document.getElementById('thresholdValue'),
        deadzone: document.getElementById('deadzoneValue')
    }
};

const state = {
    step: 0,
    defaults: { buttons: {} },
    statusFailing: false,
    busy: false
};

// ---------------------------------------------------------------- notices

let noticeTimer;

const showNotice = (message, tone = 'error') => {
    clearTimeout(noticeTimer);
    elements.notice.textContent = message;
    elements.notice.dataset.tone = tone;
    elements.notice.classList.add('is-visible');
    noticeTimer = setTimeout(() => elements.notice.classList.remove('is-visible'), NOTICE_MS);
};

const reportError = (messageKey, error) => {
    console.error(`${messageKey}:`, error);
    showNotice(t(messageKey));
};

// ---------------------------------------------------------------- controller art

const controllerTemplate = document.getElementById('controllerTemplate');

const createController = () => controllerTemplate.content.firstElementChild.cloneNode(true);

const createMagnifier = (buttonKey, cropName) => {
    const crop = CROPS[cropName];
    const svg = createController();
    const half = crop.size / 2;
    svg.setAttribute('viewBox', `${crop.x - half} ${crop.y - half} ${crop.size} ${crop.size}`);
    svg.querySelector(`[data-button="${buttonKey}"]`)?.classList.add('is-lit');

    const magnifier = document.createElement('span');
    magnifier.className = 'magnifier';
    magnifier.append(svg);
    return magnifier;
};

for (const slot of document.querySelectorAll('[data-art="controller"]')) {
    slot.prepend(createController());
}

// ---------------------------------------------------------------- tabs

const selectTab = (tab, { focus = false } = {}) => {
    for (const other of elements.tabs) {
        const selected = other === tab;
        other.setAttribute('aria-selected', String(selected));
        other.tabIndex = selected ? 0 : -1;
    }
    for (const page of elements.pages) {
        page.hidden = page.id !== tab.getAttribute('aria-controls');
    }
    document.body.dataset.page = tab.dataset.page;
    elements.sheet.scrollTop = 0;
    if (focus) {
        tab.focus();
    }
};

for (const tab of elements.tabs) {
    tab.addEventListener('click', () => selectTab(tab));
    tab.addEventListener('keydown', (event) => {
        const index = elements.tabs.indexOf(tab);
        const last = elements.tabs.length - 1;
        const target = {
            ArrowRight: elements.tabs[index === last ? 0 : index + 1],
            ArrowLeft: elements.tabs[index === 0 ? last : index - 1],
            Home: elements.tabs[0],
            End: elements.tabs[last]
        }[event.key];
        if (target) {
            event.preventDefault();
            selectTab(target, { focus: true });
        }
    });
}

// ---------------------------------------------------------------- play steps

const stepFor = (status) => {
    if (!status.controller_connected) {
        return 1;
    }
    return status.is_running ? 3 : 2;
};

const renderStep = (step) => {
    if (step === state.step) {
        return;
    }
    const firstRender = state.step === 0;
    state.step = step;
    elements.stage.dataset.step = String(step);
    elements.stage.classList.toggle('is-settled', firstRender);

    for (const panel of elements.panels) {
        const active = Number(panel.dataset.panel) === step;
        panel.classList.toggle('is-active', active);
        panel.inert = !active;
    }
    for (const chip of elements.chips) {
        const chipStep = Number(chip.dataset.step);
        chip.classList.toggle('is-done', chipStep < step);
        chip.classList.toggle('is-current', chipStep === step);
        chip.querySelector('.step-chip__state').textContent =
            chipStep < step ? t('step.done') : chipStep === step ? t('step.current') : '';
        if (chipStep === step) {
            chip.setAttribute('aria-current', 'step');
        } else {
            chip.removeAttribute('aria-current');
        }
    }
};

const applyStatus = (status) => {
    elements.deviceName.textContent = status.controller_name || t('step2.unnamed');
    renderStep(stepFor(status));
};

const updateStatus = async () => {
    try {
        applyStatus(await invoke('get_status'));
        if (state.statusFailing) {
            state.statusFailing = false;
            elements.notice.classList.remove('is-visible');
        }
    } catch (error) {
        if (!state.statusFailing) {
            state.statusFailing = true;
            reportError('error.status', error);
        }
    }
};

const runMappingCommand = async (button, command) => {
    if (state.busy) {
        return;
    }
    state.busy = true;
    button.setAttribute('aria-busy', 'true');
    try {
        await invoke(command);
        applyStatus(await invoke('get_status'));
    } catch (error) {
        reportError('error.toggle', error);
    } finally {
        state.busy = false;
        button.removeAttribute('aria-busy');
    }
};

elements.startButton.addEventListener('click', () => runMappingCommand(elements.startButton, 'start_mapping'));
elements.stopButton.addEventListener('click', () => runMappingCommand(elements.stopButton, 'stop_mapping'));

// ---------------------------------------------------------------- buttons page

const keyLabel = (value) => {
    if (!value) {
        return t('key.none');
    }
    return maybeT(`key.${value}`) ?? value;
};

const createKeySelect = (buttonKey, currentValue) => {
    const select = document.createElement('select');
    select.className = 'keycap__select';
    select.setAttribute('aria-label', t('buttons.changeLabel', { button: t(`button.${buttonKey}`) }));

    const none = new Option(t('key.none'), '');
    select.append(none);

    if (currentValue && !KNOWN_KEYS.has(currentValue)) {
        // Keep values written by hand into config.toml visible and selectable.
        select.append(new Option(t('key.custom', { key: currentValue }), currentValue));
    }

    for (const group of KEY_GROUPS) {
        const optgroup = document.createElement('optgroup');
        optgroup.label = t(group.label);
        for (const value of group.values) {
            optgroup.append(new Option(keyLabel(value), value));
        }
        select.append(optgroup);
    }

    select.value = currentValue ?? '';
    return select;
};

const renderPartText = (row, buttonKey, value) => {
    const isDefault = value != null && value === state.defaults.buttons[buttonKey];
    const action = row.querySelector('.part-row__action');
    const name = row.querySelector('.part-row__name');
    action.textContent = isDefault ? t(`action.${buttonKey}`) : t(`button.${buttonKey}`);
    name.textContent = isDefault ? t(`button.${buttonKey}`) : '';

    const keycap = row.querySelector('.keycap');
    keycap.querySelector('.keycap__label').textContent = keyLabel(value);
    keycap.classList.toggle('keycap--mouse', MOUSE_KEYS.has(value));
    keycap.classList.toggle('keycap--empty', !value);
};

const createMouseIcon = () => {
    const icon = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    icon.setAttribute('viewBox', '0 0 24 24');
    icon.setAttribute('aria-hidden', 'true');
    icon.classList.add('keycap__mouse');
    const body = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
    for (const [name, value] of Object.entries({ x: 6, y: 3, width: 12, height: 18, rx: 6 })) {
        body.setAttribute(name, String(value));
    }
    const divider = document.createElementNS('http://www.w3.org/2000/svg', 'path');
    divider.setAttribute('d', 'M12 3v7M6 10h12');
    icon.append(body, divider);
    return icon;
};

const createPartRow = (buttonKey, cropName, value) => {
    const row = document.createElement('li');
    row.className = 'part-row';
    row.dataset.button = buttonKey;

    const text = document.createElement('span');
    text.className = 'part-row__text';
    const action = document.createElement('span');
    action.className = 'part-row__action';
    const name = document.createElement('span');
    name.className = 'part-row__name';
    text.append(action, name);

    const keycap = document.createElement('label');
    keycap.className = 'keycap';
    const label = document.createElement('span');
    label.className = 'keycap__label';
    const chevron = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    chevron.setAttribute('viewBox', '0 0 24 24');
    chevron.setAttribute('aria-hidden', 'true');
    chevron.classList.add('keycap__chevron');
    const chevronPath = document.createElementNS('http://www.w3.org/2000/svg', 'path');
    chevronPath.setAttribute('d', 'M7 10l5 5 5-5');
    chevron.append(chevronPath);

    const select = createKeySelect(buttonKey, value);
    keycap.append(createMouseIcon(), label, chevron, select);

    select.addEventListener('change', async () => {
        const nextValue = select.value;
        renderPartText(row, buttonKey, nextValue);
        try {
            await invoke('update_button_mapping', {
                button: buttonKey,
                action: nextValue === '' ? null : nextValue
            });
        } catch (error) {
            reportError('error.save', error);
            // A failed save may still have changed the live mapping, so show
            // what the backend actually holds rather than the previous value.
            try {
                const config = await invoke('get_config');
                const activeValue = config.buttons?.[buttonKey] ?? '';
                select.value = activeValue;
                renderPartText(row, buttonKey, activeValue);
            } catch (reloadError) {
                reportError('error.load', reloadError);
            }
        }
    });

    row.append(createMagnifier(buttonKey, cropName), text, keycap);
    renderPartText(row, buttonKey, value);
    return row;
};

const createStickRow = (stickName, cropName, actionKey, keys) => {
    const row = document.createElement('li');
    row.className = 'part-row part-row--fixed';
    row.dataset.button = stickName;

    const text = document.createElement('span');
    text.className = 'part-row__text';
    const action = document.createElement('span');
    action.className = 'part-row__action';
    action.textContent = t(actionKey);
    const name = document.createElement('span');
    name.className = 'part-row__name';
    name.textContent = t(`stick.${stickName === 'left_stick' ? 'left' : 'right'}`);
    text.append(action, name);

    const cluster = document.createElement('span');
    cluster.className = 'keycap-cluster';
    for (const key of keys) {
        const cap = document.createElement('span');
        cap.className = 'keycap keycap--fixed';
        if (key === 'mouse') {
            cap.classList.add('keycap--mouse');
            const label = document.createElement('span');
            label.className = 'keycap__label';
            label.textContent = t('stick.mouse');
            cap.append(createMouseIcon(), label);
        } else {
            cap.textContent = key;
        }
        cluster.append(cap);
    }

    const stickButton = stickName === 'left_stick' ? 'left_thumb' : 'right_thumb';
    row.append(createMagnifier(stickButton, cropName), text, cluster);
    return row;
};

const renderParts = (buttons) => {
    elements.partsList.replaceChildren(
        createStickRow('left_stick', 'leftStick', 'stick.walk', ['W', 'A', 'S', 'D']),
        createStickRow('right_stick', 'rightStick', 'stick.look', ['mouse']),
        ...BUTTONS.map(({ key, crop }) => createPartRow(key, crop, buttons[key]))
    );
};

// Diagram and rows point at each other: hovering either lights both.
const diagramButtonFor = (rowButton) => {
    const target = { left_stick: 'left_thumb', right_stick: 'right_thumb' }[rowButton] ?? rowButton;
    return elements.diagram.querySelector(`[data-button="${target}"]`);
};

const setPointed = (rowButton, pointed) => {
    diagramButtonFor(rowButton)?.classList.toggle('is-lit', pointed);
    elements.partsList
        .querySelector(`.part-row[data-button="${rowButton}"]`)
        ?.classList.toggle('is-pointed', pointed);
};

for (const eventName of ['pointerover', 'focusin']) {
    elements.partsList.addEventListener(eventName, (event) => {
        const row = event.target.closest('.part-row');
        if (row) {
            setPointed(row.dataset.button, true);
        }
    });
}
for (const eventName of ['pointerout', 'focusout']) {
    elements.partsList.addEventListener(eventName, (event) => {
        const row = event.target.closest('.part-row');
        if (row && !row.contains(event.relatedTarget)) {
            setPointed(row.dataset.button, false);
        }
    });
}

elements.diagram.addEventListener('pointerover', (event) => {
    const part = event.target.closest('[data-button]');
    if (part) {
        setPointed(part.dataset.button, true);
    }
});
elements.diagram.addEventListener('pointerout', (event) => {
    const part = event.target.closest('[data-button]');
    if (part) {
        setPointed(part.dataset.button, false);
    }
});
elements.diagram.addEventListener('click', (event) => {
    const part = event.target.closest('[data-button]');
    const row = part && elements.partsList.querySelector(`.part-row[data-button="${part.dataset.button}"]`);
    if (row) {
        row.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
        row.querySelector('select')?.focus({ preventScroll: true });
    }
});

// ---------------------------------------------------------------- fine-tune

const SLIDER_DIGITS = { sensitivity: 1, threshold: 2, deadzone: 2 };

const showSliderValue = (name) => {
    const value = Number.parseFloat(elements.sliders[name].value);
    elements.sliderValues[name].textContent = value.toFixed(SLIDER_DIGITS[name]);
};

let saveTimer;

const saveSettings = () => {
    clearTimeout(saveTimer);
    saveTimer = setTimeout(async () => {
        try {
            await invoke('update_settings', {
                deadzone: Number.parseFloat(elements.sliders.deadzone.value),
                mouseSensitivity: Number.parseFloat(elements.sliders.sensitivity.value),
                stickToKeyThreshold: Number.parseFloat(elements.sliders.threshold.value)
            });
        } catch (error) {
            reportError('error.save', error);
        }
    }, 300);
};

for (const name of Object.keys(elements.sliders)) {
    elements.sliders[name].addEventListener('input', () => {
        showSliderValue(name);
        saveSettings();
    });
}

let resetArmedTimer;

const disarmReset = () => {
    clearTimeout(resetArmedTimer);
    elements.resetButton.classList.remove('is-armed');
    elements.resetLabel.textContent = t('tune.reset');
};

elements.resetButton.addEventListener('click', async () => {
    if (!elements.resetButton.classList.contains('is-armed')) {
        elements.resetButton.classList.add('is-armed');
        elements.resetLabel.textContent = t('tune.resetConfirm');
        resetArmedTimer = setTimeout(disarmReset, RESET_CONFIRM_MS);
        return;
    }
    disarmReset();
    try {
        applyConfig(await invoke('reset_config'));
        showNotice(t('tune.resetDone'), 'success');
    } catch (error) {
        reportError('error.reset', error);
    }
});

// ---------------------------------------------------------------- config

const applyConfig = (config) => {
    elements.sliders.deadzone.value = config.settings.deadzone;
    elements.sliders.sensitivity.value = config.settings.mouse_sensitivity;
    elements.sliders.threshold.value = config.settings.stick_to_key_threshold;
    for (const name of Object.keys(elements.sliders)) {
        showSliderValue(name);
    }
    renderParts(config.buttons ?? {});
};

const loadConfig = async () => {
    try {
        const [config, defaults] = await Promise.all([invoke('get_config'), invoke('get_default_config')]);
        state.defaults = defaults;
        applyConfig(config);
    } catch (error) {
        renderParts({});
        reportError('error.load', error);
    }
};

// ---------------------------------------------------------------- start

const init = async () => {
    translateDocument();
    selectTab(elements.tabs[0]);
    await loadConfig();
    await updateStatus();
    if (state.step === 0) {
        renderStep(1);
    }
    setInterval(updateStatus, STATUS_POLL_MS);
};

init();
