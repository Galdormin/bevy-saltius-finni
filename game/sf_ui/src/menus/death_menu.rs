//! Death menu with drag-and-drop gene selection

use bevy::{picking::Pickable, prelude::*, ui::FocusPolicy, ui::Val::*};

use sf_events::RespawnEvent;
use sf_gene::PlayerGenes;

use crate::{
    states::Menu,
    ui::{palette, theme::UiTheme, widget},
};

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Death), spawn_death_menu);
    app.add_systems(
        Update,
        (
            update_drag_icon_position,
            update_borders,
            update_hover_info,
            update_grid_visuals,
            update_slot_visuals,
        )
            .run_if(in_state(Menu::Death)),
    );
    app.add_systems(OnExit(Menu::Death), cleanup_resources);
}

// Components

/// A gene slot in the gene grid.
#[derive(Component)]
struct GeneSlot(usize);

/// An active gene slot.
#[derive(Component)]
struct ActiveSlot(Option<usize>);

/// Floating drag icon that follows the cursor.
#[derive(Component)]
struct DragGene;

/// Text showing the hovered gene's name.
#[derive(Component)]
struct HoverGeneName;

/// Text showing the hovered gene's description.
#[derive(Component)]
struct HoverGeneDesc;

// Resources

#[derive(Resource)]
struct DragState {
    gene_id: usize,
    source: DragSource,
    /// Set to `true` by `DragDrop` so `DragEnd` knows the drop was handled.
    dropped: bool,
}

#[derive(PartialEq, Clone, Copy)]
enum DragSource {
    Grid,
    Slot(Entity),
}

/// Pre-spawned floating icon — shown/hidden via `Display`.
#[derive(Resource)]
struct DragIconEntity(Entity);

/// Currently hovered gene for the info panel.
#[derive(Resource, Default)]
struct HoveredGene(Option<usize>);

// Style

const CONTAINER_BG: Color = Color::srgba(0.12, 0.12, 0.14, 0.8);
const CONTAINER_BORDER: Color = Color::srgb(0.30, 0.30, 0.35);

const GENE_SIZE: f32 = 20.0;
const GENE_COLOR: Color = Color::srgb(0.35, 0.55, 0.75);
const GENE_ACTIVE_COLOR: Color = Color::srgb(0.22, 0.22, 0.25);
const GENE_BORDER: Color = Color::srgb(0.40, 0.40, 0.40);
const GENE_HOVER_BORDER: Color = Color::srgb(0.90, 0.90, 0.90);

const SLOT_EMPTY_COLOR: Color = Color::srgb(0.18, 0.18, 0.20);
const SLOT_DROP_HIGHLIGHT: Color = Color::srgb(0.4, 0.9, 0.4);

// Setup

