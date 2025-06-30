use crate::prelude::*;
// use crate::self_update::SelfUpdateEvent;
// use crate::self_update::SelfUpdater;
use crate::Embedded;

use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex, mpsc::Receiver};

pub static CAMERA: LazyLock<Arc<RwLock<Box<dyn Camera>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Box::new(Orbit::new()))));
pub static RENDERER: LazyLock<Arc<Box<dyn Renderer>>> =
    LazyLock::new(|| Arc::new(Box::new(PBR::new())));

pub static SHAPES: LazyLock<Arc<RwLock<Vec<NodeFXGraph>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(vec![])));

pub static RENDERBUFFER: LazyLock<Arc<Mutex<RenderBuffer>>> =
    LazyLock::new(|| Arc::new(Mutex::new(RenderBuffer::new(100, 100))));

pub static VOXELGRID: LazyLock<Arc<RwLock<VoxelGrid>>> =
    LazyLock::new(|| Arc::new(RwLock::new(VoxelGrid::default())));
pub static PALETTE: LazyLock<Arc<RwLock<Palette>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Palette::default())));

pub static MODELEDITOR: LazyLock<RwLock<ModelEditor>> =
    LazyLock::new(|| RwLock::new(ModelEditor::new()));
pub static NODEEDITOR: LazyLock<RwLock<NodeEditor>> =
    LazyLock::new(|| RwLock::new(NodeEditor::new()));
pub static TOOLLIST: LazyLock<RwLock<ToolList>> =
    LazyLock::new(|| RwLock::new(ToolList::default()));
pub static UNDOMANAGER: LazyLock<RwLock<UndoManager>> =
    LazyLock::new(|| RwLock::new(UndoManager::default()));

pub struct Editor {
    event_receiver: Option<Receiver<TheEvent>>,

    context: Context,
    update_tracker: UpdateTracker,
    /*
    project: Project,
    project_path: Option<PathBuf>,

    sidebar: Sidebar,
    mapeditor: MapEditor,

    server_ctx: ServerContext,

    update_tracker: UpdateTracker,

    self_update_rx: Receiver<SelfUpdateEvent>,
    self_update_tx: Sender<SelfUpdateEvent>,
    self_updater: Arc<Mutex<SelfUpdater>>,

    update_counter: usize,

    build_values: ValueContainer,
    */
}

impl TheTrait for Editor {
    fn new() -> Self
    where
        Self: Sized,
    {
        let context = Context::default();

        //let (self_update_tx, self_update_rx) = channel();

        /*
        let mut project = Project::new();
        if let Some(bytes) = crate::Embedded::get("toml/config.toml") {
            if let Ok(source) = std::str::from_utf8(bytes.data.as_ref()) {
                project.config = source.to_string();
            }
        }

        #[cfg(not(target_os = "macos"))]
        let self_updater = SelfUpdater::new("markusmoenig", "Eldiron", "eldiron");
        #[cfg(target_os = "macos")]
        let self_updater = SelfUpdater::new("markusmoenig", "Eldiron", "Eldiron.app");
        */
        Self {
            event_receiver: None,
            context,
            update_tracker: UpdateTracker::new(),
            /*
            project,
            project_path: None,

            sidebar: Sidebar::new(),
            mapeditor: MapEditor::new(),

            server_ctx: ServerContext::default(),

            update_tracker: UpdateTracker::new(),
            event_receiver: None,

            self_update_rx,
            self_update_tx,
            self_updater: Arc::new(Mutex::new(self_updater)),

            update_counter: 0,

            build_values: ValueContainer::default(),*/
        }
    }

