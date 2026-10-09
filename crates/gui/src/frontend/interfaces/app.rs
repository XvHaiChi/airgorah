use crate::backend;

use gtk4::prelude::*;
use gtk4::*;

fn build_about_button() -> Button {
    Button::builder()
        .icon_name("help-about-symbolic")
        .tooltip_text("关于")
        .build()
}

fn build_update_button() -> Button {
    Button::builder()
        .icon_name("folder-download-symbolic")
        .tooltip_text("有可用更新")
        .build()
}

fn build_decrypt_button() -> Button {
    Button::builder()
        .icon_name("utilities-terminal-symbolic")
        .tooltip_text("WPA 解密")
        .build()
}

fn build_settings_button() -> Button {
    Button::builder()
        .icon_name("emblem-system-symbolic")
        .tooltip_text("设置")
        .build()
}

fn build_pmkid_button() -> Button {
    Button::builder()
        .icon_name("network-wireless-hotspot-symbolic")
        .tooltip_text("向所选接入点索取 PMKID（无需客户端）")
        .sensitive(false)
        .build()
}

fn build_scan_button() -> Button {
    Button::builder()
        .icon_name("media-playback-start-symbolic")
        .tooltip_text("开始 / 暂停扫描")
        .build()
}

fn build_restart_button() -> Button {
    Button::builder()
        .icon_name("view-refresh-symbolic")
        .tooltip_text("清空结果并重新扫描")
        .sensitive(false)
        .build()
}

fn build_export_button() -> Button {
    Button::builder()
        .icon_name("media-floppy-symbolic")
        .tooltip_text("将捕获的数据包保存为 .cap 文件")
        .sensitive(false)
        .build()
}

fn build_report_button() -> Button {
    Button::builder()
        .icon_name("edit-paste-symbolic")
        .tooltip_text("将捕获的数据保存为 .json 文件")
        .sensitive(false)
        .build()
}

fn build_hopping_button() -> Button {
    Button::builder()
        .icon_name("edit-select-all-symbolic")
        .tooltip_text("在所选频段的所有信道上跳频")
        .sensitive(false)
        .build()
}

fn build_focus_button() -> Button {
    Button::builder()
        .icon_name("edit-select-symbolic")
        .tooltip_text("锁定到所选接入点的信道")
        .sensitive(false)
        .build()
}

fn build_add_button() -> Button {
    Button::builder()
        .icon_name("list-add-symbolic")
        .tooltip_text("将所选接入点的信道加入跳频列表")
        .sensitive(false)
        .build()
}

fn build_previous_but() -> Button {
    Button::builder()
        .icon_name("go-up-symbolic")
        .tooltip_text("上一个接入点")
        .sensitive(false)
        .build()
}

fn build_next_but() -> Button {
    Button::builder()
        .icon_name("go-down-symbolic")
        .tooltip_text("下一个接入点")
        .sensitive(false)
        .build()
}

fn build_top_but() -> Button {
    Button::builder()
        .icon_name("go-top-symbolic")
        .tooltip_text("第一个接入点")
        .sensitive(false)
        .build()
}

fn build_bottom_but() -> Button {
    Button::builder()
        .icon_name("go-bottom-symbolic")
        .tooltip_text("最后一个接入点")
        .sensitive(false)
        .build()
}

fn build_deauth_button() -> Button {
    Button::builder()
        .icon_name("network-wireless-offline-symbolic")
        .tooltip_text("对所选接入点发起（或停止）去认证攻击")
        .sensitive(false)
        .build()
}

fn build_capture_button() -> Button {
    Button::builder()
        .icon_name("dialog-password-symbolic")
        .tooltip_text("解密在所选接入点上捕获的握手包或 PMKID")
        .sensitive(false)
        .build()
}

fn build_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("")
        .default_width(1280)
        .default_height(620)
        .build();

    window.set_size_request(500, 500);
    window.connect_close_request(|_| {
        backend::app_cleanup();
        glib::Propagation::Proceed
    });

    window
}

