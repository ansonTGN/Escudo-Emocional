// src/i18n.rs
use std::collections::HashMap;

pub fn get_translations(lang: &str) -> HashMap<&str, &str> {
    let mut map = HashMap::new();
    
    // TEXTOS COMUNES
    map.insert("author", "ANGEL A. URBINA © 2026");
    map.insert("version", "CORE v2.2");

    match lang {
        "cat" => {
            map.insert("title", "Guardià Relacional");
            map.insert("subtitle", "ESPAI DE SEGURETAT I VALIDACIÓ");
            map.insert("tab_system", "Mode Consultor (Tècnic)");
            map.insert("tab_motors", "Ajuda Personal (Estic confós/a)");
            map.insert("btn_text", "Escriure");
            map.insert("btn_file", "Pujar Arxiu");
            map.insert("btn_audio", "Gravar veu");
            map.insert("btn_analyze", "ANALITZAR");
            map.insert("btn_motors", "PROCESSAR SITUACIÓ");
            map.insert("rec_start", "🔴 Gravar");
            map.insert("rec_stop", "⏹ Aturar");
            map.insert("loading", "Validant la teva història...");
            map.insert("btn_help", "Com funciona?");
            map.insert("btn_load", "Carregar");
            map.insert("btn_exit", "Sortir");
            
            // REPORT UI
            map.insert("rep_title", "Informe de Claredat");
            map.insert("btn_new", "Tornar a començar");
            map.insert("btn_save", "Guardar còpia");
            map.insert("btn_read", "Llegir");
            map.insert("btn_pdf", "Baixar PDF");
            map.insert("btn_html", "Baixar HTML");
            map.insert("rep_confidential", "PRIVAT // NOMÉS PER A TU");
            map.insert("rep_footer", "Generat per Guardià Relacional v2.2. Eina de suport analític.");

            // FORMULARIO EMPÁTICO
            map.insert("lbl_target", "Amb qui tens el conflicte?");
            map.insert("lbl_relation", "Quina relació teniu?");
            
            map.insert("lbl_m_context", "1. L'Ambient (Què sents a l'aire?)");
            map.insert("ph_m_context", "No calen termes tècnics. Ex: 'Sento que camino sobre ous', 'Tothom calla quan ell/a entra', 'Si dic que no, em fan el buit'...");
            
            map.insert("lbl_m_obs", "2. Què ha passat darrerament?");
            map.insert("ph_m_obs", "Explica-ho com si parlessis amb un amic. 'Ahir em va cridar', 'Em sento ignorat/da des de fa dies', 'Em diu que estic boig/ja'...");
            
            map.insert("lbl_m_goal", "3. Què necessites ara mateix?");
            map.insert("ph_m_goal", "Ex: 'Saber si m'ho estic imaginant', 'Vull que pari', 'Vull marxar sense problemes', 'Només vull pau'...");
            
            // SENSORES DETALLADOS (CAT)
            map.insert("lbl_signals", "4. Panell de Control Somàtic (Neurocepció)");
            map.insert("scale_desc", "Puntua d'1 (Calma) a 5 (Intens). El cos detecta l'amenaça abans que la ment.");
            
            map.insert("sig_security", "Seguretat Física: Sento por o amenaça real al meu cos?");
            map.insert("sig_curiosity", "Estat d'Alerta (Hipervigilància): Estic escanejant l'entorn buscant perills?");
            map.insert("sig_status", "Dignitat (Estatus): Em fan sentir petit/a, humiliat/da o avergonyit/da?");
            map.insert("sig_belonging", "Pertinença: Em fan el buit o m'ignoren (Llei del gel)?");
            map.insert("sig_autonomy", "Autonomia: Sento que camino sobre ous? Tinc llibertat real?");
            map.insert("sig_mastery", "Competència (Indefensió): Em fan sentir inútil o que fallo sempre?");
            map.insert("sig_justice", "Justícia: Sento una ràbia interna per un tracte injust?");
            map.insert("sig_purpose", "Claredat (Gaslighting): Dubto de la meva memòria o seny?");
            map.insert("sig_control", "Coacció: Tinc por a les represàlies si parlo?");
            map.insert("sig_comfort", "Energia: Tinc dolors físics, insomni o esgotament sense causa?");
            
            // RELACIONES
            map.insert("rel_peer", "Amics / Grup Social");
            map.insert("rel_boss", "Jefe / Superior");
            map.insert("rel_team", "Companys de feina");
            map.insert("rel_client", "Parella / Sentimental");
            map.insert("rel_provider", "Grup Religiós / Comunitat");
            map.insert("rel_adversary", "Professor / Autoritat Acadèmica");
            map.insert("rel_regulator", "Família");
            map.insert("rel_mentor", "Mentor / Guia");
            map.insert("rel_other", "Altre");
            
            // MODO ANÁLISIS
            map.insert("lbl_situation", "Situació");
            map.insert("ph_situation", "Descripció...");
            map.insert("lbl_cv", "Perfil");
            map.insert("ph_cv", "Dades...");
            map.insert("lbl_focus", "Focus");
            map.insert("ph_focus", "Dubtes...");
        },
        "en" => {
            map.insert("title", "Relational Guardian");
            map.insert("subtitle", "SAFE SPACE & VALIDATION");
            map.insert("tab_system", "Consultant Mode (Technical)");
            map.insert("tab_motors", "Personal Help (I'm confused)");
            map.insert("btn_text", "Write");
            map.insert("btn_file", "Upload File");
            map.insert("btn_audio", "Record Voice");
            map.insert("btn_analyze", "ANALYZE");
            map.insert("btn_motors", "PROCESS SITUATION");
            map.insert("rec_start", "🔴 Record");
            map.insert("rec_stop", "⏹ Stop");
            map.insert("loading", "Validating your story...");
            map.insert("btn_help", "How does this work?");
            map.insert("btn_load", "Load");
            map.insert("btn_exit", "Exit");

            map.insert("rep_title", "Clarity Report");
            map.insert("btn_new", "Start Over");
            map.insert("btn_save", "Save Copy");
            map.insert("btn_read", "Read Aloud");
            map.insert("btn_pdf", "Download PDF");
            map.insert("btn_html", "Download HTML");
            map.insert("rep_confidential", "PRIVATE // FOR YOUR EYES ONLY");
            map.insert("rep_footer", "Generated by Relational Guardian v2.2. Analytical support tool.");

            map.insert("lbl_target", "Who is involved?");
            map.insert("lbl_relation", "Relationship type");
            
            map.insert("lbl_m_context", "1. The Atmosphere (What does it feel like?)");
            map.insert("ph_m_context", "No technical terms needed. Ex: 'I feel like walking on eggshells', 'Everyone goes silent when they enter', 'If I say no, they ignore me'...");
            
            map.insert("lbl_m_obs", "2. What happened recently?");
            map.insert("ph_m_obs", "Tell it like you're talking to a friend. 'Yesterday they yelled at me', 'I feel ignored for days', 'They say I'm crazy'...");
            
            map.insert("lbl_m_goal", "3. What do you need right now?");
            map.insert("ph_m_goal", "Ex: 'I want to know if I'm imagining it', 'I want it to stop', 'I just want peace', 'I need to leave safely'...");
            
            // SENSORS DETAILED (EN)
            map.insert("lbl_signals", "4. Somatic Control Panel (Neuroception)");
            map.insert("scale_desc", "Rate from 1 (Calm) to 5 (Intense). The body detects threats before the mind.");
            
            map.insert("sig_security", "Physical Safety: Do I feel fear or direct threat to my body?");
            map.insert("sig_curiosity", "Alertness (Hypervigilance): Am I constantly scanning for danger?");
            map.insert("sig_status", "Dignity (Status): Do I feel small, humiliated, or shamed?");
            map.insert("sig_belonging", "Belonging: Am I being shunned or ignored (Silent treatment)?");
            map.insert("sig_autonomy", "Autonomy: Am I walking on eggshells? Do I have free will?");
            map.insert("sig_mastery", "Competence (Helplessness): Do I feel useless or incapable?");
            map.insert("sig_justice", "Fairness: Do I feel internal rage due to injustice?");
            map.insert("sig_purpose", "Clarity (Gaslighting): Do I doubt my own memory or sanity?");
            map.insert("sig_control", "Coercion: Do I fear punishment if I speak up?");
            map.insert("sig_comfort", "Energy: Do I have unexplained pain, insomnia, or exhaustion?");
            
            map.insert("rel_peer", "Friends / Social Group");
            map.insert("rel_boss", "Boss / Manager");
            map.insert("rel_team", "Coworkers");
            map.insert("rel_client", "Partner / Romantic");
            map.insert("rel_provider", "Religious Group / Cult");
            map.insert("rel_adversary", "Teacher / Coach");
            map.insert("rel_regulator", "Family");
            map.insert("rel_mentor", "Mentor / Guide");
            map.insert("rel_other", "Other");

            map.insert("lbl_situation", "Situation");
            map.insert("ph_situation", "Description...");
            map.insert("lbl_cv", "Profile");
            map.insert("ph_cv", "Background...");
            map.insert("lbl_focus", "Focus");
            map.insert("ph_focus", "Specifics...");
        },
        _ => { // ES
            map.insert("title", "Guardián Relacional");
            map.insert("subtitle", "ESPACIO DE SEGURIDAD Y VALIDACIÓN");
            map.insert("tab_system", "Modo Consultor (Técnico)");
            map.insert("tab_motors", "Ayuda Personal (Estoy confundido/a)");
            map.insert("btn_text", "Escribir");
            map.insert("btn_file", "Subir Archivo");
            map.insert("btn_audio", "Grabar Voz");
            map.insert("btn_analyze", "ANALIZAR");
            map.insert("btn_motors", "PROCESAR SITUACIÓN");
            map.insert("rec_start", "🔴 Grabar");
            map.insert("rec_stop", "⏹ Detener");
            map.insert("loading", "Validando tu historia...");
            map.insert("btn_help", "¿Cómo funciona?");
            map.insert("btn_load", "Cargar");
            map.insert("btn_exit", "Salir");

            map.insert("rep_title", "Informe de Claridad");
            map.insert("btn_new", "Volver a empezar");
            map.insert("btn_save", "Guardar copia");
            map.insert("btn_read", "Leer en voz alta");
            map.insert("btn_pdf", "Descargar PDF");
            map.insert("btn_html", "Descargar HTML");
            map.insert("rep_confidential", "PRIVADO // SOLO PARA TUS OJOS");
            map.insert("rep_footer", "Generado por Guardián Relacional v2.2. Herramienta de apoyo analítico.");

            map.insert("lbl_target", "¿Con quién tienes el conflicto?");
            map.insert("lbl_relation", "¿Qué relación tenéis?");
            
            // REESCRITURA EMPÁTICA
            map.insert("lbl_m_context", "1. El Ambiente (¿Qué se siente en el aire?)");
            map.insert("ph_m_context", "No uses palabras técnicas. Ej: 'Siento que camino sobre huevos', 'Hay reglas que cambian cada día', 'Si no estoy de acuerdo, me ignoran'...");
            
            map.insert("lbl_m_obs", "2. ¿Qué ha pasado últimamente?");
            map.insert("ph_m_obs", "Cuéntalo como a un amigo/a. 'Ayer me gritó', 'Lleva días sin hablarme', 'Me dice que me invento las cosas'...");
            
            map.insert("lbl_m_goal", "3. ¿Qué necesitas ahora mismo?");
            map.insert("ph_m_goal", "Ej: 'Saber si me lo estoy imaginando', 'Quiero que pare el dolor', 'Quiero irme sin pelear', 'Necesito entender qué pasa'...");
            
            // SENSORES DETALLADOS (ES)
            map.insert("lbl_signals", "4. Panel de Control Somático (Neurocepción)");
            map.insert("scale_desc", "Puntúa del 1 (Calma) al 5 (Intenso). Tu cuerpo detecta la amenaza antes que tu mente.");
            
            map.insert("sig_security", "Seguridad Física: ¿Siento miedo real o amenaza a mi integridad?");
            map.insert("sig_curiosity", "Estado de Alerta (Hipervigilancia): ¿Estoy escaneando buscando peligro?");
            map.insert("sig_status", "Dignidad (Estatus): ¿Me hacen sentir pequeño/a, humillado/a o avergonzado/a?");
            map.insert("sig_belonging", "Pertenencia: ¿Me hacen el vacío o me ignoran (Ley del hielo)?");
            map.insert("sig_autonomy", "Autonomía: ¿Siento que camino sobre huevos? ¿Tengo libertad?");
            map.insert("sig_mastery", "Competencia (Indefensión): ¿Me hacen sentir inútil o que todo lo hago mal?");
            map.insert("sig_justice", "Justicia: ¿Siento una rabia interna o indignación profunda?");
            map.insert("sig_purpose", "Claridad (Gaslighting): ¿Dudo de mi propia memoria o cordura?");
            map.insert("sig_control", "Coacción: ¿Tengo miedo a represalias o castigos si hablo?");
            map.insert("sig_comfort", "Energía: ¿Tengo dolores físicos, insomnio o agotamiento extremo?");
            
            map.insert("rel_peer", "Amigos / Grupo Social");
            map.insert("rel_boss", "Jefe / Superior");
            map.insert("rel_team", "Compañeros de trabajo");
            map.insert("rel_client", "Pareja / Sentimental");
            map.insert("rel_provider", "Grupo Religioso / Sectario");
            map.insert("rel_adversary", "Profesor / Entrenador");
            map.insert("rel_regulator", "Familia");
            map.insert("rel_mentor", "Mentor / Guía");
            map.insert("rel_other", "Otro");

            map.insert("lbl_situation", "Situación");
            map.insert("ph_situation", "Descripción...");
            map.insert("lbl_cv", "Perfil");
            map.insert("ph_cv", "Antecedentes...");
            map.insert("lbl_focus", "Foco");
            map.insert("ph_focus", "¿Dudas concretas?");
        }
    };
    map
}