use crate::CodeCard;
use dioxus::prelude::*;
use m3e_rust_ui::*;

const LOGIN_TEMPLATE_USAGE: &str = r#"FormLayout {
    title: "Welcome back".to_string(),
    icon: Some(icons::PERSON),
    description: Some("Sign in to your workspace.".to_string()),
    TextField { label: "Work email".to_string(), input_type: "email".to_string(), value: email(), oninput: move |v| email.set(v) }
    TextField { label: "Password".to_string(), input_type: "password".to_string(), value: password(), oninput: move |v| password.set(v) }
    Button { button_type: ButtonType::Submit, variant: ButtonVariant::Filled, "Sign in" }
}"#;

const DASHBOARD_TEMPLATE_USAGE: &str = r#"ConnectedButtonGroup {
    selection: ButtonGroupSelection::Single,
    selection_required: true,
    variant: ConnectedButtonVariant::Tonal,
    items: vec![ConnectedButtonItem::new("7 days"), ConnectedButtonItem::new("30 days"), ConnectedButtonItem::new("90 days")],
    selected: vec![1],
    onchange: move |idx| timeframe.set(idx[0]),
}
Chart { label: "Sessions".to_string(), kind: ChartKind::Bar, series, data }"#;

const INBOX_TEMPLATE_USAGE: &str = r#"ListDetailLayout {
    show_detail: detail_open(),
    list: rsx! { /* selectable thread list */ },
    detail: rsx! { /* thread body, star/archive IconButtons, and quick-reply TextField */ },
}"#;

const SETTINGS_TEMPLATE_USAGE: &str = r#"SettingsSection {
    title: "Appearance & Accessibility".to_string(),
    SettingsItem {
        title: "Compact density".to_string(),
        Switch { checked: compact_density(), onchange: move |v| compact_density.set(v) }
    }
}"#;

const PRICING_TEMPLATE_USAGE: &str = r#"ConnectedButtonGroup {
    selection: ButtonGroupSelection::Single,
    selection_required: true,
    variant: ConnectedButtonVariant::Tonal,
    items: vec![ConnectedButtonItem::new("Monthly"), ConnectedButtonItem::new("Annual · Save 20%")],
    selected: vec![1],
    onchange: move |idx| annual_billing.set(idx[0] == 1),
}"#;

#[derive(Clone)]
struct InboxThread {
    id: usize,
    sender: &'static str,
    subject: &'static str,
    preview: &'static str,
    time: &'static str,
    unread: bool,
    starred: bool,
    body: &'static str,
    attachment: Option<&'static str>,
}

const INITIAL_THREADS: [(&str, &str, &str, &str, bool, bool, &str, Option<&str>); 4] = [
    (
        "Mira Lin",
        "M3 Expressive token review",
        "Updated the tonal surface roles and spring durations for Q4.",
        "10:24",
        true,
        true,
        "Hi team — I updated the tonal surface roles and spring durations for the Q4 release. Please review the new primary-container contrast on the dashboard cards before Friday's freeze.",
        Some("m3e-tokens-v4.json"),
    ),
    (
        "Jonah Vance",
        "Wasm bundle size benchmark",
        "Release build dropped by 14% after tree-shaking unused icons.",
        "09:15",
        true,
        false,
        "Good news: the latest release build dropped by 14% after tree-shaking unused icons and splitting the chart helper. Metrics are attached for the weekly sync.",
        Some("wasm-size-report.csv"),
    ),
    (
        "Clara Moreau",
        "Design system office hours",
        "Notes from yesterday's accessibility & keyboard navigation audit.",
        "Yesterday",
        false,
        false,
        "Thanks everyone for joining! We walked through roving tabindex on navigation rails, Escape dismissal on sheets, and reduced-motion fallbacks across all interactive controls.",
        None,
    ),
    (
        "DevOps Bot",
        "Production deployment #418",
        "Deployed v0.1.0 preview to GitHub Pages with 0 warnings.",
        "Mon",
        false,
        false,
        "Deployment #418 succeeded in 1m 42s. All asset hashes verified and Lighthouse accessibility score is 100.",
        None,
    ),
];

