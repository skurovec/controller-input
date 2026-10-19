const { invoke } = window.__TAURI__.core;

// DOM Elements
const controllerIndicator = document.getElementById('controllerIndicator');
const controllerStatus = document.getElementById('controllerStatus');
const mappingIndicator = document.getElementById('mappingIndicator');
const mappingStatus = document.getElementById('mappingStatus');
const toggleBtn = document.getElementById('toggleBtn');
const deadzoneSlider = document.getElementById('deadzone');
const deadzoneValue = document.getElementById('deadzoneValue');
const sensitivitySlider = document.getElementById('sensitivity');
const sensitivityValue = document.getElementById('sensitivityValue');
const thresholdSlider = document.getElementById('threshold');
const thresholdValue = document.getElementById('thresholdValue');
const resetBtn = document.getElementById('resetBtn');
const buttonMappingsGrid = document.getElementById('buttonMappingsGrid');

const BUTTON_LABELS = [
    { key: 'south', label: 'A / Cross' },
    { key: 'east', label: 'B / Circle' },
    { key: 'west', label: 'X / Square' },
    { key: 'north', label: 'Y / Triangle' },
    { key: 'left_bumper', label: 'LB / L1' },
    { key: 'right_bumper', label: 'RB / R1' },
    { key: 'left_trigger', label: 'LT / L2' },
    { key: 'right_trigger', label: 'RT / R2' },
    { key: 'start', label: 'Start' },
    { key: 'select', label: 'Select / Back' },
    { key: 'left_thumb', label: 'L3 (Left Click)' },
    { key: 'right_thumb', label: 'R3 (Right Click)' },
    { key: 'dpad_up', label: 'D-Pad Up' },
    { key: 'dpad_down', label: 'D-Pad Down' },
    { key: 'dpad_left', label: 'D-Pad Left' },
    { key: 'dpad_right', label: 'D-Pad Right' }
];

const ACTION_OPTIONS = [
    { group: null, options: [{ value: '', label: 'None' }] },
    {
        group: 'Keyboard',
        options: [
            { value: 'Space', label: 'Space' },
            { value: 'Escape', label: 'Escape' },
            { value: 'Tab', label: 'Tab' },
            { value: 'Enter', label: 'Enter' },
            { value: 'Backspace', label: 'Backspace' },
            { value: 'Shift', label: 'Shift' },
            { value: 'Control', label: 'Ctrl' },
            { value: 'Alt', label: 'Alt' }
        ]
    },
    {
        group: 'Letters',
        options: 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'.split('').map((letter) => ({
            value: letter,
            label: letter
        }))
    },
    {
        group: 'Numbers',
        options: '0123456789'.split('').map((digit) => ({ value: digit, label: digit }))
    },
    {
        group: 'Function',
        options: Array.from({ length: 12 }, (_, index) => {
            const name = `F${index + 1}`;
            return { value: name, label: name };
        })
    },
    {
        group: 'Arrows',
        options: [
            { value: 'Up', label: 'Up' },
            { value: 'Down', label: 'Down' },
            { value: 'Left', label: 'Left' },
            { value: 'Right', label: 'Right' }
        ]
    },
    {
        group: 'Mouse',
        options: [
            { value: 'LeftClick', label: 'Left Click' },
            { value: 'RightClick', label: 'Right Click' },
            { value: 'MiddleClick', label: 'Middle Click' }
        ]
    },
    {
        group: 'Scroll',
        options: [
            { value: 'ScrollUp', label: 'Scroll Up' },
            { value: 'ScrollDown', label: 'Scroll Down' }
        ]
    }
];

const ALL_ACTION_VALUES = new Set(
    ACTION_OPTIONS.flatMap((group) => group.options.map((option) => option.value))
);

// State
let isRunning = false;
let controllerConnected = false;

// Initialize
const init = async () => {
    await loadConfig();
    await updateStatus();

    // Poll status every 500ms
    setInterval(updateStatus, 500);
};

// Load config from backend
const loadConfig = async () => {
    try {
        const config = await invoke('get_config');
        applyConfigToUI(config);
    } catch (e) {
        console.error('Failed to load config:', e);
        alert('Failed to load configuration. See console for details.');
    }
};

const applyConfigToUI = (config) => {
    deadzoneSlider.value = config.settings.deadzone;
    deadzoneValue.textContent = config.settings.deadzone.toFixed(2);
    sensitivitySlider.value = config.settings.mouse_sensitivity;
    sensitivityValue.textContent = config.settings.mouse_sensitivity.toFixed(1);
    thresholdSlider.value = config.settings.stick_to_key_threshold;
    thresholdValue.textContent = config.settings.stick_to_key_threshold.toFixed(2);
    renderButtonMappings(config.buttons || {});
};

