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
        use core::ops::{Mul, Div, Add, Sub, AddAssign, SubAssign, MulAssign, DivAssign };
        use $crate::quantity::IsQuantity;
        impl IsQuantity for $quantity
        {        
            fn value(&self) -> f64 {
                self.value
            }
        
            fn definition(&self) -> UnitDefinition {
                self.unit.into()
            }
        }
        
        impl Mul<f64> for $quantity
        {
            type Output = $quantity;

            fn mul(self, rhs: f64) -> Self::Output {
                Self { value: self.value*rhs, unit: self.unit }
            }
        }
        impl Div<f64> for $quantity
        {
            type Output = $quantity;

            fn div(self, rhs: f64) -> Self::Output {
                Self { value: self.value/rhs, unit: self.unit }
            }
        }

        
        impl MulAssign<f64> for $quantity
        {
            fn mul_assign(&mut self, rhs: f64) {
                self.value *= rhs;
            }
        }


        impl DivAssign<f64> for $quantity
        {
            fn div_assign(&mut self, rhs: f64) {
                self.value /= rhs;
            }
        }

        impl PartialEq<QuantityBase> for $quantity
        {
            fn eq(&self, other: &QuantityBase) -> bool {
                QuantityBase::from(*self).eq(other)
            }
        }
        
        impl PartialEq<$quantity> for QuantityBase
        {
            fn eq(&self, other: &$quantity) -> bool {
                *other == *self
            }
        }

        impl PartialEq<$quantity> for $quantity
        {
            fn eq(&self, other: &$quantity) -> bool {
                self.value * self.definition().multiplier == other.value * other.definition().multiplier
            }
        }

        impl<T:IsQuantity> Mul<T> for $quantity
        {
            type Output = QuantityBase;
            fn mul(self, rhs: T) -> QuantityBase {
                QuantityBase{ value: self.value*rhs.value(), unit: self.definition()*rhs.definition() }
            }
        }
        impl<T:IsQuantity> Div<T> for $quantity
        {
            type Output = QuantityBase;

            fn div(self, rhs: T) -> QuantityBase {
                QuantityBase{ value: self.value/rhs.value(), unit: self.definition()/rhs.definition() }
            }
        }
        impl Add<$quantity> for $quantity
        {
            type Output=Self;

            fn add(self, rhs: $quantity) -> Self {
                let rhs_value = rhs.convert_unchecked(self.definition());
                Self{ value: self.value + rhs_value, unit: self.unit }
            }
        }
        impl Sub<$quantity> for $quantity
        {
            type Output=Self;
            fn sub(self, rhs: $quantity) -> Self {
                let rhs_value = rhs.convert_unchecked(self.definition());
                Self{ value: self.value - rhs_value, unit: self.unit }
            }
        }       

        impl AddAssign for $quantity
        {
            fn add_assign(&mut self, rhs: Self) {
                let rhs_value = rhs.convert_unchecked(self.definition());
                self.value += rhs_value;
            }
        }

        impl SubAssign for $quantity
        {
            fn sub_assign(&mut self, rhs: Self) {
                let rhs_value = rhs.convert_unchecked(self.definition());
                self.value -= rhs_value;
            }
        }

        impl Add<QuantityBase> for $quantity
        {
            type Output=Self;

            fn add(self, rhs: QuantityBase) -> Self {
                if self.definition().base != rhs.unit.base
                {
                    panic!("Incompatible units added");
                }
                let rhs_value = rhs.convert_unchecked(self.definition());
                Self{ value: self.value + rhs_value, unit: self.unit }
            }
        }
        impl Sub<QuantityBase> for $quantity
        {
            type Output=Self;
            fn sub(self, rhs: QuantityBase) -> Self {
                if self.definition().base != rhs.unit.base
                {
                    panic!("Incompatible units subtracted");
                }
                let rhs_value = rhs.convert_unchecked(self.definition());
                Self{ value: self.value - rhs_value, unit: self.unit }
            }
        }       

        impl AddAssign<QuantityBase> for $quantity
        {
            fn add_assign(&mut self, rhs: QuantityBase) {
                if self.definition().base != rhs.unit.base
                {
                    panic!("Incompatible units subtracted");
                }
                let rhs_value = rhs.convert_unchecked(self.definition());
                self.value += rhs_value;
            }
        }

        impl SubAssign<QuantityBase> for $quantity
        {
            fn sub_assign(&mut self, rhs: QuantityBase) {
                if self.definition().base != rhs.unit.base
                {
                    panic!("Incompatible units subtracted");
                }
                let rhs_value = rhs.convert_unchecked(self.definition());
                self.value -= rhs_value;
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
        use $crate::errors::RuntimeUnitError;
        use $crate::QuantityBase;
        use $crate::units_base::{UnitDefinition, UnitBase};
        paste::paste!{
        $(#[$quantity_attr])*
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        #[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
        #[allow(non_camel_case_types)]         
        #[allow(clippy::eq_op)]
        #[cfg_attr(feature="utoipa", derive(ToSchema))]        
        pub enum [<$quantity Unit>]
        {
            $($unit,)+
        }
        }        
        paste::paste! { 
            pub(crate) const [<$quantity:upper _UNIT_BASE>]: UnitBase = crate::units_base::UOMDimensions::to_unit_base(($($crate::units_base::UOMDimensions::$dimension,)+));
            $(pub(crate) const [<$quantity:upper _ $unit:upper _conversion:upper>]: f64 = $conversion;)+
        impl [<$quantity Unit>] 
        {
            
            /// get the UnitBase for this type of unit
            pub(crate) const fn unit_base() -> UnitBase
            {
                [<$quantity:upper _UNIT_BASE>]
            }

            /// Retrieve the underlying `UnitDefinition`
            pub const fn unit(&self) -> UnitDefinition
            {
                match self
                {
                    $([<$quantity Unit>]::$unit => UnitDefinition 
                        { 
                            multiplier: [<$quantity:upper _ $unit:upper _conversion:upper>], 
                            base: [<$quantity Unit>]::unit_base()
                        },
                    )+
                }
                
            }

            /// Get the base unit for this unit type (for length, as an example, this would be `meter`)
            pub fn base(&self) -> [<$quantity Unit>]
            {
                $(if [<$quantity:upper _ $unit:upper _conversion:upper>] == 1.0 { return [<$quantity Unit>]::$unit; })+
                panic!("No base unit found! for {}", stringify!($quantity));
            }
            paste::paste!{
            $(
                #[allow(clippy::eq_op)]
                pub fn [<get_$unit:snake>]() -> UnitDefinition
                {
                    UnitDefinition{ base: [<$quantity Unit>]::unit_base(), multiplier: [<$quantity:upper _ $unit:upper _conversion:upper>] }
                })+
            }        
            #[doc = "Multiplier of unit to its base quantity."]
            #[allow(clippy::eq_op)]            
            pub fn multiplier(&self) -> f64
            {
                match self
                {
                    $([<$quantity Unit>]::$unit => [<$quantity:upper _ $unit:upper _conversion:upper>],)+
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
            pub fn units() -> &'static [&'static str]
            {
                const UNITS: &[&'static str] = &[ $($singular,)+ ];                
                UNITS
            }
            
        }
        }
        paste::paste!
        {
            impl From<[<$quantity Unit>]> for $crate::Units
            {
                fn from(value: [<$quantity Unit>]) -> Self {
                    match value
                    {
                        $(
                            [<$quantity Unit>]::$unit => $crate::Units::$quantity([<$quantity Unit>]::$unit),
                        )+
                    }
                }
            }
            impl TryFrom<$crate::Units> for [<$quantity Unit>]
            {
                type Error = RuntimeUnitError;
                fn try_from(value: $crate::Units) -> Result<Self, Self::Error> 
                {                
                    match value
                    {
                        $($crate::Units::$quantity([<$quantity Unit>]::$unit) => Ok([<$quantity Unit>]::$unit),)+
                        _ => Err(RuntimeUnitError::IncompatibleUnitConversion(format!("Could not convert from {} to {}", value, stringify!($quantity))))
                    }
                }
            }
        }
        paste::paste!
        {
            $(#[$quantity_attr])*
            #[derive(Copy, Clone, Debug)]
            #[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature="utoipa", derive(ToSchema))]
            pub struct $quantity
            {
                pub value: f64,
                pub unit: [<$quantity Unit>]   
            }
            impl $quantity
            {
                #[doc = "Create a new [`" [<$quantity Unit>]"`]."]   
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

                #[doc = "Retrieve the `UnitDefinition` associated with this [`" [<$quantity>]"`]."]   
                #[inline]
                pub fn definition(&self) -> UnitDefinition
                {
                    self.unit.into()
                }
                #[inline] 
                /// Convert from one unit to another (no check is made to ensure destination unit is valid).
                fn convert_unchecked(&self, unit: UnitDefinition) -> f64
                {
                    let definition = self.definition();
                    if definition == unit
                    {
                        self.value
                    }
                    else
                    {
                        self.value * definition.multiplier / unit.multiplier()
                    }
                }
                $(
                    #[doc = "Convert to [`" [<$quantity Unit>] "::" [<$unit>] "`]."]                        
                    #[inline]
                    pub fn [<to_ $unit:snake>](&self) -> Self
                    {                     
                        Self { value: self.value * (self.definition().multiplier / ([<$quantity:upper _ $unit:upper _conversion:upper>])) as f64, unit: [<$quantity Unit>]::$unit }    
                    }
                )+

                #[doc = "Convert [`" [<$quantity>] "`] to another unit of the same quantity."]                                        
                pub fn convert(&self, unit: [<$quantity Unit>]) -> Self
                {
                    Self { value: self.convert_unchecked(unit.into()), unit: unit }
                }
                pub fn try_convert(&self, unit: $crate::Units) -> Result<Self, RuntimeUnitError>
                {
                    let destination_unit:  [<$quantity Unit>] = unit.try_into()?;                   
                    Ok(self.convert(destination_unit))                    
                }                
            }
            
            impl From<$quantity> for UnitDefinition
            {
                #[inline]
                fn from(value: $quantity) -> Self 
                {
                    paste::paste!{
                        match value.unit
                        {
                            $([<$quantity Unit>]::$unit =>  [<$quantity Unit>]::[<get_$unit:snake>](),)+
                        }   
                    }
                }
            }
            impl From<$quantity> for QuantityBase
            {    
                fn from(quantity: $quantity) -> Self {
                    Self { value: quantity.value, unit: quantity.unit.into() }
                }                
            }
            use crate::impl_quantity_ops;
            impl_quantity_ops!($quantity);
        }        
        paste::paste!
        {
            impl From<[<$quantity Unit>]> for UnitDefinition
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
            impl TryFrom<&str> for [<$quantity Unit>]
            {
                type Error = &'static str;
                fn try_from(value: &str) -> Result<Self, Self::Error> 
                {                
                    let abbreviation_check: Option<[<$quantity Unit>]> =
                    match value
                    {                        
                        $($abbreviation => Some([<$quantity Unit>]::$unit),)+
                        _ => None
                    };
                    if abbreviation_check.is_some()
                    {
                        return Ok(abbreviation_check.unwrap());
                    }
                    let singular_check: Option<[<$quantity Unit>]> = 
                    match value
                    {                        
                        $($singular => Some([<$quantity Unit>]::$unit),)+
                        _ => None
                    };
                    if singular_check.is_some()
                    {
                        return Ok(singular_check.unwrap());
                    }
                    let plural_check: Option<[<$quantity Unit>]> = 
                    match value
                    {                        
                        $($plural => Some([<$quantity Unit>]::$unit),)+
                        _ => None
                    };
                    if plural_check.is_some()
                    {
                        Ok(plural_check.unwrap())
                    }
                    else
                    {
                        Err(concat!("Unit \"{value}\" not supported for quantity \"", stringify!{$quantity}, "\""))

                    }
                }
            }
        }
        paste::paste!{
        impl core::fmt::Display for [<$quantity Unit>] {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                match self
                {
                    $(
                        [<$quantity Unit>]::$unit => write!(f, "{}", $singular),
                    )+    
                }
               
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
        use $crate::units_base::UnitDefinition;
        use $crate::QuantityBase;
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
                    pub use $crate::unit_definitions::[<$quantity:snake>]::$quantity;           
                }
            )+
            pub use $crate::quantity::QuantityBase;
            
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
            ///
            /// A list of unit types supported for the library (given feature flags selected).
            /// 
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
                ///
                /// Retrieve list of units available for this `UnitType`.
                /// 
                pub fn units(&self) -> &'static [&'static str]
                {
                    match self
                    {
                        
                        $(      
                            #[cfg(any(feature = "" $quantity, feature="All"))]                      
                            UnitTypes::$quantity=>[<$quantity:snake>]::[<$quantity Unit>]::units(),
                        )+
                        
                    }
                }       
                ///
                /// Convert a given unit string to the Corresponding `Units`
                /// 
                pub fn to_unit(&self, unit_str: &str) -> Result<$crate::Units, &'static str>
                {
                    match self
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            UnitTypes::$quantity => {
                                match crate::units::[<$quantity Unit>]::try_from(unit_str)
                                {
                                    Ok(r) => Ok(r.into()),
                                    Err(err) => Err(err)
                                }                                
                            }
                        )+
                    }
                }
            }
        }
        paste::paste!{
            #[derive(Copy, Clone, Debug, PartialEq)]
            #[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature="utoipa", derive(ToSchema))]
            ///
            /// A wrapper to hold all quantities supported by this library. It is analogous to `Units``, 
            /// but when combined with the `serde` feature flag, can serve as a way to serialize a quantity, not just the unit. 
            /// 
            pub enum Quantities
            {
                $(
                    #[cfg(any(feature = "" $quantity, feature="All"))]   
                    $quantity($quantity),
                )+
            }

            impl From<Quantities> for QuantityBase
            {
                fn from(value: Quantities) -> Self {
                    match value
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            Quantities::$quantity(x)=>QuantityBase { value: x.value, unit: UnitDefinition{multiplier: x.unit.multiplier(), base: [<$quantity:snake>]::[<$quantity Unit>]::unit_base()} },
                        )+
                    }
                }
            }
            
            paste::paste!{     
                #[derive(Copy, Clone, Debug, PartialEq, Eq)]
                #[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
                #[derive(Hash)]
                #[cfg_attr(feature="utoipa", derive(ToSchema))]
                ///
                /// A wrapper to hold all available units supported by the library. Contains utilities to convert from
                /// a given arbitrary unit to the underlying `QuantityBase` that is used to perform unit conversion calculations.
                /// 
                pub enum Units
                {                               
                    $(
                        #[cfg(any(feature = "" $quantity, feature="All"))]                 
                        $quantity([<$quantity:snake>]::[<$quantity Unit>]),
                    )+                
                }
                
                impl From<Units> for $crate::units_base::UnitDefinition
                {
                    fn from(value: Units) -> Self {
                        match value
                        {
                            $(
                                #[cfg(any(feature = "" $quantity, feature="All"))]     
                                Units::$quantity(x)=> UnitDefinition{multiplier: x.multiplier(), base: [<$quantity:snake>]::[<$quantity Unit>]::unit_base()},
                            )+
                        }
                    }
                }
                paste::paste!{
                    impl core::fmt::Display for Units{
                        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                            match self
                            {
                                $(
                                    #[cfg(any(feature = "" $quantity, feature="All"))]  
                                    Units::$quantity(x) => write!(f, "{}", x.singular()),
                                )+    
                            }
                           
                        }
                    }        
                }  
            }
            impl Units
            {
                ///
                /// Convert a given 'value' expressed in the given `Units` intto a convertible `Quantity`
                /// 
                pub fn to_quantity(&self, value: f64) -> $crate::quantity::QuantityBase
                {
                    let unit: UnitDefinition = (*self).into();
                    $crate::quantity::QuantityBase { unit, value }
                }
                paste!{

                ///
                /// Get the base unit for this particular unit type (e.g. `LengthUnit` this would be `LengthUnit::meter`)
                /// 
                pub fn base(&self) -> Units
                {
                    match *self
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]  
                            Units::$quantity(x)=>Units::$quantity(x.base()),)+
                    }
                }     
                ///
                /// Get list of units available for this `Units` type
                /// 
                pub fn units(&self) -> &'static [&'static str]
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