#[component]
pub fn TemplatesShowcase(active: String) -> Element {
    // Login state
    let mut email = use_signal(|| String::from("alex@material.dev"));
    let mut password = use_signal(|| String::from("••••••••••••"));
    let mut show_password = use_signal(|| false);
    let mut remember_me = use_signal(|| true);
    let mut signed_in_user = use_signal(|| None::<String>);
    let mut login_error = use_signal(|| None::<String>);

    // Dashboard state
    let mut timeframe = use_signal(|| String::from("30d"));
    let mut report_banner = use_signal(|| false);

    // Inbox state
    let mut threads = use_signal(|| {
        INITIAL_THREADS
            .iter()
            .enumerate()
            .map(|(id, t)| InboxThread {
                id,
                sender: t.0,
                subject: t.1,
                preview: t.2,
                time: t.3,
                unread: t.4,
                starred: t.5,
                body: t.6,
                attachment: t.7,
            })
            .collect::<Vec<_>>()
    });
    let mut inbox_filter = use_signal(|| String::from("all"));
    let mut selected_thread = use_signal(|| 0_usize);
    let mut reply_draft = use_signal(String::new);
    let mut sent_replies = use_signal(Vec::<(usize, String)>::new);
    let mut archive_snack = use_signal(|| false);
    let mut archive_rev = use_signal(|| 0_u64);

    // Settings state
    let mut profile_name = use_signal(|| String::from("Alex Rivera"));
    let mut profile_role = use_signal(|| String::from("alex@material.dev · Workspace Admin"));
    let mut profile_dialog = use_signal(|| false);
    let mut draft_name = use_signal(|| String::from("Alex Rivera"));
    let mut draft_role = use_signal(|| String::from("alex@material.dev · Workspace Admin"));
    let mut workspace_lang = use_signal(|| String::from("en"));
    let mut lang_open = use_signal(|| false);
    let mut compact_density = use_signal(|| false);
    let mut high_contrast = use_signal(|| false);
    let mut email_digest = use_signal(|| true);
    let mut push_mentions = use_signal(|| true);
    let mut two_factor = use_signal(|| true);
    let mut settings_saved = use_signal(|| false);

    // Pricing state
    let mut annual_billing = use_signal(|| true);
    let mut selected_plan = use_signal(|| String::from("Pro"));

    let (users_kpi, revenue_kpi, conv_kpi, chart_points) = match timeframe().as_str() {
        "7d" => (
            "6,420",
            "$21.4k",
            "5.1%",
            vec![
                ChartPoint::new("Mon", vec![42.0, 28.0]),
                ChartPoint::new("Tue", vec![58.0, 35.0]),
                ChartPoint::new("Wed", vec![51.0, 31.0]),
                ChartPoint::new("Thu", vec![74.0, 46.0]),
                ChartPoint::new("Fri", vec![82.0, 52.0]),
                ChartPoint::new("Sat", vec![64.0, 39.0]),
                ChartPoint::new("Sun", vec![71.0, 44.0]),
            ],
        ),
        "90d" => (
            "71,940",
            "$248.6k",
            "4.6%",
            vec![
                ChartPoint::new("Jul", vec![52.0, 34.0]),
                ChartPoint::new("Aug", vec![67.0, 41.0]),
                ChartPoint::new("Sep", vec![79.0, 49.0]),
                ChartPoint::new("Oct", vec![88.0, 57.0]),
                ChartPoint::new("Nov", vec![94.0, 62.0]),
            ],
        ),
        _ => (
            "24,810",
            "$84.2k",
            "4.8%",
            vec![
                ChartPoint::new("W1", vec![48.0, 30.0]),
                ChartPoint::new("W2", vec![63.0, 42.0]),
                ChartPoint::new("W3", vec![71.0, 48.0]),
                ChartPoint::new("W4", vec![86.0, 59.0]),
            ],
        ),
    };

    let chart_series = vec![
        ChartSeries::new("sessions", "Active sessions (k)", "var(--md-sys-color-primary)"),
        ChartSeries::new("conversions", "Sign-ups (k)", "var(--md-sys-color-tertiary)"),
    ];

    let visible_threads: Vec<InboxThread> = threads()
        .into_iter()
        .filter(|t| match inbox_filter().as_str() {
            "unread" => t.unread,
            "starred" => t.starred,
            _ => true,
        })
        .collect();

    let current_thread = threads()
        .into_iter()
        .find(|t| t.id == selected_thread())
        .or_else(|| visible_threads.first().cloned());

    rsx! {
        // 1. Sign in & Auth Template
        section {
            id: "template-login",
            class: "wrap roles-section template-page-section",
            hidden: active != "template-login",
            div { class: "template-stage template-stage--centered",
                article { class: "template-auth-card",
                    div { class: "template-auth-brand",
                        div { class: "template-auth-badge",
                            Shape { name: Some(ShapeName::FourLeafClover), size: 52, class: "template-auth-shape" }
                            Icon { icon: icons::AUTO_AWESOME }
                        }
                        div {
                            span { class: "eyebrow", "M3E WORKSPACE" }
                            h2 { "Welcome back" }
                            p { "Sign in to continue to your design & engineering workspace." }
                        }
                    }

                    if let Some(user) = signed_in_user() {
                        Alert {
                            variant: AlertVariant::Tertiary,
                            title: "Signed in".to_string(),
                            description: format!("Authenticated as {user}. Your session is active."),
                        }
                        div { class: "template-auth-actions",
                            Button {
                                variant: ButtonVariant::Tonal,
                                onclick: move |_| signed_in_user.set(None),
                                "Sign out"
                            }
                        }
                    } else {
                        if let Some(err) = login_error() {
                            Alert {
                                variant: AlertVariant::Error,
                                title: "Check your credentials".to_string(),
                                description: err,
                            }
                        }
                        form {
                            class: "template-auth-form",
                            onsubmit: move |e| {
                                e.prevent_default();
                                if email().trim().is_empty() || password().trim().is_empty() {
                                    login_error.set(Some("Enter both your email and password to continue.".to_string()));
                                } else {
                                    login_error.set(None);
                                    signed_in_user.set(Some(email()));
                                }
                            },
                            TextField {
                                label: "Work email".to_string(),
                                input_type: "email".to_string(),
                                value: email(),
                                leading_icon: Some(icons::PERSON),
                                oninput: move |v| email.set(v),
                            }
                            TextField {
                                label: "Password".to_string(),
                                input_type: if show_password() { "text".to_string() } else { "password".to_string() },
                                value: password(),
                                trailing_icon: Some(if show_password() { icons::VISIBILITY_OFF } else { icons::VISIBILITY }),
                                oninput: move |v| password.set(v),
                            }
                            div { class: "template-auth-row",
                                label { class: "template-check-label",
                                    Checkbox {
                                        checked: remember_me(),
                                        aria_label: Some("Keep me signed in".to_string()),
                                        onchange: move |v| remember_me.set(v),
                                    }
                                    span { "Keep me signed in" }
                                }
                                Button {
                                    variant: ButtonVariant::Text,
                                    onclick: move |_| show_password.toggle(),
                                    if show_password() { "Hide password" } else { "Show password" }
                                }
                            }
                            Button {
                                button_type: ButtonType::Submit,
                                variant: ButtonVariant::Filled,
                                size: ButtonSize::Medium,
                                class: "template-full-btn".to_string(),
                                "Sign in"
                            }
                        }
                        div { class: "template-divider-row",
                            Divider {}
                            span { "OR CONTINUE WITH" }
                            Divider {}
                        }
                        div { class: "template-sso-row",
                            Button {
                                variant: ButtonVariant::Outlined,
                                leading_icon: Some(rsx! { Icon { icon: icons::STAR } }),
                                onclick: move |_| {
                                    login_error.set(None);
                                    signed_in_user.set(Some("Passkey · alex@material.dev".to_string()));
                                },
                                "Passkey"
                            }
                            Button {
                                variant: ButtonVariant::Outlined,
                                onclick: move |_| {
                                    login_error.set(None);
                                    signed_in_user.set(Some("Google Workspace SSO".to_string()));
                                },
                                "Workspace SSO"
                            }
                        }
                    }
                }
                CodeCard { eyebrow: "TEMPLATE SNIPPET", title: "Sign-in & auth usage", code: LOGIN_TEMPLATE_USAGE }
            }
        }

        // 2. Analytics Dashboard Template
        section {
            id: "template-dashboard",
            class: "wrap roles-section template-page-section",
            hidden: active != "template-dashboard",
            div { class: "template-stage",
                div { class: "template-dash-header",
                    div {
                        span { class: "eyebrow", "LIVE WORKSPACE TELEMETRY" }
                        h2 { "Product overview" }
                    }
                    div { class: "template-dash-controls",
                        ConnectedButtonGroup {
                            aria_label: Some("Timeframe".to_string()),
                            selection: ButtonGroupSelection::Single,
                            selection_required: true,
                            variant: ConnectedButtonVariant::Tonal,
                            size: ButtonSize::Small,
                            items: vec![
                                ConnectedButtonItem::new("7 days"),
                                ConnectedButtonItem::new("30 days"),
                                ConnectedButtonItem::new("90 days"),
                            ],
                            selected: vec![match timeframe().as_str() {
                                "7d" => 0,
                                "90d" => 2,
                                _ => 1,
                            }],
                            onchange: move |indices: Vec<usize>| {
                                if let Some(&idx) = indices.first() {
                                    timeframe.set(match idx {
                                        0 => "7d".to_string(),
                                        2 => "90d".to_string(),
                                        _ => "30d".to_string(),
                                    });
                                }
                            },
                        }
                        Button {
                            variant: ButtonVariant::Filled,
                            leading_icon: Some(rsx! { Icon { icon: icons::ADD } }),
                            onclick: move |_| report_banner.toggle(),
                            "New report"
                        }
                    }
                }

                if report_banner() {
                    Alert {
                        variant: AlertVariant::Info,
                        title: "Report snapshot ready".to_string(),
                        description: format!("Generated summary for the {} window. Click 'New report' again to dismiss.", timeframe()),
                    }
                }

                div { class: "template-kpi-grid",
                    article { class: "template-kpi-card",
                        div { class: "template-kpi-top",
                            span { "Active users" }
                            span { class: "template-kpi-badge", Icon { icon: icons::PERSON } }
                        }
                        strong { class: "template-kpi-value", "{users_kpi}" }
                        div { class: "template-kpi-meta",
                            span { class: "template-delta", "+14.2%" }
                            span { "vs previous period" }
                        }
                        LinearProgress { value: Some(0.76) }
                    }
                    article { class: "template-kpi-card",
                        div { class: "template-kpi-top",
                            span { "Monthly revenue" }
                            span { class: "template-kpi-badge", Icon { icon: icons::TRENDING_UP } }
                        }
                        strong { class: "template-kpi-value", "{revenue_kpi}" }
                        div { class: "template-kpi-meta",
                            span { class: "template-delta", "+9.8%" }
                            span { "Net recurring" }
                        }
                        LinearProgress { value: Some(0.84) }
                    }
                    article { class: "template-kpi-card",
                        div { class: "template-kpi-top",
                            span { "Conversion rate" }
                            span { class: "template-kpi-badge", Icon { icon: icons::ANALYTICS } }
                        }
                        strong { class: "template-kpi-value", "{conv_kpi}" }
                        div { class: "template-kpi-meta",
                            span { class: "template-delta", "+0.6%" }
                            span { "Trial to paid" }
                        }
                        LinearProgress { value: Some(0.62) }
                    }
                    article { class: "template-kpi-card",
                        div { class: "template-kpi-top",
                            span { "API availability" }
                            span { class: "template-kpi-badge", Icon { icon: icons::CHECK_CIRCLE } }
                        }
                        strong { class: "template-kpi-value", "99.98%" }
                        div { class: "template-kpi-meta",
                            span { class: "template-delta", "0 incidents" }
                            span { "Global edge" }
                        }
                        LinearProgress { value: Some(0.99) }
                    }
                }

                div { class: "template-dash-grid",
                    article { class: "demo-card template-dash-panel",
                        div { class: "card-topline", span { "TRAFFIC & SIGN-UPS" } span { class: "component-index", "CHART" } }
                        Chart {
                            label: "Sessions and sign-ups".to_string(),
                            kind: ChartKind::Bar,
                            series: chart_series,
                            data: chart_points,
                        }
                    }
                    article { class: "demo-card template-dash-panel",
                        div { class: "card-topline", span { "TEAM ACTIVITY" } span { class: "component-index", "FEED" } }
                        div { class: "template-activity-list",
                            for (name, action, role, time) in [
                                ("Mira Lin", "Published Material 3 color tokens v4", "Design", "12m ago"),
                                ("Jonah Vance", "Merged wasm-opt size reduction PR #142", "Core", "48m ago"),
                                ("Clara Moreau", "Verified keyboard focus order on Sheets", "A11y", "2h ago"),
                                ("Sami Hallak", "Released m3e-rust-ui v0.1.0 crate", "Release", "5h ago"),
                            ] {
                                div { class: "template-activity-row",
                                    Avatar { label: name.to_string(), size: AvatarSize::Small }
                                    div { class: "template-activity-copy",
                                        strong { "{name}" }
                                        p { "{action}" }
                                    }
                                    div { class: "template-activity-meta",
                                        span { class: "template-pill-tag", "{role}" }
                                        small { "{time}" }
                                    }
                                }
                            }
                        }
                    }
                }

                article { class: "demo-card template-dash-panel",
                    div { class: "card-topline", span { "RECENT RELEASES" } span { class: "component-index", "TABLE" } }
                    DataTable {
                        label: "Recent workspace releases".to_string(),
                        columns: vec![
                            DataColumn::new("release", "Release"),
                            DataColumn::new("owner", "Owner"),
                            DataColumn::new("bundle", "Bundle (KB)").numeric(),
                            DataColumn::new("status", "Status"),
                        ],
                        rows: vec![
                            DataRow::new("v0.1.4", vec![DataCell::text("v0.1.4-rc1"), DataCell::text("Mira Lin"), DataCell::formatted_number(184.0, "184 KB"), DataCell::text("Ready")]),
                            DataRow::new("v0.1.3", vec![DataCell::text("v0.1.3"), DataCell::text("Jonah Vance"), DataCell::formatted_number(192.0, "192 KB"), DataCell::text("Deployed")]),
                            DataRow::new("v0.1.2", vec![DataCell::text("v0.1.2"), DataCell::text("Clara Moreau"), DataCell::formatted_number(214.0, "214 KB"), DataCell::text("Deployed")]),
                            DataRow::new("v0.1.1", vec![DataCell::text("v0.1.1"), DataCell::text("Sami Hallak"), DataCell::formatted_number(218.0, "218 KB"), DataCell::text("Archived")]),
                        ],
                    }
                }
                CodeCard { eyebrow: "TEMPLATE SNIPPET", title: "Analytics dashboard usage", code: DASHBOARD_TEMPLATE_USAGE }
            }
        }

        // 3. Inbox Workspace Template (List-Detail)
        section {
            id: "template-inbox",
            class: "wrap roles-section template-page-section",
            hidden: active != "template-inbox",
            div { class: "template-stage",
                div { class: "template-inbox-bar",
                    div { class: "template-chip-row",
                        for (id, label) in [("all", "All"), ("unread", "Unread"), ("starred", "Starred")] {
                            Chip {
                                label: label.to_string(),
                                variant: ChipVariant::Filter,
                                selected: inbox_filter() == id,
                                onclick: move |_| inbox_filter.set(id.to_string()),
                            }
                        }
                    }
                    span { class: "template-inbox-count", "{visible_threads.len()} conversations" }
                }

                div { class: "template-inbox-split",
                    div { class: "template-inbox-list",
                        for thread in visible_threads.iter() {
                            {
                                let id = thread.id;
                                let is_selected = current_thread.as_ref().map(|c| c.id) == Some(id);
                                rsx! {
                                    button {
                                        r#type: "button",
                                        class: if is_selected { "template-thread-item template-thread-item--active" } else { "template-thread-item" },
                                        onclick: move |_| {
                                            selected_thread.set(id);
                                            threads.with_mut(|list| {
                                                if let Some(item) = list.iter_mut().find(|t| t.id == id) {
                                                    item.unread = false;
                                                }
                                            });
                                        },
                                        Avatar { label: thread.sender.to_string(), size: AvatarSize::Small }
                                        div { class: "template-thread-copy",
                                            div { class: "template-thread-head",
                                                strong { "{thread.sender}" }
                                                span { "{thread.time}" }
                                            }
                                            div { class: "template-thread-subject",
                                                if thread.unread { span { class: "template-unread-dot" } }
                                                "{thread.subject}"
                                            }
                                            p { "{thread.preview}" }
                                        }
                                    }
                                }
                            }
                        }
                        if visible_threads.is_empty() {
                            EmptyState {
                                title: "No matching messages".to_string(),
                                description: "Try switching back to the 'All' filter.".to_string(),
                            }
                        }
                    }

                    div { class: "template-inbox-detail",
                        if let Some(thread) = current_thread.clone() {
                            div { class: "template-detail-header",
                                div { class: "template-detail-sender",
                                    Avatar { label: thread.sender.to_string(), size: AvatarSize::Medium }
                                    div {
                                        h3 { "{thread.subject}" }
                                        p { "{thread.sender} · {thread.time}" }
                                    }
                                }
                                div { class: "template-detail-actions",
                                    IconButton {
                                        icon: icons::STAR,
                                        toggle: true,
                                        selected: thread.starred,
                                        aria_label: Some("Toggle star".to_string()),
                                        onclick: move |_| {
                                            let id = thread.id;
                                            threads.with_mut(|list| {
                                                if let Some(item) = list.iter_mut().find(|t| t.id == id) {
                                                    item.starred = !item.starred;
                                                }
                                            });
                                        },
                                    }
                                    IconButton {
                                        icon: icons::ARCHIVE,
                                        aria_label: Some("Archive thread".to_string()),
                                        onclick: move |_| {
                                            let id = thread.id;
                                            threads.with_mut(|list| list.retain(|t| t.id != id));
                                            archive_rev += 1;
                                            archive_snack.set(true);
                                        },
                                    }
                                }
                            }
                            Divider {}
                            p { class: "template-detail-body", "{thread.body}" }
                            if let Some(file) = thread.attachment {
                                div { class: "template-attachment-chip",
                                    Chip { label: file.to_string(), variant: ChipVariant::Assist }
                                }
                            }
                            for (tid, msg) in sent_replies().into_iter().filter(|(tid, _)| *tid == thread.id) {
                                div { key: "{tid}-{msg}", class: "template-sent-reply",
                                    strong { "You" }
                                    p { "{msg}" }
                                }
                            }
                            form {
                                class: "template-reply-box",
                                onsubmit: move |e| {
                                    e.prevent_default();
                                    let text = reply_draft().trim().to_string();
                                    if !text.is_empty() {
                                        sent_replies.write().push((thread.id, text));
                                        reply_draft.set(String::new());
                                    }
                                },
                                TextField {
                                    label: "Quick reply".to_string(),
                                    value: reply_draft(),
                                    oninput: move |v| reply_draft.set(v),
                                }
                                Button {
                                    button_type: ButtonType::Submit,
                                    variant: ButtonVariant::Filled,
                                    "Send"
                                }
                            }
                        } else {
                            EmptyState {
                                title: "Select a conversation".to_string(),
                                description: "Choose a thread from the list to read and reply.".to_string(),
                            }
                        }
                    }
                }
                Snackbar {
                    open: archive_snack(),
                    message: "Conversation archived.".to_string(),
                    revision: archive_rev(),
                    duration_ms: 4000,
                    onopenchange: move |v| archive_snack.set(v),
                }
                CodeCard { eyebrow: "TEMPLATE SNIPPET", title: "Inbox workspace usage", code: INBOX_TEMPLATE_USAGE }
            }
        }

        // 4. Account Settings Template
        section {
            id: "template-settings",
            class: "wrap roles-section template-page-section",
            hidden: active != "template-settings",
            div { class: "template-stage template-stage--narrow",
                article { class: "template-profile-card",
                    Avatar { label: profile_name(), size: AvatarSize::Large }
                    div { class: "template-profile-copy",
                        h3 { "{profile_name}" }
                        p { "{profile_role}" }
                    }
                    Button {
                        variant: ButtonVariant::Tonal,
                        onclick: move |_| {
                            draft_name.set(profile_name());
                            draft_role.set(profile_role());
                            profile_dialog.set(true);
                        },
                        "Edit profile"
                    }
                }
                Dialog {
                    title: "Edit profile".to_string(),
                    description: Some("Update your display name and workspace role.".to_string()),
                    open: profile_dialog(),
                    onopenchange: move |v| profile_dialog.set(v),
                    actions: Some(rsx! {
                        Button { variant: ButtonVariant::Text, onclick: move |_| profile_dialog.set(false), "Cancel" }
                        Button {
                            variant: ButtonVariant::Filled,
                            onclick: move |_| {
                                if !draft_name().trim().is_empty() {
                                    profile_name.set(draft_name());
                                }
                                if !draft_role().trim().is_empty() {
                                    profile_role.set(draft_role());
                                }
                                profile_dialog.set(false);
                                settings_saved.set(true);
                            },
                            "Save"
                        }
                    }),
                    div { class: "template-auth-form",
                        TextField { label: "Display name".to_string(), value: draft_name(), oninput: move |v| draft_name.set(v) }
                        TextField { label: "Role & email".to_string(), value: draft_role(), oninput: move |v| draft_role.set(v) }
                    }
                }

                if settings_saved() {
                    Alert {
                        variant: AlertVariant::Tertiary,
                        title: "Preferences saved".to_string(),
                        description: "Your workspace settings have been updated.".to_string(),
                    }
                }

                SettingsSection {
                    title: "Appearance & Accessibility".to_string(),
                    description: Some("Customize language, density, and contrast preferences.".to_string()),
                    SettingsItem {
                        title: "Interface language".to_string(),
                        description: Some("Applies to navigation labels and date formatting.".to_string()),
                        Select {
                            label: "Language".to_string(),
                            value: workspace_lang(),
                            open: lang_open(),
                            options: vec![
                                SelectOption::new("en", "English (US)"),
                                SelectOption::new("fr", "Français"),
                                SelectOption::new("ja", "日本語"),
                            ],
                            onchange: move |v| workspace_lang.set(v),
                            onopenchange: move |v| lang_open.set(v),
                        }
                    }
                    Divider { inset: DividerInset::Start }
                    SettingsItem {
                        title: "Compact density".to_string(),
                        description: Some("Tighten table rows and list items on desktop.".to_string()),
                        Switch {
                            checked: compact_density(),
                            aria_label: Some("Compact density".to_string()),
                            onchange: move |v| compact_density.set(v),
                        }
                    }
                    Divider { inset: DividerInset::Start }
                    SettingsItem {
                        title: "High contrast labels".to_string(),
                        description: Some("Boost outline and secondary label legibility.".to_string()),
                        Switch {
                            checked: high_contrast(),
                            aria_label: Some("High contrast labels".to_string()),
                            onchange: move |v| high_contrast.set(v),
                        }
                    }
                }

                SettingsSection {
                    title: "Notifications & Security".to_string(),
                    description: Some("Control digests, mentions, and account protection.".to_string()),
                    SettingsItem {
                        title: "Weekly email digest".to_string(),
                        description: Some("Summary of workspace releases every Monday.".to_string()),
                        Switch {
                            checked: email_digest(),
                            aria_label: Some("Weekly email digest".to_string()),
                            onchange: move |v| email_digest.set(v),
                        }
                    }
                    Divider { inset: DividerInset::Start }
                    SettingsItem {
                        title: "Push mentions".to_string(),
                        description: Some("Instant alerts when teammates @mention you.".to_string()),
                        Switch {
                            checked: push_mentions(),
                            aria_label: Some("Push mentions".to_string()),
                            onchange: move |v| push_mentions.set(v),
                        }
                    }
                    Divider { inset: DividerInset::Start }
                    SettingsItem {
                        title: "Two-factor authentication".to_string(),
                        description: Some("Require a passkey or hardware key at sign-in.".to_string()),
                        Switch {
                            checked: two_factor(),
                            aria_label: Some("Two-factor authentication".to_string()),
                            onchange: move |v| two_factor.set(v),
                        }
                    }
                }

                div { class: "template-settings-footer",
                    Button {
                        variant: ButtonVariant::Text,
                        onclick: move |_| {
                            compact_density.set(false);
                            high_contrast.set(false);
                            email_digest.set(true);
                            push_mentions.set(true);
                            two_factor.set(true);
                            settings_saved.set(false);
                        },
                        "Reset defaults"
                    }
                    Button {
                        variant: ButtonVariant::Filled,
                        onclick: move |_| settings_saved.set(true),
                        "Save preferences"
                    }
                }
                CodeCard { eyebrow: "TEMPLATE SNIPPET", title: "Account settings usage", code: SETTINGS_TEMPLATE_USAGE }
            }
        }

        // 5. Plans & Billing Template
        section {
            id: "template-pricing",
            class: "wrap roles-section template-page-section",
            hidden: active != "template-pricing",
            div { class: "template-stage",
                div { class: "template-pricing-top",
                    div {
                        span { class: "eyebrow", "FLEXIBLE LICENSING" }
                        h2 { "Choose a workspace plan" }
                        p { "Switch between monthly and annual billing at any time." }
                    }
                    ConnectedButtonGroup {
                        aria_label: Some("Billing cycle".to_string()),
                        selection: ButtonGroupSelection::Single,
                        selection_required: true,
                        variant: ConnectedButtonVariant::Tonal,
                        size: ButtonSize::Small,
                        items: vec![
                            ConnectedButtonItem::new("Monthly"),
                            ConnectedButtonItem::new("Annual · Save 20%"),
                        ],
                        selected: vec![if annual_billing() { 1 } else { 0 }],
                        onchange: move |indices: Vec<usize>| {
                            if let Some(&idx) = indices.first() {
                                annual_billing.set(idx == 1);
                            }
                        },
                    }
                }

                Alert {
                    variant: AlertVariant::Info,
                    title: format!("Selected plan: {}", selected_plan()),
                    description: if annual_billing() {
                        "Billed annually with 20% discount applied.".to_string()
                    } else {
                        "Billed monthly, cancel anytime.".to_string()
                    },
                }

                div { class: "template-pricing-grid",
                    for (name, monthly, annual, desc, featured, features) in [
                        (
                            "Starter",
                            "$0",
                            "$0",
                            "For personal projects and quick prototypes.",
                            false,
                            vec!["All 60+ M3E components", "Dynamic seed theming", "MIT / Apache-2.0 core"],
                        ),
                        (
                            "Pro",
                            "$24",
                            "$19",
                            "For product teams shipping expressive apps.",
                            true,
                            vec!["Everything in Starter", "Shared design tokens", "Priority accessibility audits", "Team template library"],
                        ),
                        (
                            "Enterprise",
                            "Custom",
                            "Custom",
                            "For organizations with custom compliance needs.",
                            false,
                            vec!["Dedicated design system support", "Custom theme presets", "SLA & security reviews"],
                        ),
                    ] {
                        {
                            let price = if annual_billing() { annual } else { monthly };
                            let is_current = selected_plan() == name;
                            let card_class = if featured { "template-plan-card template-plan-card--featured" } else { "template-plan-card" };
                            rsx! {
                                article { class: "{card_class}",
                                    div { class: "template-plan-head",
                                        h3 { "{name}" }
                                        if featured { span { class: "template-plan-badge", "Recommended" } }
                                    }
                                    p { class: "template-plan-desc", "{desc}" }
                                    div { class: "template-plan-price",
                                        strong { "{price}" }
                                        if price != "Custom" { span { "/ seat / mo" } }
                                    }
                                    ul { class: "template-plan-features",
                                        for feat in features {
                                            li {
                                                Icon { icon: icons::CHECK_CIRCLE }
                                                span { "{feat}" }
                                            }
                                        }
                                    }
                                    Button {
                                        variant: if is_current || featured { ButtonVariant::Filled } else { ButtonVariant::Outlined },
                                        class: "template-full-btn".to_string(),
                                        onclick: move |_| selected_plan.set(name.to_string()),
                                        if is_current { "Selected" } else { "Choose {name}" }
                                    }
                                }
                            }
                        }
                    }
                }
                CodeCard { eyebrow: "TEMPLATE SNIPPET", title: "Plans & billing usage", code: PRICING_TEMPLATE_USAGE }
            }
        }
    }
}