fn spawn_death_menu(mut commands: Commands, player_genes: Res<PlayerGenes>) {
    commands.init_resource::<HoveredGene>();

    let root = commands
        .spawn((
            widget::ui_root("Death Menu"),
            GlobalZIndex(2),
            DespawnOnExit(Menu::Death),
        ))
        .id();

    commands.spawn((widget::header("You have died!"), ChildOf(root)));

    // Three-column layout
    let content = commands
        .spawn((
            Name::new("Gene Content"),
            ChildOf(root),
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                height: Percent(75.0),
                column_gap: Px(12.0),
                padding: UiRect::all(Px(8.0)),
                align_items: AlignItems::Stretch,
                ..default()
            },
        ))
        .id();

    // Left: Gene Grid
    let grid = commands
        .spawn((
            Name::new("Gene Grid"),
            ChildOf(content),
            Node {
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                grid_template_rows: RepeatedGridTrack::flex(4, 1.0),
                width: Px(102.0),
                padding: UiRect::all(Px(4.0)),
                row_gap: Px(4.0),
                column_gap: Px(4.0),
                align_content: AlignContent::Center,
                border: UiRect::all(Px(1.0)),
                ..default()
            },
            BackgroundColor(CONTAINER_BG),
            BorderColor::from(CONTAINER_BORDER),
        ))
        .id();

    let mut known = player_genes.known_genes();
    known.sort_by_key(|g| g.id);

    for gene in &known {
        let is_active = player_genes
            .active_genes()
            .iter()
            .any(|ag| ag.id == gene.id);

        let bg = if is_active {
            GENE_ACTIVE_COLOR
        } else {
            GENE_COLOR
        };

        let square = commands
            .spawn((
                Name::new(format!("Gene {}", gene.name)),
                ChildOf(grid),
                GeneSlot(gene.id),
                Interaction::default(),
                Node {
                    width: Px(GENE_SIZE),
                    height: Px(GENE_SIZE),
                    border: UiRect::all(Px(1.0)),
                    ..default()
                },
                BackgroundColor(bg),
                BorderColor::from(GENE_BORDER),
            ))
            .id();

        commands
            .entity(square)
            .observe(on_gene_drag_start(gene.id))
            .observe(on_gene_drag_end(gene.id));
    }

    // Middle: Info Panel
    let info = commands
        .spawn((
            Name::new("Info Panel"),
            ChildOf(content),
            Node {
                flex_direction: FlexDirection::Column,
                width: Px(80.0),
                padding: UiRect::all(Px(8.0)),
                row_gap: Px(4.0),
                border: UiRect::all(Px(1.0)),
                ..default()
            },
            BackgroundColor(CONTAINER_BG),
            BorderColor::from(CONTAINER_BORDER),
            Pickable::IGNORE,
        ))
        .id();

    commands.spawn((
        ChildOf(info),
        HoverGeneName,
        Text::new(""),
        UiTheme::PIXEL_ART,
        TextFont::from_font_size(10.0),
        TextColor(palette::HEADER_TEXT),
        Pickable::IGNORE,
    ));

    commands.spawn((
        ChildOf(info),
        HoverGeneDesc,
        Text::new(""),
        UiTheme::PIXEL_ART,
        TextFont::from_font_size(8.0),
        TextColor(palette::LABEL_TEXT),
        Pickable::IGNORE,
    ));

    // Right: Active Slots
    let slots_panel = commands
        .spawn((
            Name::new("Active Slots"),
            ChildOf(content),
            Node {
                flex_direction: FlexDirection::Column,
                width: Px(40.0),
                padding: UiRect::all(Px(6.0)),
                row_gap: Px(4.0),
                align_items: AlignItems::Center,
                border: UiRect::all(Px(1.0)),
                ..default()
            },
            BackgroundColor(CONTAINER_BG),
            BorderColor::from(CONTAINER_BORDER),
        ))
        .id();

    commands.spawn((
        ChildOf(slots_panel),
        Text::new("Slots"),
        UiTheme::PIXEL_ART,
        TextFont::from_font_size(9.0),
        TextColor(palette::LABEL_TEXT),
        Pickable::IGNORE,
    ));

    let active = player_genes.active_genes();
    let total_slots = active.len() + player_genes.remaining_gene_slot();
    for i in 0..total_slots {
        let gene_id = active.get(i).map(|g| g.id);
        spawn_active_slot(&mut commands, slots_panel, gene_id);
    }

    // ── Drag Icon (hidden by default) ────────────────────────────────────
    let drag_icon = commands
        .spawn((
            DragGene,
            ChildOf(root),
            Node {
                display: Display::None,
                position_type: PositionType::Absolute,
                width: Px(GENE_SIZE),
                height: Px(GENE_SIZE),
                left: Px(0.0),
                top: Px(0.0),
                ..default()
            },
            BackgroundColor(GENE_COLOR),
            GlobalZIndex(200),
            Pickable::IGNORE,
            FocusPolicy::Pass,
        ))
        .id();
    commands.insert_resource(DragIconEntity(drag_icon));

    // Respawn button
    commands.spawn((ChildOf(root), widget::button("Respawn", respawn_on_click)));
}

fn spawn_active_slot(commands: &mut Commands, parent: Entity, gene_id: Option<usize>) {
    let bg = if gene_id.is_some() {
        GENE_ACTIVE_COLOR
    } else {
        SLOT_EMPTY_COLOR
    };

    let slot = commands
        .spawn((
            Name::new("Active Slot"),
            ChildOf(parent),
            ActiveSlot(gene_id),
            Interaction::default(),
            Node {
                width: Px(GENE_SIZE),
                height: Px(GENE_SIZE),
                border: UiRect::all(Px(1.0)),
                ..default()
            },
            BackgroundColor(bg),
            BorderColor::from(GENE_BORDER),
        ))
        .id();

    commands
        .entity(slot)
        .observe(on_slot_drag_start(slot))
        .observe(on_slot_drag_end(slot))
        .observe(on_slot_drag_drop(slot))
        .observe(on_slot_right_click(slot));
}

// Observers

fn on_gene_drag_start(
    gene_id: usize,
) -> impl Fn(
    On<Pointer<DragStart>>,
    Option<Res<DragState>>,
    Res<PlayerGenes>,
    Option<Res<DragIconEntity>>,
    Query<&mut Node, With<DragGene>>,
    Commands,
) {
    move |_, drag, player_genes, icon, mut icon_nodes, mut commands| {
        if drag.is_some() {
            return;
        }

        // Can't drag genes that are already active
        if player_genes.active_genes().iter().any(|g| g.id == gene_id) {
            return;
        }

        if let Some(ref icon) = icon
            && let Ok(mut node) = icon_nodes.get_mut(icon.0)
        {
            node.display = Display::Flex;
        }

        commands.insert_resource(DragState {
            gene_id,
            source: DragSource::Grid,
            dropped: false,
        });
    }
}

