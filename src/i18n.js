// UI strings. The language follows the system locale: Czech for `cs`,
// English for everything else.

const STRINGS = {
    en: {
        'tab.play': 'Play',
        'tab.buttons': 'Buttons',
        'tab.tune': 'Fine-tune',
        'tabs.label': 'Sections',

        'chip.controller': 'Controller',
        'chip.start': 'Start',
        'chip.game': 'Game',
        'steps.label': 'Steps',
        'step.done': 'done',
        'step.current': 'now',

        'step1.title': 'Turn on your controller',
        'step1.text': 'Press the round button in the middle. It will show up here by itself.',
        'step1.waiting': 'Looking for a controller',
        'step2.title': 'Your controller is ready',
        'step2.text': 'Press Start, then go to your game.',
        'step2.button': 'Start',
        'step2.unnamed': 'Controller',
        'step3.title': 'Now switch to your game',
        'step3.text': 'The controller works as keyboard and mouse until you press Stop.',
        'step3.button': 'Stop',

        'buttons.title': 'What each button does',
        'buttons.text': 'Point at a button to find it on the controller. Tap a key to change it.',
        'buttons.diagramLabel': 'Controller picture',
        'buttons.changeLabel': 'Key for {button}',

        'tune.title': 'Fine-tune',
        'tune.text': 'For grown-ups: small tweaks if the controller feels too fast, too slow or too jumpy.',
        'tune.sensitivity': 'Look speed',
        'tune.sensitivity.low': 'Slow',
        'tune.sensitivity.high': 'Fast',
        'tune.threshold': 'How far to push the stick to walk',
        'tune.threshold.low': 'A little',
        'tune.threshold.high': 'All the way',
        'tune.deadzone': 'Ignore stick wobble',
        'tune.deadzone.low': 'Less',
        'tune.deadzone.high': 'More',
        'tune.reset': 'Put everything back',
        'tune.resetConfirm': 'Tap again to reset all buttons and settings',
        'tune.resetDone': 'Everything is back to the start.',

        'stick.left': 'Left stick',
        'stick.right': 'Right stick',
        'stick.walk': 'Walk',
        'stick.look': 'Look around',
        'stick.mouse': 'Mouse',

        'button.south': 'A · Cross',
        'button.east': 'B · Circle',
        'button.west': 'X · Square',
        'button.north': 'Y · Triangle',
        'button.left_bumper': 'LB · L1',
        'button.right_bumper': 'RB · R1',
        'button.left_trigger': 'LT · L2',
        'button.right_trigger': 'RT · R2',
        'button.start': 'Start · Options',
        'button.select': 'Back · Share',
        'button.left_thumb': 'Press left stick',
        'button.right_thumb': 'Press right stick',
        'button.dpad_up': 'Pad up',
        'button.dpad_down': 'Pad down',
        'button.dpad_left': 'Pad left',
        'button.dpad_right': 'Pad right',

        'action.south': 'Slot 5',
        'action.east': 'Jump',
        'action.west': 'Inventory',
        'action.north': 'Swap hands',
        'action.left_bumper': 'Next slot',
        'action.right_bumper': 'Previous slot',
        'action.left_trigger': 'Hit and dig',
        'action.right_trigger': 'Use and place',
        'action.start': 'Pause',
        'action.select': 'Player list',
        'action.left_thumb': 'Sprint',
        'action.right_thumb': 'Sneak',
        'action.dpad_up': 'Slot 1',
        'action.dpad_down': 'Drop item',
        'action.dpad_left': 'Slot 2',
        'action.dpad_right': 'Slot 3',

        'key.none': 'Nothing',
        'key.custom': '{key} (from file)',
        'key.Space': 'Space',
        'key.Escape': 'Esc',
        'key.Tab': 'Tab',
        'key.Enter': 'Enter',
        'key.Backspace': 'Backspace',
        'key.Shift': 'Shift',
        'key.Control': 'Ctrl',
        'key.Alt': 'Alt',
        'key.Up': 'Up',
        'key.Down': 'Down',
        'key.Left': 'Left',
        'key.Right': 'Right',
        'key.LeftClick': 'Left click',
        'key.RightClick': 'Right click',
        'key.MiddleClick': 'Middle click',
        'key.ScrollUp': 'Scroll up',
        'key.ScrollDown': 'Scroll down',

        'group.keyboard': 'Keyboard',
        'group.letters': 'Letters',
        'group.numbers': 'Numbers',
        'group.function': 'Function keys',
        'group.arrows': 'Arrows',
        'group.mouse': 'Mouse',
        'group.scroll': 'Scroll wheel',

        'error.status': 'Cannot reach the controller service. Try closing and opening the app.',
        'error.load': 'Could not load your settings. Try closing and opening the app.',
        'error.toggle': 'Could not start. Check that the controller is on and try again.',
        'error.save': 'Could not save the change. Try again.',
        'error.reset': 'Could not put everything back. Try again.'
    },
    cs: {
        'tab.play': 'Hrát',
        'tab.buttons': 'Tlačítka',
        'tab.tune': 'Doladění',
        'tabs.label': 'Sekce',

        'chip.controller': 'Ovladač',
        'chip.start': 'Start',
        'chip.game': 'Hra',
        'steps.label': 'Kroky',
        'step.done': 'hotovo',
        'step.current': 'teď',

        'step1.title': 'Zapni ovladač',
        'step1.text': 'Zmáčkni kulaté tlačítko uprostřed. Objeví se tady samo.',
        'step1.waiting': 'Hledám ovladač',
        'step2.title': 'Ovladač je připravený',
        'step2.text': 'Zmáčkni Start a pak přepni do hry.',
        'step2.button': 'Start',
        'step2.unnamed': 'Ovladač',
        'step3.title': 'Teď přepni do hry',
        'step3.text': 'Ovladač funguje jako klávesnice a myš, dokud nezmáčkneš Stop.',
        'step3.button': 'Stop',

        'buttons.title': 'Co dělá které tlačítko',
        'buttons.text': 'Ukaž na tlačítko a najdeš ho na ovladači. Klepnutím na klávesu ji změníš.',
        'buttons.diagramLabel': 'Obrázek ovladače',
        'buttons.changeLabel': 'Klávesa pro {button}',

        'tune.title': 'Doladění',
        'tune.text': 'Pro dospělé: drobné úpravy, když je ovladač moc rychlý, pomalý nebo cuká.',
        'tune.sensitivity': 'Rychlost rozhlížení',
        'tune.sensitivity.low': 'Pomalu',
        'tune.sensitivity.high': 'Rychle',
        'tune.threshold': 'Jak daleko zatlačit páčku pro chůzi',
        'tune.threshold.low': 'Trochu',
        'tune.threshold.high': 'Až na doraz',
        'tune.deadzone': 'Nevšímat si chvění páčky',
        'tune.deadzone.low': 'Méně',
        'tune.deadzone.high': 'Více',
        'tune.reset': 'Vrátit vše zpět',
        'tune.resetConfirm': 'Klepni znovu a vrátíš všechna tlačítka i nastavení',
        'tune.resetDone': 'Vše je zase jako na začátku.',

        'stick.left': 'Levá páčka',
        'stick.right': 'Pravá páčka',
        'stick.walk': 'Chůze',
        'stick.look': 'Rozhlížení',
        'stick.mouse': 'Myš',

        'button.south': 'A · Křížek',
        'button.east': 'B · Kolečko',
        'button.west': 'X · Čtverec',
        'button.north': 'Y · Trojúhelník',
        'button.left_bumper': 'LB · L1',
        'button.right_bumper': 'RB · R1',
        'button.left_trigger': 'LT · L2',
        'button.right_trigger': 'RT · R2',
        'button.start': 'Start · Options',
        'button.select': 'Back · Share',
        'button.left_thumb': 'Stisk levé páčky',
        'button.right_thumb': 'Stisk pravé páčky',
        'button.dpad_up': 'Kříž nahoru',
        'button.dpad_down': 'Kříž dolů',
        'button.dpad_left': 'Kříž doleva',
        'button.dpad_right': 'Kříž doprava',

        'action.south': 'Políčko 5',
        'action.east': 'Skok',
        'action.west': 'Inventář',
        'action.north': 'Prohodit ruce',
        'action.left_bumper': 'Další políčko',
        'action.right_bumper': 'Předchozí políčko',
        'action.left_trigger': 'Útok a kopání',
        'action.right_trigger': 'Použít a položit',
        'action.start': 'Pauza',
        'action.select': 'Seznam hráčů',
        'action.left_thumb': 'Běh',
        'action.right_thumb': 'Plížení',
        'action.dpad_up': 'Políčko 1',
        'action.dpad_down': 'Zahodit věc',
        'action.dpad_left': 'Políčko 2',
        'action.dpad_right': 'Políčko 3',

        'key.none': 'Nic',
        'key.custom': '{key} (ze souboru)',
        'key.Space': 'Mezerník',
        'key.Escape': 'Esc',
        'key.Tab': 'Tab',
        'key.Enter': 'Enter',
        'key.Backspace': 'Backspace',
        'key.Shift': 'Shift',
        'key.Control': 'Ctrl',
        'key.Alt': 'Alt',
        'key.Up': 'Nahoru',
        'key.Down': 'Dolů',
        'key.Left': 'Doleva',
        'key.Right': 'Doprava',
        'key.LeftClick': 'Levé tlačítko',
        'key.RightClick': 'Pravé tlačítko',
        'key.MiddleClick': 'Prostřední tlačítko',
        'key.ScrollUp': 'Kolečko nahoru',
        'key.ScrollDown': 'Kolečko dolů',

        'group.keyboard': 'Klávesnice',
        'group.letters': 'Písmena',
        'group.numbers': 'Číslice',
        'group.function': 'Funkční klávesy',
        'group.arrows': 'Šipky',
        'group.mouse': 'Myš',
        'group.scroll': 'Kolečko myši',

        'error.status': 'Nedaří se spojit s ovladačem. Zkus aplikaci zavřít a znovu otevřít.',
        'error.load': 'Nastavení se nepodařilo načíst. Zkus aplikaci zavřít a znovu otevřít.',
        'error.toggle': 'Nepodařilo se spustit. Zkontroluj, že je ovladač zapnutý, a zkus to znovu.',
        'error.save': 'Změnu se nepodařilo uložit. Zkus to znovu.',
        'error.reset': 'Nepodařilo se vrátit vše zpět. Zkus to znovu.'
    }
};

const detectLanguage = () => {
    const preferred = navigator.languages?.[0] ?? navigator.language ?? 'en';
    return preferred.toLowerCase().startsWith('cs') ? 'cs' : 'en';
};

export const language = detectLanguage();

/** Returns the string for `key`, filling `{name}` placeholders from `values`. */
export const t = (key, values = {}) => {
    const template = STRINGS[language][key] ?? STRINGS.en[key];
    if (template === undefined) {
        throw new Error(`Missing UI string: ${key}`);
    }
    return template.replace(/\{(\w+)\}/g, (_, name) => String(values[name] ?? ''));
};

/** Returns the string for `key` when it exists, otherwise `null`. */
export const maybeT = (key) => STRINGS[language][key] ?? STRINGS.en[key] ?? null;

/** Fills every `data-i18n` element and `data-i18n-aria-label` attribute. */
export const translateDocument = (root = document) => {
    document.documentElement.lang = language;
    for (const element of root.querySelectorAll('[data-i18n]')) {
        element.textContent = t(element.dataset.i18n);
    }
    for (const element of root.querySelectorAll('[data-i18n-aria-label]')) {
        element.setAttribute('aria-label', t(element.dataset.i18nAriaLabel));
    }
};
