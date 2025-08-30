/**
 * Content Script para AlohoPass
 * Detecta formularios de login y signup en páginas web
 */

class AlohoPassContentScript {
    constructor() {
        this.forms = new Map();
        this.currentForm = null;
        this.isConnected = false;
        this.init();
    }

    init() {
        console.log('🔐 AlohoPass: Content script iniciado');
        this.detectForms();
        this.setupEventListeners();
        this.checkConnection();
    }

    /**
     * Detecta formularios de login y signup en la página
     */
    detectForms() {
        const forms = document.querySelectorAll('form');
        let formIndex = 0;

        forms.forEach((form, index) => {
            if (this.isLoginOrSignupForm(form)) {
                const formId = `alohopass-form-${formIndex++}`;
                this.forms.set(formId, {
                    element: form,
                    type: this.detectFormType(form),
                    fields: this.extractFormFields(form),
                    url: window.location.href,
                    domain: window.location.hostname
                });

                // Marcar el formulario para AlohoPass
                form.setAttribute('data-alohopass-form', formId);
                this.addFormIndicator(form, formId);
                
                console.log(`🔐 AlohoPass: Formulario detectado - ${formId}`, {
                    type: this.forms.get(formId).type,
                    fields: this.forms.get(formId).fields
                });
            }
        });
    }

    /**
     * Determina si un formulario es de login o signup
     */
    isLoginOrSignupForm(form) {
        const formText = form.textContent.toLowerCase();
        const formHTML = form.innerHTML.toLowerCase();
        
        // Palabras clave para login
        const loginKeywords = [
            'login', 'signin', 'sign in', 'log in', 'iniciar sesión',
            'acceder', 'entrar', 'usuario', 'contraseña', 'password'
        ];

        // Palabras clave para signup
        const signupKeywords = [
            'signup', 'sign up', 'register', 'registro', 'registrarse',
            'crear cuenta', 'nueva cuenta', 'suscribirse'
        ];

        const hasLoginKeywords = loginKeywords.some(keyword => 
            formText.includes(keyword) || formHTML.includes(keyword)
        );

        const hasSignupKeywords = signupKeywords.some(keyword => 
            formText.includes(keyword) || formHTML.includes(keyword)
        );

        // Verificar si tiene campos de usuario/contraseña
        const hasPasswordField = form.querySelector('input[type="password"]');
        const hasUsernameField = form.querySelector('input[name*="user"], input[name*="email"], input[name*="login"]');

        return (hasLoginKeywords || hasSignupKeywords) && (hasPasswordField || hasUsernameField);
    }

    /**
     * Detecta el tipo de formulario (login o signup)
     */
    detectFormType(form) {
        const formText = form.textContent.toLowerCase();
        const formHTML = form.innerHTML.toLowerCase();
        
        const signupKeywords = [
            'signup', 'sign up', 'register', 'registro', 'registrarse',
            'crear cuenta', 'nueva cuenta', 'suscribirse'
        ];

        const hasSignupKeywords = signupKeywords.some(keyword => 
            formText.includes(keyword) || formHTML.includes(keyword)
        );

        return hasSignupKeywords ? 'signup' : 'login';
    }

    /**
     * Extrae los campos relevantes del formulario
     */
    extractFormFields(form) {
        const fields = {
            username: null,
            password: null,
            email: null,
            submit: null
        };

        // Buscar campo de usuario
        fields.username = form.querySelector('input[name*="user"], input[name*="login"], input[name*="email"]') ||
                         form.querySelector('input[type="email"]') ||
                         form.querySelector('input[placeholder*="usuario"], input[placeholder*="email"]');

        // Buscar campo de contraseña
        fields.password = form.querySelector('input[type="password"]');

        // Buscar campo de email (para signup)
        fields.email = form.querySelector('input[type="email"]') ||
                      form.querySelector('input[name*="email"]');

        // Buscar botón de envío
        fields.submit = form.querySelector('input[type="submit"], button[type="submit"], button:not([type])');

        return fields;
    }

    /**
     * Agrega un indicador visual al formulario
     */
    addFormIndicator(form, formId) {
        const indicator = document.createElement('div');
        indicator.className = 'alohopass-indicator';
        indicator.innerHTML = `
            <div style="
                position: absolute;
                top: -25px;
                right: 0;
                background: #10b981;
                color: white;
                padding: 2px 8px;
                border-radius: 4px;
                font-size: 11px;
                font-family: Arial, sans-serif;
                z-index: 10000;
                pointer-events: none;
            ">
                🔐 AlohoPass
            </div>
        `;

        // Posicionar el formulario para el indicador
        form.style.position = 'relative';
        form.appendChild(indicator);
    }

    /**
     * Configura los event listeners para los formularios
     */
    setupEventListeners() {
        this.forms.forEach((formData, formId) => {
            const form = formData.element;
            
            // Event listener para cuando se hace focus en campos
            form.addEventListener('focusin', (e) => {
                if (e.target.matches('input')) {
                    this.currentForm = formId;
                    this.showAutofillSuggestions(e.target, formData);
                }
            });

            // Event listener para cuando se envía el formulario
            form.addEventListener('submit', (e) => {
                this.handleFormSubmit(e, formData);
            });
        });
    }

