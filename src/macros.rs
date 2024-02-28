#[macro_export]
#[allow(clippy::eq_op)]
macro_rules! prefix {
    // SI prefixes.
    (yotta) => { 1.0_E24 };
    (zetta) => { 1.0_E21 };
    (exa) => { 1.0_E18 };
    (peta) => { 1.0_E15 };
    (tera) => { 1.0_E12 };
    (giga) => { 1.0_E9 };
    (mega) => { 1.0_E6 };
    (kilo) => { 1.0_E3 };
    (hecto) => { 1.0_E2 };
    (deca) => { 1.0_E1 };
    (none) => { 1.0_E0 };
    (deci) => { 1.0_E-1 };
    (centi) => { 1.0_E-2 };
    (milli) => { 1.0_E-3 };
    (micro) => { 1.0_E-6 };
    (nano) => { 1.0_E-9 };
    (pico) => { 1.0_E-12 };
    (femto) => { 1.0_E-15 };
    (atto) => { 1.0_E-18 };
    (zepto) => { 1.0_E-21 };
    (yocto) => { 1.0_E-24 };

    // Binary prefixes.
    (yobi) => { 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 };
    (zebi) => { 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 };
    (exbi) => { 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 };
    (pebi) => { 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 };
    (tebi) => { 1024.0 * 1024.0 * 1024.0 * 1024.0 };
    (gibi) => { 1024.0 * 1024.0 * 1024.0 };
    (mebi) => { 1024.0 * 1024.0 };
    (kibi) => { 1024.0 };
}
#[macro_export]
macro_rules! impl_quantity_ops {   
    ($quantity:ident) =>
    {
        use core::ops::{Mul, Div, Add, Sub, AddAssign, SubAssign, MulAssign, DivAssign};
        use $crate::quantity::IsQuantity;
        impl IsQuantity for $quantity
        {        
            fn value(&self) -> f64 {
                self.0.value
            }
        
            fn unit(&self) -> Unit {
                self.0.unit
            }
        }
        
        impl Mul<f64> for $quantity
        {
            type Output = $quantity;

            fn mul(self, rhs: f64) -> Self::Output {
                Self(Quantity{ value: self.value*rhs, unit: self.unit })
            }
        }
        impl Div<f64> for $quantity
        {
            type Output = $quantity;

            fn div(self, rhs: f64) -> Self::Output {
                Self(Quantity{ value: self.value/rhs, unit: self.unit })
            }
        }
        impl PartialEq<Quantity> for $quantity
        {
            fn eq(&self, other: &Quantity) -> bool {
                self.0 == *other
            }
        }
        impl PartialEq<$quantity> for Quantity
        {
            fn eq(&self, other: &$quantity) -> bool {
                other == self
            }
        }

        impl<T:IsQuantity> Mul<T> for $quantity
        {
            type Output = Quantity;
            fn mul(self, rhs: T) -> Quantity {
                Quantity{ value: self.value*rhs.value(), unit: self.unit*rhs.unit() }
            }
        }
        impl<T:IsQuantity> Div<T> for $quantity
        {
            type Output = Quantity;

            fn div(self, rhs: T) -> Quantity {
                Quantity{ value: self.value/rhs.value(), unit: self.unit/rhs.unit() }
            }
        }
        impl<T:IsQuantity> Add<T> for $quantity
        {
            type Output=Quantity;

            fn add(self, rhs: T) -> Quantity {
               Quantity{ value: self.value + rhs.value(), unit: self.unit }
            }
        }
        impl<T:IsQuantity> Sub<T> for $quantity
        {
            type Output=Quantity;
            fn sub(self, rhs: T) -> Quantity {
                Quantity{ value: self.value - rhs.value(), unit: self.unit }
            }
        }
        impl<T:IsQuantity> AddAssign<T> for $quantity
        {
            fn add_assign(&mut self, rhs: T) {
                self.value += rhs.value();
            }
        }

        impl<T:IsQuantity>  SubAssign<T> for $quantity
        {
            fn sub_assign(&mut self, rhs: T) {
                self.value -= rhs.value();
            }
        }

        impl<T:IsQuantity>  MulAssign<T> for $quantity
        {
            fn mul_assign(&mut self, rhs: T) {
                self.value *= rhs.value();
                self.unit *= rhs.unit();
            }
        }

        impl<T:IsQuantity> DivAssign<T> for $quantity
        {
            fn div_assign(&mut self, rhs: T) {
                self.value /= rhs.value();
                self.unit /= rhs.unit();
            }
        }

    }
}