    fn init(&mut self, _ctx: &mut TheContext) {
        let mut grid = VOXELGRID.write().unwrap();
        let bottom = -((grid.bounds[1] / 2.0) as i32);

        for (index, tile) in &mut grid.tiles {
            if index.1 == bottom {
                tile.add_floor();
            }
        }

        let mut shapes = SHAPES.write().unwrap();
        for _ in 0..10 {
            let mut graph = NodeFXGraph::default();
            let mut node = NodeFX::new(NodeFXRole::Brush);
            node.position = Vec2::new(10, 10);
            graph.nodes.push(node);
            shapes.push(graph);
        }

        for file in Embedded::iter() {
            let name = file.as_ref();

            if name == "aurora.txt" {
                if let Some(bytes) = Embedded::get(name) {
                    if let Ok(string) = std::str::from_utf8(bytes.data.as_ref()) {
                        let mut palette = PALETTE.write().unwrap();
                        let _ = palette.load_paintnet(string);
                    }
                }
            }
        }

        /*
        let updater = Arc::clone(&self.self_updater);
        let tx = self.self_update_tx.clone();

        thread::spawn(move || {
            let mut updater = updater.lock().unwrap();

            if let Err(err) = updater.fetch_release_list() {
                tx.send(SelfUpdateEvent::UpdateError(err.to_string()))
                    .unwrap();
            };
        });*/
    }

    fn window_title(&self) -> String {
        "Shape-Z: Shape That World".to_string()
    }

    fn default_window_size(&self) -> (usize, usize) {
        (1200, 720)
    }

