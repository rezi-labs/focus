/**
 * Font Switcher
 * Provides functionality to change fonts dynamically
 */

// Available fonts - curated selection
const AVAILABLE_FONTS = [
    { value: 'system', name: 'System', stack: '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif' },
    { value: 'serif', name: 'Serif', stack: 'Georgia, "Times New Roman", Times, serif' },
    { value: 'mono', name: 'Mono', stack: '"SF Mono", Monaco, "Cascadia Code", "Roboto Mono", Consolas, "Courier New", monospace' },
    { value: 'classic', name: 'Classic', stack: 'Garamond, Baskerville, "Baskerville Old Face", "Hoefler Text", "Times New Roman", serif' },
    { value: 'reader', name: 'Reader', stack: '"Charter", "Bitstream Charter", "Sitka Text", Cambria, serif' }
];

/**
 * Changes the current font
 * @param {string} fontValue - The font value to apply
 * @param {boolean} persist - Whether to save the font to localStorage (default: true)
 * @returns {boolean} - True if font was applied successfully, false otherwise
 */
function changeFont(fontValue, persist = true) {
    // Validate font
    if (!fontValue || typeof fontValue !== 'string') {
        console.warn('Font switcher: Invalid font provided');
        return false;
    }

    const normalizedFont = fontValue.toLowerCase().trim();
    const font = AVAILABLE_FONTS.find(f => f.value === normalizedFont);

    if (!font) {
        console.warn(`Font switcher: Font "${fontValue}" is not available. Available fonts:`, AVAILABLE_FONTS.map(f => f.value));
        return false;
    }

    try {
        // Apply font to html element
        const htmlElement = document.documentElement;
        htmlElement.style.fontFamily = font.stack;
        
        // Also set on body for compatibility
        document.body.style.fontFamily = font.stack;

        // Update any font controller selects
        updateFontSelectors();

        // Persist font to localStorage if requested
        if (persist) {
            try {
                localStorage.setItem('focus-font', normalizedFont);
            } catch (storageError) {
                console.warn('Font switcher: Could not save font to localStorage:', storageError);
            }
        }

        // Dispatch custom event for other components to listen to
        const fontChangeEvent = new CustomEvent('fontChanged', {
            detail: { font: normalizedFont, fontStack: font.stack }
        });
        document.dispatchEvent(fontChangeEvent);

        return true;

    } catch (error) {
        console.error('Font switcher: Error changing font:', error);
        return false;
    }
}

/**
 * Gets the current active font
 * @returns {string} - The current font value
 */
function getCurrentFont() {
    try {
        return localStorage.getItem('focus-font') || 'system';
    } catch (error) {
        return 'system';
    }
}

/**
 * Loads the saved font from localStorage
 * @returns {string|null} - The saved font or null if not found
 */
function getSavedFont() {
    try {
        return localStorage.getItem('focus-font');
    } catch (error) {
        console.warn('Font switcher: Could not access localStorage:', error);
        return null;
    }
}

/**
 * Initializes the font system
 * Loads the saved font or applies a default font
 */
function initializeFont() {
    const savedFont = getSavedFont();
    const defaultFont = 'system';

    if (savedFont && AVAILABLE_FONTS.find(f => f.value === savedFont)) {
        changeFont(savedFont, false); // Don't persist since it's already saved
    } else {
        changeFont(defaultFont, true);
    }
    
    // Update select dropdowns to match current font
    setTimeout(updateFontSelectors, 100);
}

/**
 * Updates all font selector dropdowns to match the current font
 */
function updateFontSelectors() {
    const currentFont = getCurrentFont();
    const fontControllers = document.querySelectorAll('select.font-controller');
    fontControllers.forEach(controller => {
        controller.value = currentFont;
    });
}

/**
 * Gets all available fonts
 * @returns {Object[]} - Array of available font objects
 */
function getAvailableFonts() {
    return [...AVAILABLE_FONTS];
}

// Auto-initialize when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', function() {
        initializeFont();
        // Ensure functions are available globally
        window.changeFont = changeFont;
        window.getCurrentFont = getCurrentFont;
        window.initializeFont = initializeFont;
        window.getAvailableFonts = getAvailableFonts;
    });
} else {
    initializeFont();
    // Ensure functions are available globally
    window.changeFont = changeFont;
    window.getCurrentFont = getCurrentFont;
    window.initializeFont = initializeFont;
    window.getAvailableFonts = getAvailableFonts;
}

// Export functions for module usage (if using modules)
if (typeof module !== 'undefined' && module.exports) {
    module.exports = {
        changeFont,
        getCurrentFont,
        getSavedFont,
        initializeFont,
        getAvailableFonts,
        AVAILABLE_FONTS
    };
}

// Also make available globally
window.changeFont = changeFont;
window.getCurrentFont = getCurrentFont;
window.getSavedFont = getSavedFont;
window.initializeFont = initializeFont;
window.updateFontSelectors = updateFontSelectors;
window.getAvailableFonts = getAvailableFonts;