fn build_aps_model() -> ListStore {
    ListStore::new(&[
        glib::Type::STRING, // ESSID
        glib::Type::STRING, // BSSID
        glib::Type::STRING, // Band
        glib::Type::I32,    // Channel
        glib::Type::I32,    // Power
        glib::Type::STRING, // Encryption
        glib::Type::I32,    // Clients
        glib::Type::STRING, // First time seen
        glib::Type::STRING, // Last time seen
        glib::Type::BOOL,   // Handshake
        glib::Type::BOOL,   // PMKID
        glib::Type::STRING, // <color>
    ])
}

fn build_aps_view() -> TreeView {
    let view = TreeView::builder().vexpand(true).hexpand(true).build();
    let columns = [
        ("ESSID", 154),
        ("BSSID", 138),
        ("频段", 64),
        ("信道", 86),
        ("信号强度", 72),
        ("加密方式", 106),
        ("客户端", 80),
        ("首次发现", 150),
        ("最近发现", 150),
        ("握手包", 106),
        ("PMKID", 80),
    ];

    // Index of the hidden per-row background-color column (last model column).
    const COLOR_COL: i32 = 11;

    for (pos, (column_name, column_size)) in columns.into_iter().enumerate() {
        let column = TreeViewColumn::builder()
            .title(column_name)
            .resizable(true)
            .fixed_width(column_size)
            .min_width(column_size)
            .sort_indicator(true)
            .sort_column_id(pos as i32)
            .expand(false)
            .build();

        if pos == 0 {
            let icon_renderer = CellRendererPixbuf::new();
            icon_renderer.set_property("icon-name", "network-wireless");

            column.pack_start(&icon_renderer, false);
            column.add_attribute(&icon_renderer, "cell-background", COLOR_COL);
            column.set_expand(true);
        }

        // The Handshake (9) and PMKID (10) columns render as read-only toggles.
        if pos == 9 || pos == 10 {
            let toggle = CellRendererToggle::new();
            toggle.set_sensitive(false);
            column.pack_start(&toggle, false);
            column.add_attribute(&toggle, "active", pos as i32);
            column.add_attribute(&toggle, "cell-background", COLOR_COL);
        } else {
            let text_renderer = CellRendererText::new();
            column.pack_start(&text_renderer, false);
            column.add_attribute(&text_renderer, "text", pos as i32);
            column.add_attribute(&text_renderer, "background", COLOR_COL);
        }

        view.append_column(&column);
    }

    view
}

fn build_aps_scroll() -> ScrolledWindow {
    let aps_scroll = ScrolledWindow::new();
    aps_scroll.set_policy(PolicyType::Never, PolicyType::Automatic);
    aps_scroll.set_height_request(140);

    aps_scroll
}

fn build_aps_menu() -> PopoverMenu {
    let copy_bssid_item = gio::MenuItem::new(Some("复制 BSSID"), Some("app.copy_bssid"));
    let copy_essid_item = gio::MenuItem::new(Some("复制 ESSID"), Some("app.copy_essid"));
    let copy_channel_item = gio::MenuItem::new(Some("复制信道"), Some("app.copy_channel"));

    let submenu = gio::Menu::new();

    submenu.append_item(&copy_bssid_item);
    submenu.append_item(&copy_essid_item);
    submenu.append_item(&copy_channel_item);

    PopoverMenu::from_model(Some(&submenu))
}

fn build_cli_model() -> ListStore {
    ListStore::new(&[
        glib::Type::STRING, // Station MAC
        glib::Type::I32,    // Packets
        glib::Type::I32,    // Power
        glib::Type::STRING, // First time seen
        glib::Type::STRING, // Last time seen
        glib::Type::STRING, // Vendor
        glib::Type::STRING, // Probes
        glib::Type::STRING, // <color>
    ])
}

