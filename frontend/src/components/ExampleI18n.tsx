import React from 'react';
import { useTranslation } from '../hooks/useI18n';
import { LanguageSelector } from './LanguageSelector';

export const ExampleI18n: React.FC = () => {
  const { t, currentLanguage, isSpanish, isEnglish } = useTranslation();

  return (
    <div className="p-6 max-w-4xl mx-auto">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          {t('welcome.title')}
        </h1>
        <LanguageSelector variant="dropdown" />
      </div>

      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6">
        <h2 className="text-xl font-semibold mb-4 text-gray-800 dark:text-gray-200">
          {t('welcome.subtitle')}
        </h2>
        
        <p className="text-gray-600 dark:text-gray-300 mb-6">
          {t('welcome.description')}
        </p>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
          <div className="bg-blue-50 dark:bg-blue-900/20 p-4 rounded-lg">
            <h3 className="font-medium text-blue-900 dark:text-blue-100 mb-2">
              {t('welcome.features.security')}
            </h3>
            <p className="text-blue-700 dark:text-blue-300 text-sm">
              {t('auth.masterPassword')}
            </p>
          </div>

          <div className="bg-green-50 dark:bg-green-900/20 p-4 rounded-lg">
            <h3 className="font-medium text-green-900 dark:text-green-100 mb-2">
              {t('welcome.features.crossPlatform')}
            </h3>
            <p className="text-green-700 dark:text-green-300 text-sm">
              {t('settings.general')}
            </p>
          </div>

          <div className="bg-purple-50 dark:bg-purple-900/20 p-4 rounded-lg">
            <h3 className="font-medium text-purple-900 dark:text-purple-100 mb-2">
              {t('welcome.features.easyToUse')}
            </h3>
            <p className="text-purple-700 dark:text-purple-300 text-sm">
              {t('dashboard.quickActions')}
            </p>
          </div>

          <div className="bg-orange-50 dark:bg-orange-900/20 p-4 rounded-lg">
            <h3 className="font-medium text-orange-900 dark:text-orange-100 mb-2">
              {t('welcome.features.openSource')}
            </h3>
            <p className="text-orange-700 dark:text-orange-300 text-sm">
              {t('settings.about')}
            </p>
          </div>
        </div>

        <div className="flex gap-4">
          <button className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-2 rounded-lg font-medium transition-colors">
            {t('welcome.getStarted')}
          </button>
          <button className="border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700 px-6 py-2 rounded-lg font-medium transition-colors">
            {t('welcome.learnMore')}
          </button>
        </div>

        <div className="mt-8 p-4 bg-gray-50 dark:bg-gray-700 rounded-lg">
          <h3 className="font-medium text-gray-800 dark:text-gray-200 mb-2">
            Información del Idioma
          </h3>
          <div className="text-sm text-gray-600 dark:text-gray-400 space-y-1">
            <p><strong>Idioma actual:</strong> {currentLanguage}</p>
            <p><strong>Es español:</strong> {isSpanish ? 'Sí' : 'No'}</p>
            <p><strong>Es inglés:</strong> {isEnglish ? 'Sí' : 'No'}</p>
            <p><strong>Traducción de ejemplo:</strong> {t('common.loading')}</p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ExampleI18n;
