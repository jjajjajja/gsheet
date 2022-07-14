pub struct CellData {
    user_entered_value: ExtendValue,
    effective_value: ExtendedValue,
    formatted_value: String,
    user_entered_format: CellFormat,
    effective_format: CellFormat,
    hyperlink: String,
    note: String,
    text_format_runs: TextFormatRun,
    data_validation: DataValidatoinRule,
    pivot_table: PivotTable,
    data_source_table: DataSourceTable,
    data_source_formula: DataSourceFomula,
}

pub struct CellFormat {
    number_format: NumberFormat,
    #[deprecated]
    background_color: Color,
    background_color_style: ColorStyle,
    borders: Borders,
    padding: Padding,
    horizontal_alignment: HorizontalAlign,
    vertical_alignment: VerticalAlign,
    wrap_strategy: WrapStrategy,
    text_direction: TextDirection,
    text_format: TextFormat,
    hyperlink_display_type: HyperlinkDisplayType,
    text_rotation: TextRotation,
}

pub struct NumberFormat {
    number_format_type: NumberFormatType,
    pattern: String,
}

enum NumberFormatType {
    NUMBER_FORMAT_TYPE_UNSPECIFIED,
    TEXT,
    NUMBER,
    PERCENT,
    CURRENCY,
    DATE,
    TIME,
    DATE_TIME,
    SCIENTIFIC,
}

pub struct Borders {
    top: Border,
    bottom: Border,
    left: Border,
    right: Border,
}

pub struct Border {
    style: Style,
    #[deprecated]
    width: i64,
    #[deprecated]
    color: Color,
    color_style: ColorStyle,
}

enum Style {
    STYLE_UNSPECIFIED,
    DOTTED,
    DASHED,
    SOLID,
    SOLID_MEDIUM,
    SOLID_THICK,
    NONE,
    DOUBLE,
}

pub struct Padding {
    top: i64,
    right: i64,
    bottom: i64,
    left: i64,
}

pub enum VerticalAlign {
    VERTICAL_ALIGN_UNSPECIFIED,
    TOP,
    MIDDLE,
    BOTTOM,
}

pub enum WrapStrategy {
    WRAP_STRATEGY_UNSPECIFIED,
    OVERFLOW_CELL,
    LEGACY_MAP,
    CLIP,
    WRAP,
}

pub enum TextDirection {
    TEXT_DIRECTION_UNSPECIFIED,
    LEFT_TO_RIGHT,
    RIGHT_TO_LEFT,
}

pub enum HyperlinkDisplayType {
    HYPERLINK_DISPLAY_TYPE_UNSPECIFIED,
    LINKED,
    PLAIN_TEXT,
}

pub struct TextRotation<T>
where
    T: i64,
    T: bool,
{
    // angle if T is integer
    // vertical if T is boolean
    value: T,
}

pub struct TextFormatRun {
    start_index: i64,
    format: TextFormat,
}
pub struct DataValidationRule {
    condition: BooleanCondition,
    input_message: String,
    strict: bool,
    show_custom_ui: bool,
}

pub struct DataSourceTable {
    data_source_id: String,
    column_selection_type: DataSourceTableColumnSelectionType,
    columns: Vec<DataSourceColumnReference>,
    filter_specs: Vec<FilterSpec>,
    sort_specs: Vec<SortSpec>,
    row_limit: i64,
    data_execution_status: DataExecutionStatus,
}

pub struct DataSourceFormat {
    data_source_id: String,
    data_execution_status: DataExecutionStatus,
}

pub enum DataSourceTableColumnSelectionType {
    DATA_SOURCE_TABLE_COLUMN_SELECTION_TYPE_UNSEPCIFIED,
    SELECTED,
    SYNC_ALL,
}

pub struct DataSourceFomula {
    data_source_id: String,
    data_execution_status: DataExecutionStatus,
}
