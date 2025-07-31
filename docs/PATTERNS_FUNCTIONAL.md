# Functional Programming Patterns in Color-rs

## Overview

This document serves as the **PRIMARY** programming patterns catalog for the color-rs project, emphasizing modern functional programming approaches that leverage Rust's unique capabilities. Unlike traditional object-oriented patterns, functional programming patterns in Rust provide better alignment with the language's ownership model, type system, and performance characteristics.

## Philosophy: Functional First, OOP When Necessary

Modern Rust development should prioritize functional programming patterns that:
- Leverage Rust's powerful type system and ownership model
- Provide composability through mathematical foundations
- Ensure memory safety and performance without runtime overhead
- Avoid the complexity and rigidity of traditional class hierarchies
- Enable powerful abstractions through algebraic data types

## Color-rs Functional Architecture

Color-rs demonstrates functional programming principles through:
- **Immutable Data Structures**: Color representations are immutable by default
- **Pure Functions**: Color transformations and calculations are side-effect free
- **Compositional Design**: Complex operations built from simple, composable functions
- **Type-Driven Development**: Rich type system prevents invalid color operations
- **Functional Error Handling**: Comprehensive use of `Result` and `Option` types

## Pattern Categories

### 1. Optics - Data Access and Manipulation
### 2. Functional Foundations - Core Algebraic Structures  
### 3. Streams - Data Flow and Processing
### 4. State Management - Functional State Handling
### 5. Recursive Schemes - Data Structure Processing
### 6. Architectural Patterns - System-Level Organization
### 7. Testing Approaches - Functional Verification
### 8. Color-Specific Idioms - Domain-Specific Patterns

---

## 1. Optics - Data Access and Manipulation

Optics provide composable and immutable ways to access and update nested data structures.

### Lens Pattern

**Purpose**: Bidirectional access to nested data with functional updates.