fn build_cli_view() -> TreeView {
    let view = TreeView::builder().vexpand(true).hexpand(true).build();
    let columns = [
        ("客户端 MAC", 200),
        ("数据包", 110),
        ("信号强度", 100),
        ("首次发现", 160),
        ("最近发现", 160),
        ("厂商", 200),
        ("探测请求", 300),
    ];

    for (pos, (column_name, column_size)) in columns.into_iter().enumerate() {
        let column = TreeViewColumn::builder()
            .title(column_name)
            .resizable(true)
            .fixed_width(column_size)
            .min_width(column_size)
            .sort_indicator(true)
            .sort_column_id(pos as i32)
            .expand(false)
            .build();

        if pos == 0 {
            let icon_renderer = CellRendererPixbuf::new();
            icon_renderer.set_property("icon-name", "computer");
            column.pack_start(&icon_renderer, false);
            column.add_attribute(&icon_renderer, "cell-background", 7);
        } else if pos == 5 || pos == 6 {
            column.set_expand(true);
        }

        let text_renderer = CellRendererText::new();
        column.pack_start(&text_renderer, true);
        column.add_attribute(&text_renderer, "text", pos as i32);
        column.add_attribute(&text_renderer, "background", 7);

        view.append_column(&column);
    }

    view
}

fn build_cli_scroll() -> ScrolledWindow {
    let aps_scroll = ScrolledWindow::new();
    aps_scroll.set_policy(PolicyType::Never, PolicyType::Automatic);

    aps_scroll
}

fn build_cli_menu() -> PopoverMenu {
    let copy_mac_item = gio::MenuItem::new(Some("复制 MAC"), Some("app.copy_mac"));
    let copy_vendor_item = gio::MenuItem::new(Some("复制厂商"), Some("app.copy_vendor"));
    let copy_probes_item = gio::MenuItem::new(Some("复制探测请求"), Some("app.copy_probes"));

    let submenu = gio::Menu::new();

    submenu.append_item(&copy_mac_item);
    submenu.append_item(&copy_vendor_item);
    submenu.append_item(&copy_probes_item);

    PopoverMenu::from_model(Some(&submenu))
}

pub struct AppGui {
    // Header bar
    pub about_button: Button,
    pub update_button: Button,
    pub decrypt_button: Button,
    pub settings_button: Button,
    // Main window
    pub window: ApplicationWindow,
    pub aps_model: ListStore,
    pub aps_view: TreeView,
    pub aps_scroll: ScrolledWindow,
    pub aps_menu: PopoverMenu,
    pub cli_model: ListStore,
    pub cli_view: TreeView,
    pub cli_scroll: ScrolledWindow,
    pub cli_menu: PopoverMenu,
    pub ghz_2_4_but: CheckButton,
    pub ghz_5_but: CheckButton,
    pub channel_filter_entry: Entry,
    pub scan_but: Button,
    pub restart_but: Button,
    pub export_but: Button,
    pub report_but: Button,
    pub previous_but: Button,
    pub next_but: Button,
    pub top_but: Button,
    pub bottom_but: Button,
    pub hopping_but: Button,
    pub focus_but: Button,
    pub add_but: Button,
    pub deauth_but: Button,
    pub pmkid_but: Button,
    pub capture_but: Button,
    pub client_status_bar: Statusbar,
    pub channel_status_bar: Statusbar,
    pub iface_status_bar: Statusbar,
}