    /**
     * Muestra sugerencias de autofill
     */
    showAutofillSuggestions(input, formData) {
        if (!this.isConnected) {
            console.log('🔐 AlohoPass: No conectado a la aplicación');
            return;
        }

        // Enviar mensaje al background script para obtener contraseñas
        chrome.runtime.sendMessage({
            action: 'getPasswords',
            domain: formData.domain,
            formType: formData.type
        }, (response) => {
            if (response && response.success) {
                this.displayPasswordSuggestions(input, response.passwords, formData);
            }
        });
    }

    /**
     * Muestra las sugerencias de contraseñas
     */
    displayPasswordSuggestions(input, passwords, formData) {
        // Crear overlay de sugerencias
        const overlay = document.createElement('div');
        overlay.className = 'alohopass-suggestions';
        overlay.style.cssText = `
            position: absolute;
            top: ${input.offsetTop + input.offsetHeight}px;
            left: ${input.offsetLeft}px;
            width: ${input.offsetWidth}px;
            background: white;
            border: 1px solid #ccc;
            border-radius: 4px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            z-index: 10001;
            max-height: 200px;
            overflow-y: auto;
        `;

        if (passwords.length === 0) {
            overlay.innerHTML = `
                <div style="padding: 10px; color: #666; text-align: center;">
                    No hay contraseñas guardadas para este sitio
                </div>
            `;
        } else {
            passwords.forEach(password => {
                const item = document.createElement('div');
                item.className = 'alohopass-suggestion-item';
                item.style.cssText = `
                    padding: 8px 12px;
                    cursor: pointer;
                    border-bottom: 1px solid #eee;
                    display: flex;
                    align-items: center;
                    gap: 8px;
                `;
                
                item.innerHTML = `
                    <span style="font-weight: bold;">${password.username}</span>
                    <span style="color: #666;">${password.title || 'Sin título'}</span>
                `;
                
                item.addEventListener('click', () => {
                    this.autofillForm(formData, password);
                    overlay.remove();
                });
                
                overlay.appendChild(item);
            });
        }

        // Agregar botón para crear nueva entrada
        const createButton = document.createElement('div');
        createButton.style.cssText = `
            padding: 8px 12px;
            background: #10b981;
            color: white;
            text-align: center;
            cursor: pointer;
            border-radius: 0 0 4px 4px;
        `;
        createButton.textContent = '➕ Crear nueva entrada';
        createButton.addEventListener('click', () => {
            this.createNewPasswordEntry(formData);
            overlay.remove();
        });
        
        overlay.appendChild(createButton);

        // Agregar al DOM
        document.body.appendChild(overlay);

        // Remover al hacer click fuera
        document.addEventListener('click', function removeOverlay(e) {
            if (!overlay.contains(e.target) && e.target !== input) {
                overlay.remove();
                document.removeEventListener('click', removeOverlay);
            }
        });
    }

    /**
     * Autocompleta el formulario con la contraseña seleccionada
     */
    autofillForm(formData, password) {
        const { fields } = formData;
        
        if (fields.username && password.username) {
            fields.username.value = password.username;
            fields.username.dispatchEvent(new Event('input', { bubbles: true }));
        }
        
        if (fields.password && password.password) {
            fields.password.value = password.password;
            fields.password.dispatchEvent(new Event('input', { bubbles: true }));
        }
        
        if (fields.email && password.email) {
            fields.email.value = password.email;
            fields.email.dispatchEvent(new Event('input', { bubbles: true }));
        }

        console.log('🔐 AlohoPass: Formulario autocompletado');
    }

    /**
     * Crea una nueva entrada de contraseña
     */
    createNewPasswordEntry(formData) {
        const { fields, domain, url, type } = formData;
        
        const newEntry = {
            title: document.title || domain,
            username: fields.username ? fields.username.value : '',
            password: fields.password ? fields.password.value : '',
            email: fields.email ? fields.email.value : '',
            url: url,
            domain: domain,
            type: type
        };

        // Enviar al background script para guardar
        chrome.runtime.sendMessage({
            action: 'createPassword',
            entry: newEntry
        }, (response) => {
            if (response && response.success) {
                console.log('🔐 AlohoPass: Nueva contraseña creada');
            }
        });
    }

    /**
     * Maneja el envío del formulario
     */
    handleFormSubmit(event, formData) {
        if (formData.type === 'signup') {
            // Para signup, crear nueva entrada
            this.createNewPasswordEntry(formData);
        }
        
        console.log('🔐 AlohoPass: Formulario enviado', formData.type);
    }

    /**
     * Verifica la conexión con la aplicación Tauri
     */
    checkConnection() {
        chrome.runtime.sendMessage({
            action: 'checkConnection'
        }, (response) => {
            this.isConnected = response && response.connected;
            console.log('🔐 AlohoPass: Conexión con app Tauri:', this.isConnected);
        });
    }
}

// Inicializar el content script
new AlohoPassContentScript();