fn on_gene_drag_end(
    gene_id: usize,
) -> impl Fn(
    On<Pointer<DragEnd>>,
    Option<Res<DragState>>,
    Option<Res<DragIconEntity>>,
    Query<&mut Node, With<DragGene>>,
    Commands,
) {
    move |_, drag, icon, mut icon_nodes, mut commands| {
        let Some(drag_state) = drag else { return };
        if drag_state.gene_id != gene_id || drag_state.source != DragSource::Grid {
            return;
        }

        if let Some(ref icon) = icon
            && let Ok(mut node) = icon_nodes.get_mut(icon.0)
        {
            node.display = Display::None;
        }

        commands.remove_resource::<DragState>();
    }
}

// ── Active Slot Observers ──────────────────────────────────────────────────

fn on_slot_drag_start(
    slot_entity: Entity,
) -> impl Fn(
    On<Pointer<DragStart>>,
    Option<Res<DragState>>,
    Query<&mut ActiveSlot>,
    Option<Res<DragIconEntity>>,
    Query<&mut Node, With<DragGene>>,
    Commands,
) {
    move |_, drag, mut slots, icon, mut icon_nodes, mut commands| {
        if drag.is_some() {
            return;
        }
        let Ok(mut slot) = slots.get_mut(slot_entity) else {
            return;
        };
        let Some(gene_id) = slot.0.take() else {
            return;
        };

        if let Some(ref icon) = icon
            && let Ok(mut node) = icon_nodes.get_mut(icon.0)
        {
            node.display = Display::Flex;
        }

        commands.insert_resource(DragState {
            gene_id,
            source: DragSource::Slot(slot_entity),
            dropped: false,
        });
    }
}

fn on_slot_drag_end(
    slot_entity: Entity,
) -> impl Fn(
    On<Pointer<DragEnd>>,
    Option<Res<DragState>>,
    Option<Res<DragIconEntity>>,
    Query<&mut Node, With<DragGene>>,
    ResMut<PlayerGenes>,
    Commands,
) {
    move |_, drag, icon, mut icon_nodes, mut player_genes, mut commands| {
        let Some(drag_state) = drag else { return };
        if drag_state.source != DragSource::Slot(slot_entity) {
            return;
        }

        // Dropped in void → deactivate the gene
        if !drag_state.dropped {
            player_genes.remove_active_gene(drag_state.gene_id);
        }

        if let Some(ref icon) = icon
            && let Ok(mut node) = icon_nodes.get_mut(icon.0)
        {
            node.display = Display::None;
        }

        commands.remove_resource::<DragState>();
    }
}

/// Fires on the **target** slot when a dragged entity is released over it.
fn on_slot_drag_drop(
    slot_entity: Entity,
) -> impl Fn(On<Pointer<DragDrop>>, Option<ResMut<DragState>>, Query<&mut ActiveSlot>, ResMut<PlayerGenes>)
{
    move |_, drag, mut slots, mut player_genes| {
        let Some(mut drag_state) = drag else { return };

        // Place gene in target slot; retrieve evicted gene if any.
        let evicted = {
            let Ok(mut slot) = slots.get_mut(slot_entity) else {
                return;
            };
            slot.0.replace(drag_state.gene_id)
        };

        match drag_state.source {
            DragSource::Grid => {
                // Remove evicted first so `add_active_gene` has capacity.
                if let Some(evicted_id) = evicted {
                    player_genes.remove_active_gene(evicted_id);
                }
                player_genes.add_active_gene(drag_state.gene_id);
            }
            DragSource::Slot(source_entity) => {
                // Slot → Slot swap: return evicted gene to source slot.
                if let Some(evicted_id) = evicted
                    && let Ok(mut source) = slots.get_mut(source_entity)
                {
                    source.0 = Some(evicted_id);
                }
                // Both genes stay active — no `PlayerGenes` change.
            }
        }

        drag_state.dropped = true;
    }
}

/// Right-click on an active slot deactivates the gene.
fn on_slot_right_click(
    slot_entity: Entity,
) -> impl Fn(On<Pointer<Click>>, Query<&mut ActiveSlot>, ResMut<PlayerGenes>) {
    move |event, mut slots, mut player_genes| {
        if event.button != PointerButton::Secondary {
            return;
        }
        let Ok(mut slot) = slots.get_mut(slot_entity) else {
            return;
        };
        if let Some(gene_id) = slot.0.take() {
            player_genes.remove_active_gene(gene_id);
        }
    }
}