**Color-rs Implementation**:
```rust
// Color analysis result with nested structure
pub struct ColorAnalysisResult {
    pub metadata: AnalysisMetadata,
    pub conversion: ConversionData,
    pub contrast: ContrastData,
    pub grayscale: GrayscaleData,
}

// Lens for accessing and updating conversion data
impl ColorAnalysisResult {
    pub fn with_conversion<F>(self, f: F) -> Self 
    where 
        F: FnOnce(ConversionData) -> ConversionData,
    {
        Self {
            conversion: f(self.conversion),
            ..self
        }
    }
    
    pub fn with_contrast<F>(self, f: F) -> Self 
    where 
        F: FnOnce(ContrastData) -> ContrastData,
    {
        Self {
            contrast: f(self.contrast),
            ..self
        }
    }
}

// Usage example: Update RGB values functionally
let updated_result = analysis_result
    .with_conversion(|conv| ConversionData {
        rgb: [255, 0, 0],  // Update to red
        ..conv
    });
```
    pub fn database_host(&self) -> &str {
        &self.database.host
    }
    
    pub fn with_database_host(mut self, host: String) -> Self {
        self.database.host = host;
        self
    }
    
    // Lens combinator for deep updates
    pub fn update_database_host<F>(mut self, f: F) -> Self 
    where 
        F: FnOnce(String) -> String 
    {
### Prism Pattern

**Purpose**: Partial access to sum types (enums) with optional extraction.

**Color-rs Implementation**:
```rust
// Color format enumeration with prism access
#[derive(Debug, Clone)]
pub enum ColorFormat {
    Rgb { r: u8, g: u8, b: u8 },
    Hsl { h: f64, s: f64, l: f64 },
    Lab { l: f64, a: f64, b: f64 },
    Hex(String),
}

impl ColorFormat {
    // Prism for RGB colors
    pub fn as_rgb(&self) -> Option<(u8, u8, u8)> {
        match self {
            ColorFormat::Rgb { r, g, b } => Some((*r, *g, *b)),
            _ => None,
        }
    }
    
    // Prism for LAB colors
    pub fn as_lab(&self) -> Option<(f64, f64, f64)> {
        match self {
            ColorFormat::Lab { l, a, b } => Some((*l, *a, *b)),
            _ => None,
        }
    }
    
    // Prism combinator for color adjustments
    pub fn map_rgb_brightness<F>(self, f: F) -> Self 
    where 
        F: FnOnce(u8, u8, u8) -> (u8, u8, u8)
    {
        match self {
            ColorFormat::Rgb { r, g, b } => {
                let (new_r, new_g, new_b) = f(r, g, b);
                ColorFormat::Rgb { r: new_r, g: new_g, b: new_b }
            }
            color => color,
        }
    }
}

// Usage in color processing pipeline
let adjusted_color = color_format
    .map_rgb_brightness(|r, g, b| {
        let brightness_factor = 1.2;
        (
            ((r as f64 * brightness_factor) as u8).min(255),
            ((g as f64 * brightness_factor) as u8).min(255),
            ((b as f64 * brightness_factor) as u8).min(255),
        )
    });
```

### Traversal Pattern

**Purpose**: Access and modify multiple elements within a structure.

**Color-rs Implementation**:
```rust
// Gradient with multiple color stops
pub struct Gradient {
    pub stops: Vec<GradientStop>,
}

pub struct GradientStop {
    pub position: f64,
    pub color: ColorFormat,
}

impl Gradient {
    // Traversal for all colors in gradient
    pub fn map_colors<F>(self, f: F) -> Self 
    where 
        F: Fn(ColorFormat) -> ColorFormat,
    {
        Self {
            stops: self.stops
                .into_iter()
                .map(|stop| GradientStop {
                    position: stop.position,
                    color: f(stop.color),
                })
                .collect(),
        }
    }
    
    // Traversal with position-aware transformation
    pub fn map_colors_with_position<F>(self, f: F) -> Self 
    where 
        F: Fn(f64, ColorFormat) -> ColorFormat,
    {
        Self {
            stops: self.stops
                .into_iter()
                .map(|stop| GradientStop {
                    position: stop.position,
                    color: f(stop.position, stop.color),
                })
                .collect(),
        }
    }
}

// Usage: Apply color correction across entire gradient
let corrected_gradient = gradient
    .map_colors(|color| apply_gamma_correction(color, 2.2))
    .map_colors_with_position(|pos, color| {
        if pos > 0.5 { darken_color(color, 0.1) } else { color }
    });
```

### Traversal Pattern

**Purpose**: Functional iteration over data structures with optional updates.

```rust
// Example: Processing arrays of market data
use ndarray::Array1;

pub trait Traversable<T> {
    fn traverse<F, U>(self, f: F) -> Option<Vec<U>>
    where
        F: Fn(T) -> Option<U>;
        
    fn traverse_array<F>(self, f: F) -> Option<Array1<f64>>
    where
        F: Fn(T) -> Option<f64>;
}

impl Traversable<f64> for Vec<f64> {
    fn traverse<F, U>(self, f: F) -> Option<Vec<U>>
    where
        F: Fn(f64) -> Option<U>
    {
        self.into_iter().map(f).collect()
    }
    
    fn traverse_array<F>(self, f: F) -> Option<Array1<f64>>
    where
        F: Fn(f64) -> Option<f64>
    {
        let results: Option<Vec<f64>> = self.into_iter().map(f).collect();
        results.map(|vec| Array1::from_vec(vec))
    }
}

// Usage in data validation
let validated_prices = raw_prices
    .traverse(|price| if price > 0.0 { Some(price) } else { None })?;
```

---

## 2. Functional Foundations - Core Algebraic Structures

### Functor Pattern

**Purpose**: Map functions over wrapped values while preserving structure.

```rust
// Example: Result and Option mapping in error handling
pub trait Functor<T> {
    type Wrapped<U>;
    
    fn fmap<U, F>(self, f: F) -> Self::Wrapped<U>
    where
        F: FnOnce(T) -> U;
}

impl<T, E> Functor<T> for Result<T, E> {
    type Wrapped<U> = Result<U, E>;
    
    fn fmap<U, F>(self, f: F) -> Self::Wrapped<U>
    where
        F: FnOnce(T) -> U
    {
        self.map(f)
    }
}

// Usage in market data processing
let processed_data = market_data_result
    .fmap(|data| data.normalize())
    .fmap(|data| data.apply_filters());
```

### Applicative Pattern

**Purpose**: Apply functions wrapped in a context to values in the same context.

```rust
// Example: Validation combining multiple results
pub fn combine_validations<T, U, V, E, F>(
    result1: Result<T, E>,
    result2: Result<U, E>,
    f: F,
) -> Result<V, E>
where
    F: FnOnce(T, U) -> V,
    E: Clone,
{
    match (result1, result2) {
        (Ok(a), Ok(b)) => Ok(f(a, b)),
        (Err(e), _) | (_, Err(e)) => Err(e),
    }
}

// Usage in configuration validation
let valid_config = combine_validations(
    validate_database_config(&config.database),
    validate_server_config(&config.server),
    |db, server| ValidatedConfig { database: db, server }
);
```

### Monad Pattern

**Purpose**: Sequential computation with automatic error handling and context preservation.

```rust
// Example: Chaining operations that might fail
pub trait Monad<T>: Functor<T> {
    fn unit(value: T) -> Self;
    fn bind<U, F>(self, f: F) -> Self::Wrapped<U>
    where
        F: FnOnce(T) -> Self::Wrapped<U>;
}

impl<T, E> Monad<T> for Result<T, E> {
    fn unit(value: T) -> Self {
        Ok(value)
    }
    
    fn bind<U, F>(self, f: F) -> Self::Wrapped<U>
    where
        F: FnOnce(T) -> Result<U, E>
    {
        self.and_then(f)
    }
}

// Usage in trading pipeline
let trading_result = fetch_market_data(symbol)
    .bind(|data| validate_data(data))
    .bind(|valid_data| analyze_signals(valid_data))
    .bind(|signals| execute_trades(signals));
```

### Semigroup and Monoid Patterns

**Purpose**: Composable data combination with identity elements.

```rust
// Example: Combining trading metrics
pub trait Semigroup {
    fn combine(self, other: Self) -> Self;
}

pub trait Monoid: Semigroup {
    fn identity() -> Self;
}

#[derive(Debug, Clone)]
pub struct TradingMetrics {
    pub total_return: f64,
    pub trade_count: usize,
    pub max_drawdown: f64,
}

impl Semigroup for TradingMetrics {
    fn combine(self, other: Self) -> Self {
        TradingMetrics {
            total_return: self.total_return + other.total_return,
            trade_count: self.trade_count + other.trade_count,
            max_drawdown: self.max_drawdown.max(other.max_drawdown),
        }
    }
}

impl Monoid for TradingMetrics {
    fn identity() -> Self {
        TradingMetrics {
            total_return: 0.0,
            trade_count: 0,
            max_drawdown: 0.0,
        }
    }
}

// Usage in metrics aggregation
let combined_metrics = portfolio_strategies
    .into_iter()
    .map(|strategy| strategy.get_metrics())
    .fold(TradingMetrics::identity(), |acc, metrics| acc.combine(metrics));
```

---

## 3. Streams - Data Flow and Processing

### Iterator Pipeline Pattern

**Purpose**: Lazy, composable data processing chains.

```rust
// Example: Processing market data streams
pub fn process_candle_stream<I>(candles: I) -> impl Iterator<Item = ProcessedCandle>
where
    I: Iterator<Item = RawCandle>,
{
    candles
        .filter(|candle| candle.volume > 0.0)
        .map(|candle| normalize_candle(candle))
        .filter_map(|result| result.ok())
        .scan(None, |prev_state, candle| {
            let processed = apply_indicators(candle, prev_state.as_ref());
            *prev_state = Some(processed.clone());
            Some(processed)
        })
}

// Usage with lazy evaluation
let processed_stream = process_candle_stream(raw_candles)
    .take(1000)
    .collect::<Vec<_>>();
```

### Functional Reactive Programming (FRP) Pattern

**Purpose**: Time-varying values and event streams with functional composition.

```rust
use tokio::sync::broadcast;

// Example: Real-time signal processing
pub struct Signal<T> {
    receiver: broadcast::Receiver<T>,
}

impl<T: Clone> Signal<T> {
    pub fn map<U, F>(self, f: F) -> Signal<U>
    where
        F: Fn(T) -> U + Send + 'static,
        U: Clone + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = broadcast::channel(100);
        let mut receiver = self.receiver;
        
        tokio::spawn(async move {
            while let Ok(value) = receiver.recv().await {
                let mapped_value = f(value);
                let _ = tx.send(mapped_value);
            }
        });
        
        Signal { receiver: rx }
    }
    
    pub fn filter<F>(self, predicate: F) -> Signal<T>
    where
        F: Fn(&T) -> bool + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = broadcast::channel(100);
        let mut receiver = self.receiver;
        
        tokio::spawn(async move {
            while let Ok(value) = receiver.recv().await {
                if predicate(&value) {
                    let _ = tx.send(value);
                }
            }
        });
        
        Signal { receiver: rx }
    }
}

// Usage in real-time trading
let buy_signals = price_signal
    .map(|price| calculate_indicators(price))
    .filter(|indicators| indicators.should_buy())
    .map(|indicators| TradingSignal::Buy { 
        confidence: indicators.confidence,
        quantity: indicators.suggested_quantity 
    });
```

---

## 4. State Management - Functional State Handling

### Typestate Pattern

**Purpose**: Compile-time state machine enforcement through types.

```rust
// Example: Trading order lifecycle management
pub struct Order<State> {
    id: String,
    symbol: String,
    quantity: f64,
    price: f64,
    _state: std::marker::PhantomData<State>,
}

pub struct Draft;
pub struct Submitted;
pub struct Executed;
pub struct Cancelled;

impl Order<Draft> {
    pub fn new(symbol: String, quantity: f64, price: f64) -> Self {
        Order {
            id: uuid::Uuid::new_v4().to_string(),
            symbol,
            quantity,
            price,
            _state: std::marker::PhantomData,
        }
    }
    
    pub fn submit(self) -> Result<Order<Submitted>, OrderError> {
        // Validation logic
        Ok(Order {
            id: self.id,
            symbol: self.symbol,
            quantity: self.quantity,
            price: self.price,
            _state: std::marker::PhantomData,
        })
    }
}

impl Order<Submitted> {
    pub fn execute(self) -> Order<Executed> {
        Order {
            id: self.id,
            symbol: self.symbol,
            quantity: self.quantity,
            price: self.price,
            _state: std::marker::PhantomData,
        }
    }
    
    pub fn cancel(self) -> Order<Cancelled> {
        Order {
            id: self.id,
            symbol: self.symbol,
            quantity: self.quantity,
            price: self.price,
            _state: std::marker::PhantomData,
        }
    }
}

// Compile-time enforcement - this won't compile:
// let order = Order::new(...);
// order.execute(); // Error: can't execute draft order
```

### Actor Model Pattern

**Purpose**: Isolated state with message-based communication.

```rust
use tokio::sync::mpsc;

// Example: Portfolio management actor
pub struct PortfolioActor {
    positions: HashMap<String, Position>,
    receiver: mpsc::Receiver<PortfolioMessage>,
}

#[derive(Debug)]
pub enum PortfolioMessage {
    AddPosition { symbol: String, quantity: f64 },
    UpdatePrice { symbol: String, price: f64 },
    GetPosition { symbol: String, respond_to: oneshot::Sender<Option<Position>> },
    GetTotalValue { respond_to: oneshot::Sender<f64> },
}

impl PortfolioActor {
    pub async fn run(mut self) {
        while let Some(message) = self.receiver.recv().await {
            match message {
                PortfolioMessage::AddPosition { symbol, quantity } => {
                    self.add_position(symbol, quantity);
                }
                PortfolioMessage::UpdatePrice { symbol, price } => {
                    self.update_price(symbol, price);
                }
                PortfolioMessage::GetPosition { symbol, respond_to } => {
                    let position = self.positions.get(&symbol).cloned();
                    let _ = respond_to.send(position);
                }
                PortfolioMessage::GetTotalValue { respond_to } => {
                    let total = self.calculate_total_value();
                    let _ = respond_to.send(total);
                }
            }
        }
    }
    
    fn add_position(&mut self, symbol: String, quantity: f64) {
        // Implementation
    }
    
    fn update_price(&mut self, symbol: String, price: f64) {
        // Implementation
    }
    
    fn calculate_total_value(&self) -> f64 {
        // Implementation
        self.positions.values().map(|p| p.value()).sum()
    }
}
```

---

## 5. Recursive Schemes - Data Structure Processing

### Catamorphism (Fold) Pattern

**Purpose**: Systematic deconstruction of recursive data structures.

```rust
// Example: Processing nested configuration structures
#[derive(Debug, Clone)]
pub enum ConfigValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<ConfigValue>),
    Object(HashMap<String, ConfigValue>),
}

impl ConfigValue {
    // Catamorphism: fold over the structure
    pub fn cata<T, F>(&self, alg: &F) -> T
    where
        F: Fn(&ConfigValueF<T>) -> T,
    {
        let mapped = match self {
            ConfigValue::String(s) => ConfigValueF::String(s.clone()),
            ConfigValue::Number(n) => ConfigValueF::Number(*n),
            ConfigValue::Boolean(b) => ConfigValueF::Boolean(*b),
            ConfigValue::Array(arr) => {
                let mapped_arr = arr.iter().map(|v| v.cata(alg)).collect();
                ConfigValueF::Array(mapped_arr)
            }
            ConfigValue::Object(obj) => {
                let mapped_obj = obj.iter()
                    .map(|(k, v)| (k.clone(), v.cata(alg)))
                    .collect();
                ConfigValueF::Object(mapped_obj)
            }
        };
        alg(&mapped)
    }
    
    // Count total nodes in configuration
    pub fn count_nodes(&self) -> usize {
        self.cata(&|config_f| match config_f {
            ConfigValueF::String(_) | ConfigValueF::Number(_) | ConfigValueF::Boolean(_) => 1,
            ConfigValueF::Array(arr) => 1 + arr.iter().sum::<usize>(),
            ConfigValueF::Object(obj) => 1 + obj.values().sum::<usize>(),
        })
    }
}

#[derive(Debug)]
pub enum ConfigValueF<T> {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<T>),
    Object(HashMap<String, T>),
}
```

### Anamorphism (Unfold) Pattern

**Purpose**: Systematic construction of recursive data structures.

```rust
// Example: Generating trading signal sequences
pub fn unfold_signals<F, S>(initial_state: S, next: F) -> impl Iterator<Item = TradingSignal>
where
    F: Fn(S) -> Option<(TradingSignal, S)>,
    S: Clone,
{
    std::iter::successors(Some(initial_state), move |state| {
        next(state.clone()).map(|(_, next_state)| next_state)
    })
    .filter_map(move |state| next(state).map(|(signal, _)| signal))
}

// Usage: Generate signal sequence based on market conditions
let signal_stream = unfold_signals(initial_market_state, |state| {
    let signal = analyze_market(&state);
    let next_state = update_state(state, &signal);
    Some((signal, next_state))
});
```

---

## 6. Architectural Patterns - System-Level Organization

### Hexagonal Architecture Pattern

**Purpose**: Isolate business logic from external dependencies through ports and adapters.

```rust
// Example: Trading system with hexagonal architecture

// Core domain (business logic)
pub trait MarketDataPort {
    fn get_current_price(&self, symbol: &str) -> Result<f64, MarketError>;
    fn get_historical_data(&self, symbol: &str, period: TimePeriod) -> Result<Vec<Candle>, MarketError>;
}

pub trait OrderExecutionPort {
    fn place_order(&self, order: Order) -> Result<OrderId, OrderError>;
    fn cancel_order(&self, order_id: OrderId) -> Result<(), OrderError>;
}

// Business logic (pure, testable)
pub struct TradingService {
    market_data: Box<dyn MarketDataPort>,
    order_execution: Box<dyn OrderExecutionPort>,
}

impl TradingService {
    pub fn execute_strategy(&self, strategy: &dyn TradingStrategy) -> Result<Vec<OrderId>, TradingError> {
        let market_data = self.market_data.get_current_price("AAPL")?;
        let signals = strategy.generate_signals(&market_data)?;
        
        signals
            .into_iter()
            .map(|signal| self.order_execution.place_order(signal.into()))
            .collect()
    }
}

// Adapters (external dependencies)
pub struct TInvestMarketDataAdapter {
    client: TInvestClient,
}

impl MarketDataPort for TInvestMarketDataAdapter {
    fn get_current_price(&self, symbol: &str) -> Result<f64, MarketError> {
        // T-Invest API implementation
        self.client.get_last_price(symbol)
            .map_err(|e| MarketError::ApiError(e))
    }
}
```

### CQRS + Event Sourcing Pattern

**Purpose**: Separate read and write models with event-driven state reconstruction.

```rust
// Example: Portfolio event sourcing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PortfolioEvent {
    PositionAdded { symbol: String, quantity: f64, price: f64, timestamp: DateTime<Utc> },
    PositionSold { symbol: String, quantity: f64, price: f64, timestamp: DateTime<Utc> },
    DividendReceived { symbol: String, amount: f64, timestamp: DateTime<Utc> },
}

pub struct PortfolioAggregate {
    id: String,
    positions: HashMap<String, Position>,
    version: u64,
}

impl PortfolioAggregate {
    pub fn apply_event(&mut self, event: &PortfolioEvent) {
        match event {
            PortfolioEvent::PositionAdded { symbol, quantity, price, .. } => {
                let position = self.positions.entry(symbol.clone()).or_default();
                position.add_shares(*quantity, *price);
            }
            PortfolioEvent::PositionSold { symbol, quantity, price, .. } => {
                if let Some(position) = self.positions.get_mut(symbol) {
                    position.sell_shares(*quantity, *price);
                }
            }
            PortfolioEvent::DividendReceived { symbol, amount, .. } => {
                if let Some(position) = self.positions.get_mut(symbol) {
                    position.add_dividend(*amount);
                }
            }
        }
        self.version += 1;
    }
    
    pub fn from_events(id: String, events: &[PortfolioEvent]) -> Self {
        let mut aggregate = PortfolioAggregate {
            id,
            positions: HashMap::new(),
            version: 0,
        };
        
        for event in events {
            aggregate.apply_event(event);
        }
        
        aggregate
    }
}

// Command handler (writes)
pub struct PortfolioCommandHandler {
    event_store: Box<dyn EventStore>,
}

impl PortfolioCommandHandler {
    pub fn handle_add_position(&self, cmd: AddPositionCommand) -> Result<(), PortfolioError> {
        let events = self.event_store.get_events(&cmd.portfolio_id)?;
        let mut portfolio = PortfolioAggregate::from_events(cmd.portfolio_id.clone(), &events);
        
        // Business logic validation
        let event = PortfolioEvent::PositionAdded {
            symbol: cmd.symbol,
            quantity: cmd.quantity,
            price: cmd.price,
            timestamp: Utc::now(),
        };
        
        portfolio.apply_event(&event);
        self.event_store.append_event(&cmd.portfolio_id, event)?;
        
        Ok(())
    }
}

// Query handler (reads)
pub struct PortfolioQueryHandler {
    read_store: Box<dyn PortfolioReadStore>,
}

impl PortfolioQueryHandler {
    pub fn get_portfolio_summary(&self, portfolio_id: &str) -> Result<PortfolioSummary, PortfolioError> {
        self.read_store.get_summary(portfolio_id)
    }
}
```

---

## 7. Testing Approaches - Functional Verification

### Property-Based Testing Pattern

**Purpose**: Test properties that should hold for all valid inputs rather than specific examples.

```rust
use proptest::prelude::*;

// Example: Testing trading calculation properties
#[cfg(test)]
mod tests {
    use super::*;
    
    proptest! {
        #[test]
        fn portfolio_value_never_negative(
            positions in prop::collection::vec(
                (any::<String>(), 0.0f64..1000.0, 0.0f64..100.0),
                0..10
            )
        ) {
            let portfolio = Portfolio::new();
            for (symbol, quantity, price) in positions {
                portfolio.add_position(symbol, quantity, price);
            }
            
            prop_assert!(portfolio.total_value() >= 0.0);
        }
        
        #[test]
        fn commission_calculation_monotonic(
            quantity in 0.0f64..1000.0,
            price in 0.01f64..1000.0,
            rate in 0.0f64..0.1
        ) {
            let commission1 = calculate_commission(quantity, price, rate);
            let commission2 = calculate_commission(quantity * 2.0, price, rate);
            
            prop_assert!(commission2 >= commission1);
        }
    }
}
```

### Golden/Snapshot Testing Pattern

**Purpose**: Verify complex outputs by comparing against known-good reference results.

```rust
// Example: Testing strategy optimization results
#[cfg(test)]
mod golden_tests {
    use super::*;
    use std::fs;
    
    #[test]
    fn test_sma_strategy_optimization_results() {
        let strategy = SMAStrategy::new();
        let market_data = load_test_data("test-data/AAPL-2023.csv");
        
        let optimization_result = optimize_strategy(&strategy, &market_data);
        let result_json = serde_json::to_string_pretty(&optimization_result).unwrap();
        
        // Compare with golden file
        let golden_path = "tests/golden/sma_optimization_result.json";
        
        if std::env::var("UPDATE_GOLDEN").is_ok() {
            fs::write(golden_path, &result_json).unwrap();
        } else {
            let expected = fs::read_to_string(golden_path).unwrap();
            assert_eq!(result_json, expected, "Optimization result differs from golden file");
        }
    }
}
```

---

## 8. Useful Idioms - Rust-Specific Patterns

### Newtype Pattern

**Purpose**: Create distinct types for better type safety and API clarity.

```rust
// Example: Financial value types with different semantics
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Price(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Quantity(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Commission(f64);

impl Price {
    pub fn new(value: f64) -> Result<Self, ValueError> {
        if value >= 0.0 && value.is_finite() {
            Ok(Price(value))
        } else {
            Err(ValueError::InvalidPrice(value))
        }
    }
    
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl std::ops::Mul<Quantity> for Price {
    type Output = MarketValue;
    
    fn mul(self, quantity: Quantity) -> MarketValue {
        MarketValue(self.0 * quantity.0)
    }
}

// Type-safe calculations
let total_value = price * quantity; // Returns MarketValue, not f64
let commission = total_value * commission_rate; // Type-safe commission calculation
```

### Smart Constructor Pattern

**Purpose**: Ensure data validity at construction time through validation.

```rust
// Example: Validated trading parameters
#[derive(Debug, Clone)]
pub struct TradingParameters {
    short_window: u32,
    long_window: u32,
    stop_loss_percent: f64,
}

impl TradingParameters {
    pub fn new(
        short_window: u32, 
        long_window: u32, 
        stop_loss_percent: f64
    ) -> Result<Self, ValidationError> {
        if short_window == 0 {
            return Err(ValidationError::InvalidShortWindow);
        }
        
        if long_window <= short_window {
            return Err(ValidationError::LongWindowTooSmall);
        }
        
        if stop_loss_percent <= 0.0 || stop_loss_percent >= 1.0 {
            return Err(ValidationError::InvalidStopLossPercent);
        }
        
        Ok(TradingParameters {
            short_window,
            long_window,
            stop_loss_percent,
        })
    }
    
    // Getters provide access to validated data
    pub fn short_window(&self) -> u32 { self.short_window }
    pub fn long_window(&self) -> u32 { self.long_window }
    pub fn stop_loss_percent(&self) -> f64 { self.stop_loss_percent }
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Short window must be greater than 0")]
    InvalidShortWindow,
    #[error("Long window must be greater than short window")]
    LongWindowTooSmall,
    #[error("Stop loss percent must be between 0 and 1")]
    InvalidStopLossPercent,
}
```

### Builder Pattern Through Ownership

**Purpose**: Fluent API construction with compile-time validation.

```rust
// Example: Trading strategy builder
pub struct StrategyBuilder<State = Incomplete> {
    short_period: Option<u32>,
    long_period: Option<u32>,
    risk_tolerance: Option<f64>,
    _state: std::marker::PhantomData<State>,
}

pub struct Incomplete;
pub struct Complete;

impl StrategyBuilder<Incomplete> {
    pub fn new() -> Self {
        StrategyBuilder {
            short_period: None,
            long_period: None,
            risk_tolerance: None,
            _state: std::marker::PhantomData,
        }
    }
    
    pub fn short_period(mut self, period: u32) -> Self {
        self.short_period = Some(period);
        self
    }
    
    pub fn long_period(mut self, period: u32) -> Self {
        self.long_period = Some(period);
        self
    }
    
    pub fn risk_tolerance(self, tolerance: f64) -> StrategyBuilder<Complete> {
        StrategyBuilder {
            short_period: self.short_period,
            long_period: self.long_period,
            risk_tolerance: Some(tolerance),
            _state: std::marker::PhantomData,
        }
    }
}

impl StrategyBuilder<Complete> {
    pub fn build(self) -> Result<TradingStrategy, BuildError> {
        let short = self.short_period.ok_or(BuildError::MissingShortPeriod)?;
        let long = self.long_period.ok_or(BuildError::MissingLongPeriod)?;
        let risk = self.risk_tolerance.ok_or(BuildError::MissingRiskTolerance)?;
        
        if long <= short {
            return Err(BuildError::InvalidPeriods);
        }
        
        Ok(TradingStrategy::new(short, long, risk))
    }
}

// Usage with compile-time enforcement
let strategy = StrategyBuilder::new()
    .short_period(20)
    .long_period(50)
    .risk_tolerance(0.02)  // This transforms to Complete state
    .build()?;             // Only available in Complete state
```

---

## Migration Guidelines from OOP Patterns

When encountering traditional OOP patterns in the codebase, apply these functional alternatives:

### Pattern Migration Map

| OOP Pattern | Functional Alternative | Rust Implementation |
|-------------|----------------------|-------------------|
| Singleton | Module constants / Dependency injection | `static` / `lazy_static!` / DI container |
| Factory Method | Associated functions | `impl Type { pub fn new() -> Self }` |
| Strategy | Function pointers / Closures | `Fn` traits / trait objects |
| Command | Closures / Function objects | `Box<dyn Fn()>` / enum variants |
| Observer | Channels / Event streams | `tokio::sync::watch` / `broadcast` |
| State | Enum state machines / Typestate | `enum` + `match` / phantom types |
| Visitor | Pattern matching | `match` expressions |
| Template Method | Higher-order functions | Generic functions with closures |
| Decorator | Functional composition | Wrapper types / combinators |
| Chain of Responsibility | Iterator chains | `.filter_map()` / `.fold()` |

### Best Practices for Functional Patterns

1. **Prefer composition over inheritance**: Use trait composition and functional combinators
2. **Leverage the type system**: Use phantom types and generics for compile-time guarantees
3. **Embrace immutability**: Default to immutable data structures and functional updates
4. **Use algebraic data types**: Model domains with enums and pattern matching
5. **Apply systematic transformations**: Use catamorphisms and anamorphisms for recursive data
6. **Design for composability**: Create small, composable functions that can be combined
7. **Validate at boundaries**: Use smart constructors and newtypes for input validation
8. **Test properties, not implementations**: Use property-based testing for robust verification

### Anti-Pattern Recognition

Watch for these signs that OOP patterns might be inappropriately applied:
- Complex inheritance hierarchies with multiple levels
- Excessive use of dynamic dispatch where static dispatch would suffice
- State management through mutable object fields instead of functional state
- Command objects where simple closures would be more appropriate
- Factory hierarchies where associated functions would be clearer
- Visitor pattern implementations where pattern matching would be simpler

## Conclusion

Functional programming patterns in Rust provide powerful abstractions that align with the language's core principles of safety, performance, and expressiveness. By preferring these patterns over traditional OOP approaches, we can create more maintainable, testable, and performant code that leverages Rust's unique strengths.

The key is to think in terms of data transformation, composition, and immutability rather than object hierarchies and mutable state. This functional-first approach leads to code that is easier to reason about, test, and extend while taking full advantage of Rust's compile-time guarantees.
