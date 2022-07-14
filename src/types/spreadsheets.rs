pub struct Spreadsheets {
    spreadsheet_id: string,
    properties: SpreadsheetProperties,
    sheets: Vec<Sheet>,
    namedranges: Vec<NamedRange>,
    spreadsheet_url: String,
    developer_metadata: Vec<DeveloperMetadata>,
    datasource: Vec<DataSource>,
    datasource_schedules: Vec<DataSourceRefreshSchedule>,
}

pub struct SpreadsheetProperties {
    title: String,
    locale: String,
    auto_recalc: RecalculationInterval,
    timezone: string,
    default_format: CellFormat,
    iterative_calculation_settings: IterativeCalculationSettings,
    spreadsheet_theme: SpreadsheetTheme,
}

enum RecalculationInterval {
    RECALCULATION_INTERVAL_UNSPECIFIED = 0,
    ON_CHANGE = 1,
    MINUTE = 2,
    HOUR = 3,
}

pub struct IterativeCalculationSettings {
    max_iterations: i64,
    convergence_threshold: f64,
}

pub struct SpreadsheetTheme {
    primary_font_family: string,
    theme_colors: Vec<ThemeColorPair>,
}

pub struct ThemeColorPair {
    color_type: ThemeColorType,
    color: ColorStyle,
}

pub struct NamedRange {
    named_range_id: String,
    name: String,
    range: GridRange,
}

pub struct DataSource {
    datasource_id: String,
    spec: DataSourceSpec,
    calculated_columns: Vec<DataSourceColumn>,
    sheet_id: i64,
}

pub struct DataSourceSpec {
    parameters: Vec<DataSourceParameter>,
    big_query: BigQueryDataSourceSpec,
}

pub struct BigQueryDataSourceSpec {
    project_id: String,
    query_spec: BigQueryQuerySpec,
    table_spec: BigQueryTableSpec,
}
pub struct BigQueryQuerySpec {
    raw_query: String,
}

pub struct BigQueryTableSpec {
    table_project_id: String,
    table_id: String,
    dataset_id: String,
}

pub struct DataSourceParameter {
    name: String,
    named_range_id: String,
    range: GridRange,
}

pub struct DataSourceRefreshSchedule<T>
where
    T: ScheduleConfig,
{
    enabled: bool,
    refresh_scope: DataSourceRefreshScope,
    next_run: Interval,
    // union field scheduling_config can be only one of the following...
    schedule: T, // dailySchedule, weeklySchedule, monthlySchedule
}

pub enum DataSourceRefreshScope {
    DATA_SOURCE_REFRESH_SCOPE_UNSPECIFIED = 0,
    ALL_DATA_SOURCE = 1,
}

pub struct DataSourceRefreshDailySchedule {
    start_time: TimeOfDay,
}
pub struct TimeOfDay {
    hours: i32,
    minutes: i32,
    seconds: i32,
    nanos: i32,
}

pub struct DataSourceRefreshWeeklySchedule {
    start_tile: TimeOfDay,
    days_of_week: DayOfWeek,
}

pub enum DayOfWeek {
    DAY_OF_WEEK_UNSPECIFIED = -1,
    MONDAY = 0,
    TUESDAY = 1,
    WEDNESDAY = 2,
    THURSDAY = 3,
    FRIDAY = 4,
    SATURDAY = 5,
    SUNDAY = 7,
}

pub struct DataSourceRefreshMonthlySchedule {
    start_time: TimeOfDay,
    days_of_month: i32,
}

pub struct Interval {
    start_time: String,
    end_time: String,
}
