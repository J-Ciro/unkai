// App.js - Frontend para YASB

const { invoke, listen } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;

let debugMode = false;

// Elementos DOM
const statusEl = document.getElementById('status');
const widgetsEl = document.getElementById('widgets');
const btnReload = document.getElementById('btn-reload');
const btnDebug = document.getElementById('btn-debug');

// ============================================================================
// INIT
// ============================================================================

async function init() {
    console.log('[INIT] Starting unkai frontend...');
    
    updateStatus('Initializing...');
    
    try {
        // Escuchar eventos de app ready
        listen('app:ready', (event) => {
            console.log('[APP] Backend ready:', event.payload);
            updateStatus('Connected ✓');
            loadWidgets();
        });

        // Escuchar actualización de widgets
        listen('widget:updated', (event) => {
            console.log('[EVENT] Widget updated:', event.payload);
            if (debugMode) {
                console.log('[DEBUG]', JSON.stringify(event.payload, null, 2));
            }
            renderWidget(event.payload);
        });

        // Escuchar cambios de monitores
        listen('monitors:changed', (event) => {
            console.log('[EVENT] Monitors changed:', event.payload);
        });

        // Escuchar errores
        listen('widget:error', (event) => {
            console.error('[ERROR] Widget error:', event.payload);
            showError(event.payload.widget_id, event.payload.error);
        });

        // Cargar config inicial
        await loadWidgets();
        
    } catch (err) {
        console.error('[ERROR] Init failed:', err);
        updateStatus('Error: ' + err.message);
    }
}

// ============================================================================
// COMMANDS
// ============================================================================

async function loadWidgets() {
    try {
        const response = await invoke('get_all_widgets');
        
        if (response.success && response.data) {
            widgetsEl.innerHTML = '';
            
            response.data.forEach(widget => {
                renderWidget(widget);
            });
            
            updateStatus(`Loaded ${response.data.length} widgets ✓`);
        }
    } catch (err) {
        console.error('[ERROR] Failed to load widgets:', err);
        updateStatus('Failed to load widgets');
    }
}

function renderWidget(widget) {
    // Encontrar o crear widget elemento
    let widgetEl = document.getElementById(`widget-${widget.id}`);
    
    if (!widgetEl) {
        widgetEl = document.createElement('div');
        widgetEl.id = `widget-${widget.id}`;
        widgetEl.className = 'widget';
        widgetsEl.appendChild(widgetEl);
    }

    // Renderizar contenido
    const data = widget.data || {};
    
    let html = `<div class="widget-title">${widget.id}</div>`;
    html += '<div class="widget-content">';
    
    // Mostrar datos disponibles
    if (data.cpu_percent !== undefined) {
        html += `
            <div class="metric">
                <div class="metric-label">CPU</div>
                <div class="metric-value">${data.cpu_percent.toFixed(1)}%</div>
            </div>
        `;
    }
    
    if (data.memory_percent !== undefined) {
        html += `
            <div class="metric">
                <div class="metric-label">Memory</div>
                <div class="metric-value">${data.memory_percent.toFixed(1)}%</div>
            </div>
        `;
    }
    
    if (data.memory_mb !== undefined) {
        html += `
            <div class="metric">
                <div class="metric-label">Used</div>
                <div class="metric-value">${Math.round(data.memory_mb)}MB</div>
            </div>
        `;
    }
    
    if (data.total_memory_mb !== undefined) {
        html += `
            <div class="metric">
                <div class="metric-label">Total</div>
                <div class="metric-value">${Math.round(data.total_memory_mb)}MB</div>
            </div>
        `;
    }
    
    html += '</div>';
    
    widgetEl.innerHTML = html;
}

async function reloadConfig() {
    try {
        updateStatus('Reloading config...');
        const response = await invoke('reload_config');
        
        if (response.success) {
            updateStatus('Config reloaded ✓');
            await loadWidgets();
        } else {
            updateStatus('Error: ' + response.error);
        }
    } catch (err) {
        console.error('[ERROR] Reload failed:', err);
        updateStatus('Error: ' + err.message);
    }
}

function toggleDebug() {
    debugMode = !debugMode;
    updateStatus(debugMode ? 'Debug ON' : 'Debug OFF');
    console.log('[DEBUG] Debug mode:', debugMode);
}

// ============================================================================
// UI HELPERS
// ============================================================================

function updateStatus(message) {
    const p = statusEl.querySelector('p');
    if (p) {
        p.textContent = message;
    }
}

function showError(widgetId, error) {
    const errorEl = document.createElement('div');
    errorEl.className = 'error';
    errorEl.innerHTML = `<strong>${widgetId}</strong>: ${error}`;
    statusEl.appendChild(errorEl);
    
    // Remover error después de 5s
    setTimeout(() => errorEl.remove(), 5000);
}

// ============================================================================
// EVENT LISTENERS
// ============================================================================

btnReload.addEventListener('click', reloadConfig);
btnDebug.addEventListener('click', toggleDebug);

// ============================================================================
// START
// ============================================================================

window.addEventListener('DOMContentLoaded', init);