const buildActionSelect = (buttonKey, currentValue) => {
    const select = document.createElement('select');
    select.className = 'mapping-select';
    select.dataset.button = buttonKey;

    for (const group of ACTION_OPTIONS) {
        const parent = group.group
            ? (() => {
                  const optgroup = document.createElement('optgroup');
                  optgroup.label = group.group;
                  return optgroup;
              })()
            : select;

        for (const opt of group.options) {
            const optionElement = document.createElement('option');
            optionElement.value = opt.value;
            optionElement.textContent = opt.label;
            parent.appendChild(optionElement);
        }
        if (group.group) {
            select.appendChild(parent);
        }
    }

    const normalized = currentValue == null ? '' : currentValue;
    if (normalized !== '' && !ALL_ACTION_VALUES.has(normalized)) {
        // Unknown value coming from config — show it so user can see what is set
        const customOption = document.createElement('option');
        customOption.value = normalized;
        customOption.textContent = `${normalized} (custom)`;
        select.insertBefore(customOption, select.firstChild);
    }
    select.value = normalized;

    select.addEventListener('change', async () => {
        try {
            await invoke('update_button_mapping', {
                button: buttonKey,
                action: select.value === '' ? null : select.value
            });
        } catch (e) {
            console.error('Failed to update button mapping:', e);
            alert(`Failed to update mapping for ${buttonKey}: ${e}`);
        }
    });

    return select;
};

const renderButtonMappings = (buttonsMap) => {
    buttonMappingsGrid.innerHTML = '';
    for (const { key, label } of BUTTON_LABELS) {
        const row = document.createElement('div');
        row.className = 'button-mapping-row';

        const labelEl = document.createElement('span');
        labelEl.className = 'mapping-input';
        labelEl.textContent = label;

        const arrow = document.createElement('span');
        arrow.className = 'mapping-arrow';
        arrow.textContent = '→';

        const select = buildActionSelect(key, buttonsMap[key]);

        row.appendChild(labelEl);
        row.appendChild(arrow);
        row.appendChild(select);
        buttonMappingsGrid.appendChild(row);
    }
};

// Update status from backend
const updateStatus = async () => {
    try {
        const status = await invoke('get_status');
        isRunning = status.is_running;
        controllerConnected = status.controller_connected;
        updateUI(status.controller_name);
    } catch (e) {
        console.error('Failed to get status:', e);
    }
};

// Update UI based on state
const updateUI = (controllerName = null) => {
    // Controller status
    if (controllerConnected) {
        controllerIndicator.classList.add('connected');
        controllerStatus.textContent = controllerName || 'Connected';
        toggleBtn.disabled = false;
    } else {
        controllerIndicator.classList.remove('connected');
        controllerStatus.textContent = 'Not connected';
        toggleBtn.disabled = true;
    }

    // Mapping status
    if (isRunning) {
        mappingIndicator.classList.add('active');
        mappingStatus.textContent = 'Active';
        toggleBtn.classList.add('active');
        toggleBtn.querySelector('.btn-text').textContent = 'Stop Mapping';
    } else {
        mappingIndicator.classList.remove('active');
        mappingStatus.textContent = 'Inactive';
        toggleBtn.classList.remove('active');
        toggleBtn.querySelector('.btn-text').textContent = 'Start Mapping';
    }
};

// Event Listeners
toggleBtn.addEventListener('click', async () => {
    try {
        const newState = await invoke('toggle_mapping');
        isRunning = newState;
        updateUI();
    } catch (e) {
        console.error('Failed to toggle mapping:', e);
    }
});

// Debounce function for sliders
const debounce = (func, wait) => {
    let timeout;
    return (...args) => {
        clearTimeout(timeout);
        timeout = setTimeout(() => func.apply(this, args), wait);
    };
};

// Save settings with debounce
const saveSettings = debounce(async () => {
    try {
        await invoke('update_settings', {
            deadzone: parseFloat(deadzoneSlider.value),
            mouseSensitivity: parseFloat(sensitivitySlider.value),
            stickToKeyThreshold: parseFloat(thresholdSlider.value)
        });
    } catch (e) {
        console.error('Failed to save settings:', e);
    }
}, 300);

deadzoneSlider.addEventListener('input', () => {
    deadzoneValue.textContent = parseFloat(deadzoneSlider.value).toFixed(2);
    saveSettings();
});

sensitivitySlider.addEventListener('input', () => {
    sensitivityValue.textContent = parseFloat(sensitivitySlider.value).toFixed(1);
    saveSettings();
});

thresholdSlider.addEventListener('input', () => {
    thresholdValue.textContent = parseFloat(thresholdSlider.value).toFixed(2);
    saveSettings();
});

resetBtn.addEventListener('click', async () => {
    try {
        const config = await invoke('reset_config');
        applyConfigToUI(config);
    } catch (e) {
        console.error('Failed to reset config:', e);
        alert('Failed to reset configuration. See console for details.');
    }
});

// Start the app
init();
