pub const LANGUAGES: [(&str, &str); 8] = [
    ("fr", "Français"),
    ("en", "English"),
    ("es", "Español"),
    ("de", "Deutsch"),
    ("it", "Italiano"),
    ("pt", "Português"),
    ("zh-CN", "简体中文"),
    ("ja", "日本語"),
];

pub fn t(lang: &str, key: &str) -> &'static str {
    let i = LANGUAGES
        .iter()
        .position(|(code, _)| *code == lang)
        .unwrap_or(0);
    let row: &[&str; 8] = match key {
        "devices" => &["Périphériques", "Devices", "Dispositivos", "Geräte", "Dispositivi", "Dispositivos", "设备", "デバイス"],
        "refresh" => &["Actualiser", "Refresh", "Actualizar", "Aktualisieren", "Aggiorna", "Atualizar", "刷新", "更新"],
        "profiles" => &["Profils", "Profiles", "Perfiles", "Profile", "Profili", "Perfis", "配置文件", "プロファイル"],
        "add" => &["Ajouter", "Add", "Añadir", "Hinzufügen", "Aggiungi", "Adicionar", "添加", "追加"],
        "delete" => &["Supprimer", "Delete", "Eliminar", "Löschen", "Elimina", "Excluir", "删除", "削除"],
        "save" => &["Enregistrer", "Save", "Guardar", "Speichern", "Salva", "Salvar", "保存", "保存"],
        "apply" => &["Appliquer au périphérique", "Apply to device", "Aplicar al dispositivo", "Auf Gerät anwenden", "Applica al dispositivo", "Aplicar ao dispositivo", "应用到设备", "デバイスに適用"],
        "name" => &["Nom", "Name", "Nombre", "Name", "Nome", "Nome", "名称", "名前"],
        "rate" => &["Intervalle (ms)", "Report interval (ms)", "Intervalo (ms)", "Abfrageintervall (ms)", "Intervallo (ms)", "Intervalo (ms)", "报告间隔 (毫秒)", "レポート間隔 (ms)"],
        "color" => &["Couleur RVB", "RGB color", "Color RGB", "RGB-Farbe", "Colore RGB", "Cor RGB", "RGB 颜色", "RGB 色"],
        "notes" => &["Notes", "Notes", "Notas", "Notizen", "Note", "Notas", "备注", "メモ"],
        "status" => &["État", "Status", "Estado", "Status", "Stato", "Estado", "状态", "状態"],
        "no_device" => &["Aucun périphérique Logitech détecté", "No Logitech device detected", "No se detectó ningún dispositivo Logitech", "Kein Logitech-Gerät erkannt", "Nessun dispositivo Logitech rilevato", "Nenhum dispositivo Logitech detectado", "未检测到 Logitech 设备", "Logitech デバイスが見つかりません"],
        "linux_note" => &["Sous Linux, ratbagd permet d'appliquer DPI et intervalle aux souris compatibles. Les autres fonctions matérielles sont en développement.", "On Linux, ratbagd can apply DPI and report interval to compatible mice. Other hardware features are in development.", "En Linux, ratbagd aplica DPI e intervalo a ratones compatibles. Otras funciones están en desarrollo.", "Unter Linux kann ratbagd DPI und Intervall für kompatible Mäuse setzen. Weitere Hardwarefunktionen folgen.", "Su Linux, ratbagd applica DPI e intervallo ai mouse compatibili. Altre funzioni sono in sviluppo.", "No Linux, ratbagd aplica DPI e intervalo a mouses compatíveis. Outras funções estão em desenvolvimento.", "Linux 可通过 ratbagd 设置兼容鼠标的 DPI 和报告间隔。其他硬件功能仍在开发中。", "Linux では ratbagd で対応マウスの DPI とレポート間隔を設定できます。その他の機能は開発中です。"],
        "desktop_note" => &["La détection USB fonctionne. La configuration matérielle sur ce système est en développement.", "USB discovery works. Hardware configuration on this system is in development.", "La detección USB funciona. La configuración de hardware está en desarrollo.", "USB-Erkennung funktioniert. Hardwarekonfiguration ist in Entwicklung.", "Il rilevamento USB funziona. La configurazione hardware è in sviluppo.", "A detecção USB funciona. A configuração de hardware está em desenvolvimento.", "USB 检测可用。此系统上的硬件配置仍在开发中。", "USB 検出は利用できます。このシステムでのハードウェア設定は開発中です。"],
        "windows_note" => &["Sous Windows, le DPI et l'intervalle sont expérimentaux sur les souris filaires HID++ compatibles. Les autres appareils sont détectés sans contrôle matériel.", "On Windows, DPI and report interval are experimental for compatible corded HID++ mice. Other devices are listed without hardware control.", "En Windows, DPI e intervalo son experimentales en ratones HID++ con cable compatibles. Los demás dispositivos solo se detectan.", "Unter Windows sind DPI und Abfrageintervall für kompatible kabelgebundene HID++ Mäuse experimentell. Andere Geräte werden nur erkannt.", "Su Windows, DPI e intervallo sono sperimentali per mouse HID++ cablati compatibili. Gli altri dispositivi vengono solo rilevati.", "No Windows, DPI e intervalo são experimentais em mouses HID++ com fio compatíveis. Os outros dispositivos são apenas detectados.", "Windows 上，兼容的有线 HID++ 鼠标可实验性设置 DPI 和报告间隔。其他设备仅供检测。", "Windows では対応する有線 HID++ マウスの DPI とレポート間隔を試験的に設定できます。他の機器は検出のみです。"],
        "catalog" => &["Catalogue Linux", "Linux catalog", "Catálogo Linux", "Linux-Katalog", "Catalogo Linux", "Catálogo Linux", "Linux 目录", "Linux カタログ"],
        "catalog_note" => &["Identifiants référencés par libratbag ; la liste ne garantit pas la prise en charge par LJ Hub.", "IDs listed by libratbag; listing does not imply LJ Hub hardware support.", "IDs de libratbag; aparecer aquí no garantiza compatibilidad con LJ Hub.", "IDs aus libratbag; ein Eintrag garantiert keine LJ Hub-Unterstützung.", "ID di libratbag; la presenza non garantisce il supporto LJ Hub.", "IDs do libratbag; a listagem não garante suporte no LJ Hub.", "libratbag 收录的 ID；列出不代表 LJ Hub 支持硬件控制。", "libratbag の ID。掲載は LJ Hub のハードウェア対応を保証しません。"],
        "apply_dpi" => &["Appliquer le DPI", "Apply DPI", "Aplicar DPI", "DPI anwenden", "Applica DPI", "Aplicar DPI", "应用 DPI", "DPI を適用"],
        "apply_rate" => &["Appliquer la fréquence", "Apply report rate", "Aplicar frecuencia", "Abfragerate anwenden", "Applica frequenza", "Aplicar frequência", "应用报告速率", "レポートレートを適用"],
        "apply_rgb" => &["Appliquer le RVB", "Apply RGB", "Aplicar RGB", "RGB anwenden", "Applica RGB", "Aplicar RGB", "应用 RGB", "RGB を適用"],
        "rgb_control" => &["Autoriser LJ Hub à contrôler le RVB", "Allow LJ Hub to control RGB", "Permitir que LJ Hub controle RGB", "LJ Hub darf RGB steuern", "Consenti a LJ Hub di controllare RGB", "Permitir que LJ Hub controle RGB", "允许 LJ Hub 控制 RGB", "LJ Hub に RGB 制御を許可"],
        "rgb_note" => &["Désactivé par défaut pour laisser l'éclairage à rJb. Aucune commande RVB n'est envoyée tant que cette option reste désactivée.", "Off by default so rJb can control lighting. No RGB command is sent while off.", "Desactivado por defecto para dejar la iluminación a rJb. No se envían comandos RGB mientras esté desactivado.", "Standardmäßig aus, damit rJb die Beleuchtung steuert. Keine RGB-Befehle im ausgeschalteten Zustand.", "Disattivato per impostazione predefinita per lasciare l'illuminazione a rJb. Nessun comando RGB viene inviato.", "Desativado por padrão para deixar a iluminação com rJb. Nenhum comando RGB é enviado.", "默认关闭，让 rJb 控制灯光。关闭时不会发送 RGB 命令。", "既定でオフ。照明は rJb に任せ、オフの間は RGB コマンドを送信しません。"],
        "system_inventory" => &["Inventaire système", "System inventory", "Inventario del sistema", "Systeminventar", "Inventario di sistema", "Inventário do sistema", "系统设备清单", "システムの機器一覧"],
        "inventory_note" => &["USB et PnP Logitech, y compris caméras et volants ; détection seulement, sans contrôle matériel.", "Logitech USB and PnP, including cameras and wheels; detection only, no hardware control.", "USB y PnP Logitech, incluidas cámaras y volantes; solo detección, sin control.", "Logitech USB und PnP, einschließlich Kameras und Lenkrädern; nur Erkennung.", "USB e PnP Logitech, incluse webcam e volanti; solo rilevamento.", "USB e PnP Logitech, incluindo câmeras e volantes; apenas detecção.", "Logitech USB 和 PnP，包括摄像头和方向盘；仅检测，不控制硬件。", "カメラやホイールを含む Logitech USB/PnP。検出のみで制御しません。"],
        "scanning" => &["Recherche en cours…", "Scanning…", "Buscando…", "Suche läuft…", "Ricerca in corso…", "A procurar…", "正在扫描…", "検索中…"],
        _ => &["", "", "", "", "", "", "", ""],
    };
    row[i]
}
