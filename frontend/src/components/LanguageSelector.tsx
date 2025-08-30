import React, { useState } from 'react';
import { useLanguageSwitcher } from '../hooks/useI18n';
import { ChevronDown, Globe } from 'lucide-react';

interface LanguageSelectorProps {
  className?: string;
  variant?: 'button' | 'dropdown' | 'minimal';
}

export const LanguageSelector: React.FC<LanguageSelectorProps> = ({ 
  className = '', 
  variant = 'dropdown' 
}) => {
  const { 
    currentLanguage, 
    availableLanguages, 
    changeLanguage,
    isSystemLanguageSupported 
  } = useLanguageSwitcher();

  const [isDropdownOpen, setIsDropdownOpen] = useState(false);

  const currentLang = availableLanguages.find(lang => lang.code === currentLanguage);

  const handleLanguageChange = (lang: string) => {
    changeLanguage(lang as 'en' | 'es');
    setIsDropdownOpen(false);
  };

  if (variant === 'minimal') {
    return (
      <button
        onClick={() => changeLanguage(currentLanguage === 'en' ? 'es' : 'en')}
        className={`flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors ${className}`}
        title={`Switch to ${currentLanguage === 'en' ? 'Español' : 'English'}`}
      >
        <span className="text-lg">{currentLanguage === 'en' ? '🇪🇸' : '🇺🇸'}</span>
        <span className="text-sm font-medium">
          {currentLanguage === 'en' ? 'ES' : 'EN'}
        </span>
      </button>
    );
  }

  if (variant === 'button') {
    return (
      <button
        onClick={() => changeLanguage(currentLanguage === 'en' ? 'es' : 'en')}
        className={`flex items-center gap-2 px-4 py-2 rounded-lg border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors ${className}`}
      >
        <Globe className="w-4 h-4" />
        <span className="text-lg">{currentLang?.flag}</span>
        <span className="font-medium">{currentLang?.name}</span>
        <ChevronDown className="w-4 h-4" />
      </button>
    );
  }

  // Variant dropdown (default)
  return (
    <div className={`relative ${className}`}>
      <button
        className="flex items-center gap-2 px-4 py-2 rounded-lg border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
        onClick={() => setIsDropdownOpen(!isDropdownOpen)}
      >
        <Globe className="w-4 h-4" />
        <span className="text-lg">{currentLang?.flag}</span>
        <span className="font-medium">{currentLang?.name}</span>
        <ChevronDown className={`w-4 h-4 transition-transform ${isDropdownOpen ? 'rotate-180' : ''}`} />
      </button>

      {isDropdownOpen && (
        <div className="absolute right-0 mt-2 w-48 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg shadow-lg z-50">
          <div className="py-2">
            {availableLanguages.map((lang) => (
              <button
                key={lang.code}
                onClick={() => handleLanguageChange(lang.code)}
                className={`w-full flex items-center gap-3 px-4 py-2 text-left hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors ${
                  currentLanguage === lang.code 
                    ? 'bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400' 
                    : 'text-gray-700 dark:text-gray-300'
                }`}
              >
                <span className="text-lg">{lang.flag}</span>
                <span className="font-medium">{lang.name}</span>
                {currentLanguage === lang.code && (
                  <div className="ml-auto w-2 h-2 bg-blue-500 rounded-full"></div>
                )}
              </button>
            ))}
          </div>
          
          {!isSystemLanguageSupported && (
            <div className="px-4 py-2 text-xs text-gray-500 dark:text-gray-400 border-t border-gray-200 dark:border-gray-700">
              System language not supported
            </div>
          )}
        </div>
      )}

      {/* Overlay para cerrar el dropdown al hacer clic fuera */}
      {isDropdownOpen && (
        <div 
          className="fixed inset-0 z-40" 
          onClick={() => setIsDropdownOpen(false)}
        />
      )}
    </div>
  );
};

export default LanguageSelector;
