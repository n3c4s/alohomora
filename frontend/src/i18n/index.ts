import en from '../locales/en.json';
import es from '../locales/es.json';

// Tipos para TypeScript
export type SupportedLanguage = 'en' | 'es';
export type LanguageInfo = {
  code: string;
  name: string;
  flag: string;
};

// Función para detectar el idioma del sistema
function detectSystemLanguage(): string {
  // Obtener el idioma del sistema
  const systemLang = navigator.language || navigator.languages?.[0] || 'en';
  
  // Mapear códigos de idioma a nuestros idiomas soportados
  const langMap: { [key: string]: string } = {
    'en': 'en',
    'en-US': 'en',
    'en-GB': 'en',
    'en-CA': 'en',
    'en-AU': 'en',
    'es': 'es',
    'es-ES': 'es',
    'es-MX': 'es',
    'es-AR': 'es',
    'es-CO': 'es',
    'es-PE': 'es',
    'es-VE': 'es',
    'es-CL': 'es',
    'es-EC': 'es',
    'es-GT': 'es',
    'es-CR': 'es',
    'es-CU': 'es',
    'es-BO': 'es',
    'es-DO': 'es',
    'es-HN': 'es',
    'es-PY': 'es',
    'es-SV': 'es',
    'es-NI': 'es',
    'es-PA': 'es',
    'es-UY': 'es',
    'es-GQ': 'es',
    'es-419': 'es', // Latinoamérica
  };
  
  // Buscar coincidencia exacta
  if (langMap[systemLang]) {
    return langMap[systemLang];
  }
  
  // Buscar coincidencia parcial (solo el código principal)
  const mainLang = systemLang.split('-')[0];
  if (langMap[mainLang]) {
    return langMap[mainLang];
  }
  
  // Por defecto, inglés
  return 'en';
}

// Función para obtener el idioma guardado en localStorage
function getSavedLanguage(): string | null {
  try {
    return localStorage.getItem('alohopass-language');
  } catch {
    return null;
  }
}

// Función para guardar el idioma en localStorage
function saveLanguage(lang: string): void {
  try {
    localStorage.setItem('alohopass-language', lang);
  } catch {
    // Ignorar errores de localStorage
  }
}

// Función para obtener el idioma actual
function getCurrentLanguage(): string {
  // Primero, verificar si hay un idioma guardado
  const savedLang = getSavedLanguage();
  if (savedLang && (savedLang === 'en' || savedLang === 'es')) {
    return savedLang;
  }
  
  // Si no hay idioma guardado, detectar del sistema
  return detectSystemLanguage();
}

// Función para cambiar el idioma
export function setLanguage(lang: SupportedLanguage): void {
  if (lang === 'en' || lang === 'es') {
    saveLanguage(lang);
    
    // Actualizar el atributo lang del HTML
    document.documentElement.lang = lang;
    
    // Disparar evento personalizado para notificar el cambio
    window.dispatchEvent(new CustomEvent('languageChanged', { detail: { language: lang } }));
  }
}

// Función para obtener el idioma actual
export function getLanguage(): string {
  return getSavedLanguage() || detectSystemLanguage();
}

// Función para obtener el idioma del sistema
export function getSystemLanguage(): string {
  return detectSystemLanguage();
}

// Función para obtener idiomas disponibles
export function getAvailableLanguages(): Array<LanguageInfo> {
  return [
    { code: 'en', name: 'English', flag: '🇺🇸' },
    { code: 'es', name: 'Español', flag: '🇪🇸' }
  ];
}

// Función para obtener el nombre del idioma actual
export function getCurrentLanguageName(): string {
  const lang = getLanguage();
  const available = getAvailableLanguages();
  const current = available.find(l => l.code === lang);
  return current ? current.name : 'English';
}

// Función para obtener la bandera del idioma actual
export function getCurrentLanguageFlag(): string {
  const lang = getLanguage();
  const available = getAvailableLanguages();
  const current = available.find(l => l.code === lang);
  return current ? current.flag : '🇺🇸';
}

// Función para alternar entre idiomas
export function toggleLanguage(): void {
  const currentLang = getLanguage();
  const newLang = currentLang === 'en' ? 'es' : 'en';
  setLanguage(newLang);
}

// Función para reinicializar el idioma (útil para cambios dinámicos)
export function reinitializeLanguage(): void {
  const currentLang = getCurrentLanguage();
  setLanguage(currentLang as SupportedLanguage);
}

// Función para obtener traducciones
export function t(key: string, lang?: string): string {
  const currentLang = lang || getLanguage();
  const messages = currentLang === 'es' ? es : en;
  
  // Dividir la clave por puntos para navegar por el objeto
  const keys = key.split('.');
  let value: any = messages;
  
  for (const k of keys) {
    if (value && typeof value === 'object' && k in value) {
      value = value[k];
    } else {
      // Si no se encuentra la traducción, devolver la clave
      return key;
    }
  }
  
  return typeof value === 'string' ? value : key;
}

// Función para obtener todas las traducciones de un idioma
export function getMessages(lang: SupportedLanguage) {
  return lang === 'es' ? es : en;
}

// Función para obtener las traducciones del idioma actual
export function getCurrentMessages() {
  const lang = getLanguage() as SupportedLanguage;
  return getMessages(lang);
}

// Configurar el idioma inicial
const initialLang = getCurrentLanguage();
setLanguage(initialLang as SupportedLanguage);
