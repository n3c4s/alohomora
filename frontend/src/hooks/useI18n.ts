import { useState, useEffect, useCallback } from 'react';
import { 
  t, 
  setLanguage, 
  getLanguage, 
  getSystemLanguage, 
  getAvailableLanguages,
  getCurrentLanguageName,
  getCurrentLanguageFlag,
  toggleLanguage,
  getCurrentMessages,
  type SupportedLanguage,
  type LanguageInfo
} from '../i18n';

// Hook personalizado para internacionalización
export function useI18n() {
  const [currentLanguage, setCurrentLanguageState] = useState<SupportedLanguage>(getLanguage() as SupportedLanguage);
  const [messages, setMessages] = useState(getCurrentMessages());

  // Función para cambiar el idioma
  const changeLanguage = useCallback((lang: SupportedLanguage) => {
    setLanguage(lang);
    setCurrentLanguageState(lang);
    setMessages(getCurrentMessages());
  }, []);

  // Función para alternar entre idiomas
  const switchLanguage = useCallback(() => {
    const newLang = currentLanguage === 'en' ? 'es' : 'en';
    changeLanguage(newLang);
  }, [currentLanguage, changeLanguage]);

  // Función para obtener traducciones
  const translate = useCallback((key: string, lang?: string) => {
    return t(key, lang);
  }, []);

  // Función para detectar si el idioma actual es español
  const isSpanish = currentLanguage === 'es';
  
  // Función para detectar si el idioma actual es inglés
  const isEnglish = currentLanguage === 'en';

  // Función para obtener el idioma preferido del usuario
  const getUserPreferredLanguage = useCallback((): SupportedLanguage => {
    // Primero verificar si hay un idioma guardado
    const savedLang = getLanguage();
    if (savedLang && (savedLang === 'en' || savedLang === 'es')) {
      return savedLang as SupportedLanguage;
    }
    
    // Si no hay idioma guardado, usar el del sistema
    const systemLang = getSystemLanguage();
    return systemLang as SupportedLanguage;
  }, []);

  // Función para obtener el idioma del sistema como string legible
  const getSystemLanguageName = useCallback((): string => {
    const lang = getSystemLanguage();
    const available = getAvailableLanguages();
    const system = available.find(l => l.code === lang);
    return system ? system.name : 'English';
  }, []);

  // Función para obtener la bandera del idioma del sistema
  const getSystemLanguageFlag = useCallback((): string => {
    const lang = getSystemLanguage();
    const available = getAvailableLanguages();
    const system = available.find(l => l.code === lang);
    return system ? system.flag : '🇺🇸';
  }, []);

  // Función para verificar si el idioma del sistema está soportado
  const isSystemLanguageSupported = getAvailableLanguages().some(lang => lang.code === getSystemLanguage());

  // Función para obtener el idioma de fallback
  const getFallbackLanguage = useCallback((): SupportedLanguage => 'en', []);

  // Función para reinicializar el idioma
  const reinitializeLanguage = useCallback(() => {
    const preferredLang = getUserPreferredLanguage();
    changeLanguage(preferredLang);
  }, [getUserPreferredLanguage, changeLanguage]);

  // Función para obtener información completa del idioma actual
  const getCurrentLanguageInfo = (): LanguageInfo => {
    const available = getAvailableLanguages();
    const current = available.find(l => l.code === currentLanguage);
    return current || { code: 'en', name: 'English', flag: '🇺🇸' };
  };

  // Función para obtener información del idioma del sistema
  const getSystemLanguageInfo = (): LanguageInfo => {
    const lang = getSystemLanguage();
    const available = getAvailableLanguages();
    const system = available.find(l => l.code === lang);
    return system || { code: 'en', name: 'English', flag: '🇺🇸' };
  };

  // Escuchar cambios de idioma
  useEffect(() => {
    const handleLanguageChange = (event: CustomEvent) => {
      const newLang = event.detail.language as SupportedLanguage;
      setCurrentLanguageState(newLang);
      setMessages(getCurrentMessages());
    };

    window.addEventListener('languageChanged', handleLanguageChange as EventListener);
    
    return () => {
      window.removeEventListener('languageChanged', handleLanguageChange as EventListener);
    };
  }, []);

  return {
    // Estado del idioma
    currentLanguage,
    messages,
    
    // Funciones de traducción
    t: translate,
    
    // Funciones de control del idioma
    changeLanguage,
    switchLanguage,
    reinitializeLanguage,
    
    // Funciones de detección
    isSpanish,
    isEnglish,
    isSystemLanguageSupported,
    
    // Funciones de información
    getUserPreferredLanguage,
    getSystemLanguageName,
    getSystemLanguageFlag,
    getFallbackLanguage,
    getCurrentLanguageInfo,
    getSystemLanguageInfo,
    
    // Datos estáticos
    availableLanguages: getAvailableLanguages(),
    systemLanguage: getSystemLanguage(),
    
    // Tipos
    SupportedLanguage,
    LanguageInfo
  };
}

// Hook simplificado para solo traducciones
export function useTranslation() {
  const { t, currentLanguage, isSpanish, isEnglish } = useI18n();
  
  return {
    t,
    currentLanguage,
    isSpanish,
    isEnglish
  };
}

// Hook para cambio de idioma
export function useLanguageSwitcher() {
  const { 
    changeLanguage, 
    switchLanguage, 
    currentLanguage, 
    availableLanguages,
    isSystemLanguageSupported,
    getUserPreferredLanguage
  } = useI18n();
  
  return {
    changeLanguage,
    switchLanguage,
    currentLanguage,
    availableLanguages,
    isSystemLanguageSupported,
    getUserPreferredLanguage
  };
}