impl AppGui {
    pub fn new(app: &Application) -> Self {
        let window = build_window(app);
        let header_bar = HeaderBar::new();

        window.set_titlebar(Some(&header_bar));

        let scan_but = build_scan_button();
        let restart_but = build_restart_button();
        let export_but = build_export_button();
        let report_but = build_report_button();

        let hopping_but = build_hopping_button();
        let focus_but = build_focus_button();
        let add_but = build_add_button();

        let previous_but = build_previous_but();
        let next_but = build_next_but();
        let top_but = build_top_but();
        let bottom_but = build_bottom_but();

        let deauth_but = build_deauth_button();
        let pmkid_but = build_pmkid_button();
        let capture_but = build_capture_button();

        let about_button = build_about_button();
        let decrypt_button = build_decrypt_button();
        let settings_button = build_settings_button();
        let update_button = build_update_button();

        update_button.hide();

        // Scan filters

        let ghz_2_4_but = CheckButton::builder()
            .active(true)
            .sensitive(false)
            .label("2.4 GHz")
            .build();
        let ghz_5_but = CheckButton::builder()
            .active(false)
            .sensitive(false)
            .label("5 GHz")
            .build();

        // Channel

        let channel_filter_entry = Entry::builder()
            .placeholder_text("信道（例：1,6,11）")
            .hexpand(true)
            .sensitive(false)
            .build();

        let css_provider = CssProvider::new();
        css_provider.load_from_data(
            std::str::from_utf8(
                b"
                    .error {
                        color: red;
                        border-color: red;
                    }
                ",
            )
            .unwrap(),
        );

        let style_context = channel_filter_entry.style_context();
        style_context.add_provider(&css_provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

        // Header Bar

        header_bar.pack_start(&scan_but);
        header_bar.pack_start(&restart_but);
        header_bar.pack_start(&export_but);
        header_bar.pack_start(&report_but);

        header_bar.pack_start(&Separator::new(Orientation::Vertical));

        header_bar.pack_start(&previous_but);
        header_bar.pack_start(&next_but);
        header_bar.pack_start(&top_but);
        header_bar.pack_start(&bottom_but);

        header_bar.pack_start(&Separator::new(Orientation::Vertical));

        header_bar.pack_start(&deauth_but);
        header_bar.pack_start(&pmkid_but);
        header_bar.pack_start(&capture_but);

        header_bar.pack_start(&Separator::new(Orientation::Vertical));

        header_bar.pack_start(&decrypt_button);
        header_bar.pack_start(&settings_button);
        header_bar.pack_start(&about_button);
        header_bar.pack_start(&update_button);

        header_bar.pack_end(&ghz_5_but);
        header_bar.pack_end(&ghz_2_4_but);

        header_bar.pack_end(&channel_filter_entry);

        header_bar.pack_end(&hopping_but);
        header_bar.pack_end(&focus_but);
        header_bar.pack_end(&add_but);

        // Left View (Access Points and Clients)

        let aps_model = build_aps_model();
        let aps_view = build_aps_view();
        let aps_scroll = build_aps_scroll();
        let aps_menu = build_aps_menu();

        aps_menu.set_parent(&aps_scroll);
        aps_scroll.set_child(Some(&aps_view));
        aps_view.set_model(Some(&aps_model));

        let cli_model = build_cli_model();
        let cli_view = build_cli_view();
        let cli_scroll = build_cli_scroll();
        let cli_menu = build_cli_menu();

        cli_menu.set_parent(&cli_scroll);
        cli_scroll.set_child(Some(&cli_view));
        cli_view.set_model(Some(&cli_model));

        // Set main window childs

        let panned_cli_aps = Paned::new(Orientation::Vertical);
        panned_cli_aps.set_wide_handle(true);
        panned_cli_aps.set_start_child(Some(&aps_scroll));
        panned_cli_aps.set_end_child(Some(&cli_scroll));

        let client_status_bar = Statusbar::new();
        client_status_bar.push(0, "显示未关联的客户端");

        let channel_status_bar = Statusbar::new();
        channel_status_bar
            .set_tooltip_text(Some("网卡当前监听的信道"));
        channel_status_bar.push(0, "信道：无");

        let iface_status_bar = Statusbar::new();
        iface_status_bar.set_tooltip_text(Some("用于扫描和攻击的无线网卡"));
        iface_status_bar.push(0, "网卡：无");

        let status_bar = Box::new(Orientation::Horizontal, 0);
        status_bar.append(&client_status_bar);
        status_bar.append(&channel_status_bar);
        status_bar.append(&iface_status_bar);

        client_status_bar.set_hexpand(true);

        let vbox = Box::new(Orientation::Vertical, 0);
        vbox.append(&panned_cli_aps);
        vbox.append(&status_bar);

        window.set_child(Some(&vbox));

        Self {
            // Header bar
            about_button,
            update_button,
            decrypt_button,
            settings_button,
            // Main window
            window,
            aps_model,
            aps_view,
            aps_scroll,
            aps_menu,
            cli_model,
            cli_view,
            cli_scroll,
            cli_menu,
            ghz_2_4_but,
            ghz_5_but,
            channel_filter_entry,
            scan_but,
            restart_but,
            export_but,
            report_but,
            previous_but,
            next_but,
            top_but,
            bottom_but,
            hopping_but,
            focus_but,
            add_but,
            deauth_but,
            pmkid_but,
            capture_but,
            client_status_bar,
            channel_status_bar,
            iface_status_bar,
        }
    }

    pub fn show(&self) {
        self.window.show();
    }
}