// Systems

fn update_drag_icon_position(
    drag: Option<Res<DragState>>,
    icon: Option<Res<DragIconEntity>>,
    windows: Query<&Window>,
    ui_scale: Res<UiScale>,
    mut nodes: Query<&mut Node, With<DragGene>>,
) {
    let (Some(_), Some(icon)) = (drag, icon) else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };
    let Some(position) = window.cursor_position() else {
        return;
    };
    let scale = ui_scale.0;
    if let Ok(mut node) = nodes.get_mut(icon.0) {
        let half = GENE_SIZE / 2.0;
        node.left = Px(position.x / scale - half);
        node.top = Px(position.y / scale - half);
    }
}

/// Highlights borders on hover and updates the `HoveredGene` resource.
fn update_borders(
    drag: Option<Res<DragState>>,
    mut genes: Query<(&GeneSlot, &Interaction, &mut BorderColor)>,
    mut slots: Query<(&ActiveSlot, &Interaction, &mut BorderColor), Without<GeneSlot>>,
    mut hovered: ResMut<HoveredGene>,
) {
    let is_dragging = drag.is_some();
    let mut new_hover: Option<usize> = None;

    for (gene, interaction, mut bc) in &mut genes {
        let target: BorderColor = match interaction {
            Interaction::Hovered | Interaction::Pressed if !is_dragging => {
                new_hover = Some(gene.0);
                GENE_HOVER_BORDER.into()
            }
            _ => GENE_BORDER.into(),
        };
        *bc = target;
    }

    for (slot, interaction, mut bc) in &mut slots {
        let target: BorderColor = match interaction {
            Interaction::Hovered | Interaction::Pressed if is_dragging => {
                SLOT_DROP_HIGHLIGHT.into()
            }
            Interaction::Hovered | Interaction::Pressed => {
                new_hover = slot.0;
                GENE_HOVER_BORDER.into()
            }
            _ => GENE_BORDER.into(),
        };
        *bc = target;
    }

    if hovered.0 != new_hover {
        hovered.0 = new_hover;
    }
}

/// Updates the info panel when the hovered gene changes.
fn update_hover_info(
    hovered: Res<HoveredGene>,
    player_genes: Res<PlayerGenes>,
    mut name_text: Query<&mut Text, With<HoverGeneName>>,
    mut desc_text: Query<&mut Text, (With<HoverGeneDesc>, Without<HoverGeneName>)>,
) {
    if !hovered.is_changed() {
        return;
    }

    let (name, desc) = match hovered.0 {
        Some(id) => player_genes
            .known_genes()
            .iter()
            .find(|g| g.id == id)
            .map(|g| (g.name.clone(), g.description()))
            .unwrap_or_default(),
        None => (String::new(), String::new()),
    };

    if let Ok(mut text) = name_text.single_mut() {
        text.0 = name;
    }
    if let Ok(mut text) = desc_text.single_mut() {
        text.0 = desc;
    }
}

/// Grays out active genes in the left grid.
fn update_grid_visuals(
    player_genes: Res<PlayerGenes>,
    mut genes: Query<(&GeneSlot, &mut BackgroundColor)>,
) {
    if !player_genes.is_changed() {
        return;
    }

    let active_ids: Vec<usize> = player_genes.active_genes().iter().map(|g| g.id).collect();

    for (gene, mut bg) in &mut genes {
        let color = if active_ids.contains(&gene.0) {
            GENE_ACTIVE_COLOR
        } else {
            GENE_COLOR
        };
        if bg.0 != color {
            bg.0 = color;
        }
    }
}

/// Syncs slot background when content changes.
fn update_slot_visuals(mut slots: Query<(&ActiveSlot, &mut BackgroundColor), Changed<ActiveSlot>>) {
    for (slot, mut bg) in &mut slots {
        bg.0 = if slot.0.is_some() {
            GENE_COLOR
        } else {
            SLOT_EMPTY_COLOR
        };
    }
}

// Helpers

fn cleanup_resources(mut commands: Commands) {
    commands.remove_resource::<DragState>();
    commands.remove_resource::<DragIconEntity>();
    commands.remove_resource::<HoveredGene>();
}

fn respawn_on_click(
    _: On<Pointer<Click>>,
    mut events: MessageWriter<RespawnEvent>,
    mut next_menu: ResMut<NextState<Menu>>,
) {
    events.write(RespawnEvent);
    next_menu.set(Menu::None);
}