    fn window_icon(&self) -> Option<(Vec<u8>, u32, u32)> {
        if let Some(file) = Embedded::get("window_logo.png") {
            let data = std::io::Cursor::new(file.data);

            let decoder = png::Decoder::new(data);
            if let Ok(mut reader) = decoder.read_info() {
                let mut buf = vec![0; reader.output_buffer_size()];
                let info = reader.next_frame(&mut buf).unwrap();
                let bytes = &buf[..info.buffer_size()];

                Some((bytes.to_vec(), info.width, info.height))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn init_ui(&mut self, ui: &mut TheUI, ctx: &mut TheContext) {
        ctx.ui
            .send(TheEvent::Custom(TheId::named("Startup"), TheValue::Empty));

        // Embedded Icons
        for file in Embedded::iter() {
            let name = file.as_ref();

            if name.ends_with(".png") {
                if let Some(file) = Embedded::get(name) {
                    let data = std::io::Cursor::new(file.data);

                    let decoder = png::Decoder::new(data);
                    if let Ok(mut reader) = decoder.read_info() {
                        let mut buf = vec![0; reader.output_buffer_size()];
                        let info = reader.next_frame(&mut buf).unwrap();
                        let bytes = &buf[..info.buffer_size()];

                        let mut cut_name = name.replace("icons/", "");
                        cut_name = cut_name.replace(".png", "");

                        ctx.ui.add_icon(
                            cut_name.to_string(),
                            TheRGBABuffer::from(bytes.to_vec(), info.width, info.height),
                        );
                    }
                }
            }
        }

        // ---

        ui.set_statusbar_name("Statusbar".to_string());

        // Menu

        let mut menu_canvas = TheCanvas::new();
        let mut menu = TheMenu::new(TheId::named("Menu"));

        let mut file_menu = TheContextMenu::named(str!("File"));
        file_menu.add(TheContextMenuItem::new(str!("New"), TheId::named("New")));
        file_menu.add_separator();
        file_menu.add(TheContextMenuItem::new_with_accel(
            str!("Open..."),
            TheId::named("Open"),
            TheAccelerator::new(TheAcceleratorKey::CTRLCMD, 'o'),
        ));
        file_menu.add(TheContextMenuItem::new_with_accel(
            str!("Save"),
            TheId::named("Save"),
            TheAccelerator::new(TheAcceleratorKey::CTRLCMD, 's'),
        ));
        file_menu.add(TheContextMenuItem::new_with_accel(
            str!("Save As ..."),
            TheId::named("Save As"),
            TheAccelerator::new(TheAcceleratorKey::CTRLCMD, 'a'),
        ));
        let mut edit_menu = TheContextMenu::named(str!("Edit"));
        edit_menu.add(TheContextMenuItem::new_with_accel(
            str!("Undo"),
            TheId::named("Undo"),
            TheAccelerator::new(TheAcceleratorKey::CTRLCMD, 'z'),
        ));
        edit_menu.add(TheContextMenuItem::new_with_accel(
            str!("Redo"),
            TheId::named("Redo"),
            TheAccelerator::new(TheAcceleratorKey::CTRLCMD | TheAcceleratorKey::SHIFT, 'z'),
        ));
        edit_menu.add_separator();
        edit_menu.add(TheContextMenuItem::new_with_accel(
            str!("Cut"),
            TheId::named("Cut"),
            TheAccelerator::new(TheAcceleratorKey::CTRLCMD, 'x'),
        ));
        edit_menu.add(TheContextMenuItem::new_with_accel(
            str!("Copy"),
            TheId::named("Copy"),
            TheAccelerator::new(TheAcceleratorKey::CTRLCMD, 'c'),
        ));
        edit_menu.add(TheContextMenuItem::new_with_accel(
            str!("Paste"),
            TheId::named("Paste"),
            TheAccelerator::new(TheAcceleratorKey::CTRLCMD, 'v'),
        ));

        file_menu.register_accel(ctx);
        edit_menu.register_accel(ctx);

        menu.add_context_menu(file_menu);
        menu.add_context_menu(edit_menu);
        menu_canvas.set_widget(menu);

        // Menubar
        let mut top_canvas = TheCanvas::new();

        let mut menubar = TheMenubar::new(TheId::named("Menubar"));
        menubar.limiter_mut().set_max_height(43 + 22);

        let mut logo_button = TheMenubarButton::new(TheId::named("Logo"));
        logo_button.set_icon_name("logo_toolbar".to_string());
        logo_button.set_status_text("Open the Eldiron Website ...");

        let mut open_button = TheMenubarButton::new(TheId::named("Open"));
        open_button.set_icon_name("icon_role_load".to_string());
        open_button.set_status_text("Open an existing Eldiron project...");

        let mut save_button = TheMenubarButton::new(TheId::named("Save"));
        save_button.set_status_text("Save the current project.");
        save_button.set_icon_name("icon_role_save".to_string());

        let mut save_as_button = TheMenubarButton::new(TheId::named("Save As"));
        save_as_button.set_icon_name("icon_role_save_as".to_string());
        save_as_button.set_status_text("Save the current project to a new file.");
        save_as_button.set_icon_offset(Vec2::new(2, -5));

        let mut undo_button = TheMenubarButton::new(TheId::named("Undo"));
        undo_button.set_status_text("Undo the last action.");
        undo_button.set_icon_name("icon_role_undo".to_string());

        let mut redo_button = TheMenubarButton::new(TheId::named("Redo"));
        redo_button.set_status_text("Redo the last action.");
        redo_button.set_icon_name("icon_role_redo".to_string());

        let mut play_button = TheMenubarButton::new(TheId::named("Play"));
        play_button.set_status_text("Start the server for live editing and debugging.");
        play_button.set_icon_name("play".to_string());
        //play_button.set_fixed_size(vec2i(28, 28));

        let mut pause_button = TheMenubarButton::new(TheId::named("Pause"));
        pause_button.set_status_text("Pause. Click for single stepping the server.");
        pause_button.set_icon_name("play-pause".to_string());

        let mut stop_button = TheMenubarButton::new(TheId::named("Stop"));
        stop_button.set_status_text("Stop the server.");
        stop_button.set_icon_name("stop-fill".to_string());

        let mut time_slider = TheTimeSlider::new(TheId::named("Server Time Slider"));
        time_slider.set_status_text("Adjust the server time.");
        time_slider.set_continuous(true);
        time_slider.limiter_mut().set_max_width(400);
        time_slider.set_value(TheValue::Time(TheTime::default()));

        let mut update_button = TheMenubarButton::new(TheId::named("Update"));
        update_button.set_status_text("Update application.");
        update_button.set_icon_name("arrows-clockwise".to_string());

        let mut patreon_button = TheMenubarButton::new(TheId::named("Patreon"));
        patreon_button.set_status_text("Visit my Patreon page.");
        patreon_button.set_icon_name("patreon".to_string());
        // patreon_button.set_fixed_size(vec2i(36, 36));
        patreon_button.set_icon_offset(Vec2::new(-4, -2));

        let mut hlayout = TheHLayout::new(TheId::named("Menu Layout"));
        hlayout.set_background_color(None);
        hlayout.set_margin(Vec4::new(10, 2, 10, 1));
        hlayout.add_widget(Box::new(logo_button));
        hlayout.add_widget(Box::new(TheMenubarSeparator::new(TheId::empty())));
        hlayout.add_widget(Box::new(open_button));
        hlayout.add_widget(Box::new(save_button));
        hlayout.add_widget(Box::new(save_as_button));
        hlayout.add_widget(Box::new(TheMenubarSeparator::new(TheId::empty())));
        hlayout.add_widget(Box::new(undo_button));
        hlayout.add_widget(Box::new(redo_button));
        hlayout.add_widget(Box::new(TheMenubarSeparator::new(TheId::empty())));
        hlayout.add_widget(Box::new(play_button));
        hlayout.add_widget(Box::new(pause_button));
        hlayout.add_widget(Box::new(stop_button));
        hlayout.add_widget(Box::new(TheMenubarSeparator::new(TheId::empty())));
        hlayout.add_widget(Box::new(time_slider));
        //hlayout.add_widget(Box::new(TheMenubarSeparator::new(TheId::empty())));

        hlayout.add_widget(Box::new(update_button));
        hlayout.add_widget(Box::new(TheMenubarSeparator::new(TheId::empty())));
        hlayout.add_widget(Box::new(patreon_button));

        hlayout.set_reverse_index(Some(3));

        top_canvas.set_widget(menubar);
        top_canvas.set_layout(hlayout);
        top_canvas.set_top(menu_canvas);
        ui.canvas.set_top(top_canvas);

        let mut model_canvas = TheCanvas::new();
        let model_view = TheRenderView::new(TheId::named("ModelView"));
        model_canvas.set_widget(model_view);

        let mut stack_canvas = TheCanvas::new();
        let mut stack_layout = TheStackLayout::new(TheId::named("StackLayout"));

        let mut palette_canvas = TheCanvas::default();
        let palette_picker = ThePalettePicker::new(TheId::named("PalettePicker"));
        palette_canvas.set_widget(palette_picker);

        stack_layout.add_canvas(palette_canvas);

        stack_canvas.set_layout(stack_layout);

        let mut hsplitlayout = TheSharedHLayout::new(TheId::named("Shared HLayout"));
        hsplitlayout.add_canvas(stack_canvas);
        hsplitlayout.add_canvas(model_canvas);
        hsplitlayout.set_shared_ratio(0.25);
        hsplitlayout.set_mode(TheSharedHLayoutMode::Shared);

        let mut top_canvas = TheCanvas::new();
        top_canvas.set_layout(hsplitlayout);

        // Tool Params
        let mut toolbar_hlayout = TheHLayout::new(TheId::named("Tool Params"));
        toolbar_hlayout.set_background_color(None);
        toolbar_hlayout.set_margin(Vec4::new(10, 2, 5, 2));

        let mut toolbar_canvas = TheCanvas::default();
        toolbar_canvas.set_widget(TheTraybar::new(TheId::empty()));
        toolbar_canvas.set_layout(toolbar_hlayout);

        top_canvas.bottom_is_expanding = true;
        top_canvas.set_bottom(toolbar_canvas);

        // Nodes View
        let mut bottom_canvas = TheCanvas::new();

        let mut shared_layout = TheSharedHLayout::new(TheId::named("Shared Panel Layout"));
        shared_layout.set_shared_ratio(0.8);
        shared_layout.set_mode(TheSharedHLayoutMode::Shared);

        let mut node_canvas = TheCanvas::new();
        let node_view = TheNodeCanvasView::new(TheId::named("NodeView"));
        node_canvas.set_widget(node_view);

        let mut settings_canvas = TheCanvas::new();
        let mut text_layout = TheTextLayout::new(TheId::named("Node Settings"));
        // text_layout.limiter_mut().set_max_width(self.width);
        text_layout.set_text_margin(20);
        text_layout.set_text_align(TheHorizontalAlign::Right);
        settings_canvas.set_layout(text_layout);

        shared_layout.add_canvas(node_canvas);
        shared_layout.add_canvas(settings_canvas);
        bottom_canvas.set_layout(shared_layout);

        //

        let mut vsplitlayout = TheSharedVLayout::new(TheId::named("Shared VLayout"));
        vsplitlayout.add_canvas(top_canvas);
        vsplitlayout.add_canvas(bottom_canvas);
        vsplitlayout.set_shared_ratio(0.65);
        vsplitlayout.set_mode(TheSharedVLayoutMode::Shared);

        let mut shared_canvas = TheCanvas::new();
        shared_canvas.set_layout(vsplitlayout);

        // Mode List
        let mut tool_list_canvas: TheCanvas = TheCanvas::new();

        let mut tool_list_bar_canvas = TheCanvas::new();
        let mut tool_list_bar = TheToolListBar::new(TheId::empty());
        tool_list_bar.set_value(TheValue::Text("MODE".into()));

        tool_list_bar_canvas.set_widget(tool_list_bar);
        tool_list_canvas.set_top(tool_list_bar_canvas);

        let mut v_tool_list_layout = TheVLayout::new(TheId::named("Tool List Layout"));
        v_tool_list_layout.limiter_mut().set_max_width(51);
        v_tool_list_layout.set_margin(Vec4::new(2, 2, 2, 2));
        v_tool_list_layout.set_padding(1);

        let mut b = TheToolListButton::new(TheId::named("Palette Mode"));
        b.set_icon_name("move".into());
        b.set_status_text("Palette Mode.");
        b.set_state(TheWidgetState::Selected);
        v_tool_list_layout.add_widget(Box::new(b));

        let mut b = TheToolListButton::new(TheId::named("Point Mode"));
        b.set_icon_name("move".into());
        b.set_status_text("Point mode.");
        v_tool_list_layout.add_widget(Box::new(b));

        let mut b = TheToolListButton::new(TheId::named("History Mode"));
        b.set_icon_name("move".into());
        b.set_status_text("History mode.");
        v_tool_list_layout.add_widget(Box::new(b));

        let mut tool_list_bar = TheToolListBar::new(TheId::empty());
        tool_list_bar.set_value(TheValue::Text("TOOL".into()));
        v_tool_list_layout.add_widget(Box::new(tool_list_bar));

        TOOLLIST
            .write()
            .unwrap()
            .add_tools(&mut v_tool_list_layout, ctx);

        tool_list_canvas.set_layout(v_tool_list_layout);

        let mut tool_list_border_canvas = TheCanvas::new();
        let mut border_widget = TheIconView::new(TheId::empty());
        border_widget.set_border_color(Some([82, 82, 82, 255]));
        border_widget.limiter_mut().set_max_width(1);
        border_widget.limiter_mut().set_max_height(i32::MAX);
        tool_list_border_canvas.set_widget(border_widget);

        tool_list_canvas.set_right(tool_list_border_canvas);

        shared_canvas.set_left(tool_list_canvas);

        //

        ui.canvas.set_center(shared_canvas);

        let mut status_canvas = TheCanvas::new();
        let mut statusbar = TheStatusbar::new(TheId::named("Statusbar"));
        statusbar.set_text("Welcome to Shape-Z".to_string());
        status_canvas.set_widget(statusbar);

        ui.canvas.set_bottom(status_canvas);

        //

        ctx.ui.set_disabled("Save");
        ctx.ui.set_disabled("Save As");
        ctx.ui.set_disabled("Undo");
        ctx.ui.set_disabled("Redo");

        self.event_receiver = Some(ui.add_state_listener("Main Receiver".into()));
    }

    /// Set the command line arguments
    fn set_cmd_line_args(&mut self, args: Vec<String>, ctx: &mut TheContext) {
        if args.len() > 1 {
            #[allow(irrefutable_let_patterns)]
            if let Ok(path) = PathBuf::from_str(&args[1]) {
                ctx.ui.send(TheEvent::FileRequesterResult(
                    TheId::named("Open"),
                    vec![path],
                ));
            }
        }
    }

    /// Handle UI events and UI state
    fn update_ui(&mut self, ui: &mut TheUI, ctx: &mut TheContext) -> bool {
        let mut redraw = false;

        if let Some(receiver) = &mut self.event_receiver {
            while let Ok(event) = receiver.try_recv() {
                if TOOLLIST
                    .write()
                    .unwrap()
                    .handle_event(&event, ui, ctx, &mut self.context)
                {
                    redraw = true;
                }
                if MODELEDITOR
                    .write()
                    .unwrap()
                    .handle_event(&event, ui, ctx, &mut self.context)
                {
                    redraw = true;
                }
                if NODEEDITOR
                    .write()
                    .unwrap()
                    .handle_event(&event, ui, ctx, &mut self.context)
                {
                    redraw = true;
                }
                #[allow(clippy::single_match)]
                match &event {
                    TheEvent::Custom(id, _) => {
                        if id.name == "Startup" {
                            crate::utils::update_palette_ui(ui, ctx);
                            let mut toollist = TOOLLIST.write().unwrap();
                            let id = toollist.tools[0].id().uuid;
                            toollist.set_tool(id, ui, ctx, &mut self.context);
                            ctx.ui.send(TheEvent::PaletteIndexChanged(
                                TheId::named("PalettePicker"),
                                0,
                            ));
                        }
                    }
                    TheEvent::PaletteIndexChanged(_, index) => {
                        self.context.palette_index = *index as u8;
                    }
                    TheEvent::StateChanged(id, _) => {
                        if id.name == "Palette Mode" {
                            self.context.mode = ToolMode::Palette;
                            ctx.ui
                                .set_widget_state("Point Mode".into(), TheWidgetState::None);
                            ctx.ui
                                .set_widget_state("History Mode".into(), TheWidgetState::None);
                        } else if id.name == "Point Mode" {
                            self.context.mode = ToolMode::Point;
                            ctx.ui
                                .set_widget_state("Palette Mode".into(), TheWidgetState::None);
                            ctx.ui
                                .set_widget_state("History Mode".into(), TheWidgetState::None);
                        } else if id.name == "History Mode" {
                            self.context.mode = ToolMode::History;
                            ctx.ui
                                .set_widget_state("Palette Mode".into(), TheWidgetState::None);
                            ctx.ui
                                .set_widget_state("Point Mode".into(), TheWidgetState::None);
                        }

                        if id.name == "Undo" {
                            if ui.focus_widget_supports_undo_redo(ctx) {
                                if id.name == "Undo" {
                                    ui.undo(ctx);
                                }
                            } else {
                                let mut manager = UNDOMANAGER.write().unwrap();
                                manager.undo(ui, ctx, &mut self.context);
                            }
                        } else if id.name == "Redo" {
                            if ui.focus_widget_supports_undo_redo(ctx) {
                                ui.redo(ctx);
                            } else {
                                let mut manager = UNDOMANAGER.write().unwrap();
                                manager.redo(ui, ctx, &mut self.context);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // Check for redraw (30fps) and tick updates
        let (redraw_update, _tick_update) = self.update_tracker.update((1000 / 30) as u64, 250_u64);

        if redraw_update {
            MODELEDITOR.write().unwrap().draw(ui);

            redraw = true;
        }
        redraw
    }
}
