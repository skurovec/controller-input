const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

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

// State
let isRunning = false;
let controllerConnected = false;

// Initialize
const init = async () => {
    await loadConfig();
    await updateStatus();

    // Poll status every 500ms
    setInterval(updateStatus, 500);

    // Listen for status changes from tray
    listen('status-changed', (event) => {
        isRunning = event.payload;
        updateUI();
    });
};

// Load config from backend
const loadConfig = async () => {
    try {
        const config = await invoke('get_config');
        deadzoneSlider.value = config.settings.deadzone;
        deadzoneValue.textContent = config.settings.deadzone.toFixed(2);
        sensitivitySlider.value = config.settings.mouse_sensitivity;
        sensitivityValue.textContent = config.settings.mouse_sensitivity.toFixed(1);
        thresholdSlider.value = config.settings.stick_to_key_threshold;
        thresholdValue.textContent = config.settings.stick_to_key_threshold.toFixed(2);
    } catch (e) {
        console.error('Failed to load config:', e);
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
        deadzoneSlider.value = config.settings.deadzone;
        deadzoneValue.textContent = config.settings.deadzone.toFixed(2);
        sensitivitySlider.value = config.settings.mouse_sensitivity;
        sensitivityValue.textContent = config.settings.mouse_sensitivity.toFixed(1);
        thresholdSlider.value = config.settings.stick_to_key_threshold;
        thresholdValue.textContent = config.settings.stick_to_key_threshold.toFixed(2);
    } catch (e) {
        console.error('Failed to reset config:', e);
    }
});

// Start the app
init();