#[macro_export]
macro_rules! quantity {
    (        
        $(#[$quantity_attr:meta])* quantity: $quantity:ident; $description:expr;
        $(#[$dim_attr:meta])* dimension: $system:ident<$($dimension:ident),+>;
        $(kind: $kind:ty;)?
        units {            
            $($(#[$unit_attr:meta])* @$unit:ident: $conversion:expr; $abbreviation:literal,
                $singular:literal, $plural:literal;)+
        }
    ) => {
        #[cfg(feature="utoipa")]
        use utoipa::ToSchema;
        use core::ops::{Deref, DerefMut};
        use once_cell::sync::Lazy;
        use strum_macros::EnumIter;
        use $crate::Quantity;
        use $crate::units_base::{Unit, UnitBase};
        paste::paste!{
        $(#[$quantity_attr])*
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        #[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(EnumIter)]
        #[allow(non_camel_case_types)]         
        #[allow(clippy::eq_op)]
        #[cfg_attr(feature="utoipa", derive(ToSchema))]
        pub enum [<$quantity Unit>]
        {
            $($unit,)+
        }
        }
        paste::paste! { 
        impl [<$quantity Unit>] 
        {
            /// get the UnitBase for this type of unit
            pub(crate) fn get_unit_base() -> UnitBase
            {
                $crate::units_base::UOMDimensions::to_unit_base(($($crate::units_base::UOMDimensions::$dimension,)+))
            }

            /// Get the base unit for this unit type (for length, as an example, this would be `meter`)
            pub fn base_unit(&self) -> [<$quantity Unit>]
            {
                $(if $conversion == 1.0 { return [<$quantity Unit>]::$unit; })+
                panic!("No base unit found! for {}", stringify!($quantity));
            }
            paste::paste!{
            $(
                #[allow(clippy::eq_op)]
                pub fn [<get_$unit:snake>]() -> Unit
                {
                    Unit{ base: [<$quantity Unit>]::get_unit_base(), multiplier: $conversion }
                })+
            }        
            #[doc = "Multiplier of unit to its base quantity."]
            #[allow(clippy::eq_op)]            
            pub fn multiplier(&self) -> f64
            {
                match self
                {
                    $([<$quantity Unit>]::$unit => $conversion,)+
                }
            }
            #[doc = "Abbreviation of unit."]
            pub fn abbreviation(&self) -> &'static str
            {
                match self
                {
                    $([<$quantity Unit>]::$unit => $abbreviation,)+
                }
            }
            #[doc = "Singular name of unit."]
            pub fn singular(&self) -> &'static str
            {
                match self
                {
                    $([<$quantity Unit>]::$unit => $singular,)+
                }
            }
            #[doc = "Plural name of unit."]
            pub fn plural(&self) -> &'static str
            {
                match self
                {
                    $([<$quantity Unit>]::$unit => $plural,)+
                }
            }
            #[doc = "Available units for this `[" [<$quantity Unit>] "`]."]
            pub fn units() -> &'static Vec<&'static str>
            {
                use strum::IntoEnumIterator;
                static UNITS: Lazy<Vec<&'static str>> = Lazy::new(|| { 
                    [<$quantity Unit>]::iter().map(|item|item.singular()).collect()
                });
                &UNITS
            }
            
        }
        }
        paste::paste!
        {
            $(#[$quantity_attr])*
            #[derive(Copy, Clone, Debug, PartialEq)]
            #[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature="utoipa", derive(ToSchema))]
            pub struct $quantity
            {
                pub value: f64,
                pub unit: [<$quantity Unit>]   
            }
            impl $quantity
            {
                pub fn new(value: f64, unit: [<$quantity Unit>]) -> Self
                {
                    Self {value, unit }
                }
                $(
                    #[doc = "Create a new [`" [<$quantity>] "`] with units of [`" [<$quantity Unit>] "::" [<$unit>] "`]."] 
                    pub fn [<$unit:snake>](value: f64) -> Self
                    {
                        Self { value, unit: [<$quantity Unit>]::$unit.into() }
                    }
                )+
                #[doc = "Create a [`" [<Quantity>] "`] from this [`"[<$quantity>]"`]." ]
                pub fn to_quantity(&self) -> Quantity
                {
                    (*self).into()
                }
            }
            impl From<$quantity> for Quantity
            {    
                fn from(quantity: $quantity) -> Self {
                    Self { value: quantity.value, unit: quantity.unit.into() }
                }                
            }
            impl From<$quantity> for [<$quantity Quantity>]
            {    
                fn from(quantity: $quantity) -> Self {
                    Self (Quantity{ value: quantity.value, unit: quantity.unit.into() })
                }                
            }
            use crate::impl_quantity_ops;
            impl_quantity_ops!([<$quantity Quantity>]);
        }        
        paste::paste!
        {
            impl From<[<$quantity Unit>]> for Unit
            {
                #[inline]
                fn from(value: [<$quantity Unit>]) -> Self 
                {
                    paste::paste!{
                        match value
                        {
                            $([<$quantity Unit>]::$unit => [<$quantity Unit>]::[<get_$unit:snake>](),)+
                        }   
                    }
                }
            }
        }
        paste::paste!
        {
            impl From<&str> for [<$quantity Unit>]
            {
                fn from(value: &str) -> Self 
                {                
                    let abbreviation_check: Option<[<$quantity Unit>]> =
                    match value
                    {                        
                        $($abbreviation => Some([<$quantity Unit>]::$unit),)+
                        _ => None
                    };
                    if abbreviation_check.is_some()
                    {
                        return abbreviation_check.unwrap();
                    }
                    let singular_check: Option<[<$quantity Unit>]> = 
                    match value
                    {                        
                        $($singular => Some([<$quantity Unit>]::$unit),)+
                        _ => None
                    };
                    if singular_check.is_some()
                    {
                        return singular_check.unwrap();
                    }
                    let plural_check: Option<[<$quantity Unit>]> = 
                    match value
                    {                        
                        $($plural => Some([<$quantity Unit>]::$unit),)+
                        _ => None
                    };
                    if plural_check.is_some()
                    {
                        plural_check.unwrap()
                    }
                    else
                    {
                        panic!("Unit \"{value}\" not supported for quantity \"{}\"", stringify!{$quantity})
                    }
                }
            }
        }
        paste::paste!{
        impl std::fmt::Display for [<$quantity Unit>] {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self
                {
                    $(
                        [<$quantity Unit>]::$unit => write!(f, "{}", $singular),
                    )+    
                }
               
            }
        }       
        }
        // now create Quantity element to enable conversions...        
        paste::paste!{
            $(#[$quantity_attr])*
            #[derive(Copy, Clone, Debug)]
            pub struct [<$quantity Quantity>](Quantity);
            impl [<$quantity Quantity>]
            {
                #[doc = "Create a new quantity with a given `value` and [`" [<$quantity Unit>]"`]."]   
                pub fn new(value: f64, unit: [<$quantity Unit>]) -> Self
                {
                    Self(Quantity { value, unit: unit.into() })
                }
                paste::paste!
                {
                    $(
                        #[doc = "Convert to [`" [<$quantity Unit>] "::" [<$unit>] "`]."]                        
                        #[inline]
                        pub fn [<to_ $unit:snake>](&self) -> Self
                        {                     
                            Self(Quantity { value: self.value * (self.unit.multiplier / ($conversion)) as f64, unit: [<$quantity Unit>]::$unit.into() })    
                        }
                    )+
                    $(
                        #[doc = "Create a new [`" [<$quantity Quantity>] "`] with units of [`" [<$quantity Unit>] "::" [<$unit>] "`]."] 
                        pub fn [<$unit:snake>](value: f64) -> Self
                        {
                            Self(Quantity { value, unit: [<$quantity Unit>]::$unit.into() })
                        }

                    )+
                    
                    #[doc = "Convert [`" [<$quantity Quantity>] "`] to another unit of the same quantity."]                                        
                    pub fn [<convert_ $quantity:snake>](&self, unit: [<$quantity Unit>]) -> Self
                    {
                        Self(Quantity { value: self.convert_unit(unit.into()), unit: unit.into() })
                    }
                }      
            }
            impl Deref for [<$quantity Quantity>]
            {
                type Target = Quantity;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            impl DerefMut for [<$quantity Quantity>]
            {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            } 
        }
    }
}

#[macro_export]
macro_rules! system {
    (
        $(#[$units_attr:meta])* units: $units:ident {
            $($quantity:ident,)+
        }
    ) => {
        use $crate::units_base::Unit;
        use $crate::units_base::UnitBase;
        use $crate::Quantity;
        #[cfg(feature="utoipa")]
        use utoipa::ToSchema;
        use paste::paste;
        paste::paste!{$(            
           paste::paste!{#[cfg(any(feature = "" $quantity, feature="All"))]
           mod [<$quantity:snake>];
            }
           paste::paste!{#[cfg(any(feature = "" $quantity, feature="All"))]           
           pub use $crate::unit_definitions::[<$quantity:snake>]::$quantity;      
        }
           
        )+
        pub mod quantities
        {
            $(  
                paste::paste!{
                    #[allow(clippy::eq_op)]
                    #[cfg(any(feature = "" $quantity, feature="All"))]                    
                    pub use $crate::unit_definitions::[<$quantity:snake>]::[<$quantity Quantity>];           
                }
            )+
            pub use $crate::quantity::Quantity;
            
        }
        pub mod units
        {
         
            $(                
                paste::paste!{#[cfg(any(feature = "" $quantity, feature="All"))]    
                pub use $crate::unit_definitions::[<$quantity:snake>]::[<$quantity Unit>];      
                }
            )+
        }
        }
        paste!{
            #[derive(Copy, Clone, Debug)]
            pub enum UnitTypes
            {
                $(
                   #[cfg(any(feature = "" $quantity, feature="All"))] 
                    $quantity,)+
            }
        }
        impl UnitTypes
        {
            paste::paste!{
                pub fn units(&self) -> &'static Vec<&'static str>
                {
                    match self
                    {
                        
                        $(      
                            #[cfg(any(feature = "" $quantity, feature="All"))]                      
                            UnitTypes::$quantity=>[<$quantity:snake>]::[<$quantity Unit>]::units(),
                        )+
                        
                    }
                }       
            }
        }
        paste::paste!{
            #[derive(Copy, Clone, Debug, PartialEq)]
            #[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature="utoipa", derive(ToSchema))]
            pub enum Quantities
            {
                $(
                    #[cfg(any(feature = "" $quantity, feature="All"))]   
                    $quantity($quantity),
                )+
            }

            impl From<Quantities> for Quantity
            {
                fn from(value: Quantities) -> Self {
                    match value
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            Quantities::$quantity(x)=>Quantity { value: x.value, unit: Unit{multiplier: x.unit.multiplier(), base: [<$quantity:snake>]::[<$quantity Unit>]::get_unit_base()} },
                        )+
                    }
                }
            }
            
            paste::paste!{     
            #[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
            #[derive(Hash)]
            #[cfg_attr(feature="utoipa", derive(ToSchema))]
            pub enum Units
            {                               
                $(
                    #[cfg(any(feature = "" $quantity, feature="All"))]                 
                    $quantity([<$quantity:snake>]::[<$quantity Unit>]),
                )+                
            }
            
            impl From<Units> for $crate::units_base::Unit
            {
                fn from(value: Units) -> Self {
                    match value
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]     
                            Units::$quantity(x)=> Unit{multiplier: x.multiplier(), base: [<$quantity:snake>]::[<$quantity Unit>]::get_unit_base()},
                        )+
                    }
                }
            }
            }
            impl Units
            {
                pub fn to_quantity(&self, value: f64) -> $crate::quantity::Quantity
                {
                    let unit: Unit = (*self).into();
                    $crate::quantity::Quantity { unit, value }
                }
                paste!{
                pub fn multiplier(&self) -> f64
                {
                    match *self
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]  
                            Units::$quantity(x)=>x.multiplier(),)+
                    }
                }
                
                pub fn base(&self) -> UnitBase
                {
                    match *self
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            Units::$quantity(_)=>[<$quantity:snake>]::[<$quantity Unit>]::get_unit_base(),
                        )+
                    }
                }                
                
                pub fn base_unit(&self) -> Units
                {
                    match *self
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]  
                            Units::$quantity(x)=>Units::$quantity(x.base_unit()),)+
                    }
                }     
                
                pub fn units(&self) -> &'static Vec<&'static str>
                {
                    match *self
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            Units::$quantity(_)=>[<$quantity:snake>]::[<$quantity Unit>]::units(),
                        )+
                    }
                }      
                }  
            }
           
            
        }
    };
}
