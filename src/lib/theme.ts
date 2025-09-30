import { setTheme as tauriSetTheme } from '@tauri-apps/api/app';

export type Theme = 'light' | 'dark' | 'system';

// Simple theme manager class
class ThemeManager {
  private currentTheme: Theme = 'system';
  private currentResolvedTheme: 'light' | 'dark' = 'light';
  private listeners: ((theme: 'light' | 'dark') => void)[] = [];

  constructor() {
    this.updateResolvedTheme();
    this.listenForThemeChanges();
  }

  private updateResolvedTheme() {
    if (this.currentTheme === 'system') {
      // Use CSS media query to detect system preference
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      this.currentResolvedTheme = prefersDark ? 'dark' : 'light';
    } else {
      this.currentResolvedTheme = this.currentTheme;
    }

    // Update the document class for Tailwind dark mode
    if (this.currentResolvedTheme === 'dark') {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }

    // Notify listeners
    this.listeners.forEach(listener => listener(this.currentResolvedTheme));
  }

  async setTheme(newTheme: Theme) {
    this.currentTheme = newTheme;

    // Set Tauri theme (this affects the titlebar on some platforms)
    if (newTheme === 'system') {
      await tauriSetTheme(null);
    } else {
      await tauriSetTheme(newTheme);
    }

    this.updateResolvedTheme();
  }

  getTheme(): Theme {
    return this.currentTheme;
  }

  getResolvedTheme(): 'light' | 'dark' {
    return this.currentResolvedTheme;
  }

  subscribe(listener: (theme: 'light' | 'dark') => void) {
    this.listeners.push(listener);
    // Return unsubscribe function
    return () => {
      const index = this.listeners.indexOf(listener);
      if (index > -1) {
        this.listeners.splice(index, 1);
      }
    };
  }

  private listenForThemeChanges() {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');

    const handleChange = () => {
      if (this.currentTheme === 'system') {
        this.updateResolvedTheme();
      }
    };

    mediaQuery.addEventListener('change', handleChange);
  }
}

// Create singleton instance
export const themeManager = new ThemeManager();