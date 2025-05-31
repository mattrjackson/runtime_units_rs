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
        use $crate::traits::IsScalarQuantity;
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

        impl PartialEq<Quantity> for $quantity
        {
            fn eq(&self, other: &Quantity) -> bool {
                Quantity::from(*self).eq(other)
            }
        }
        
        impl PartialEq<$quantity> for Quantity
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
        
        impl PartialOrd for $quantity
        {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                (self.value * self.definition().multiplier).partial_cmp(&(other.value * other.definition().multiplier))
            }        
        }

        impl<T:IsScalarQuantity> Mul<T> for $quantity
        {
            type Output = Quantity;
            fn mul(self, rhs: T) -> Quantity {
                Quantity{ value: self.value*rhs.value(), unit: self.definition()*rhs.unit() }
            }
        }
        impl<T:IsScalarQuantity> Div<T> for $quantity
        {
            type Output = Quantity;

            fn div(self, rhs: T) -> Quantity {
                Quantity{ value: self.value/rhs.value(), unit: self.definition()/rhs.unit() }
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

        impl Add<Quantity> for $quantity
        {
            type Output=Self;

            fn add(self, rhs: Quantity) -> Self {
                if self.definition().base != rhs.unit.base
                {
                    panic!("Incompatible units added");
                }
                let rhs_value = rhs.convert_unchecked(self.definition());
                Self{ value: self.value + rhs_value, unit: self.unit }
            }
        }
        impl Sub<Quantity> for $quantity
        {
            type Output=Self;
            fn sub(self, rhs: Quantity) -> Self {
                if self.definition().base != rhs.unit.base
                {
                    panic!("Incompatible units subtracted");
                }
                let rhs_value = rhs.convert_unchecked(self.definition());
                Self{ value: self.value - rhs_value, unit: self.unit }
            }
        }       

        impl AddAssign<Quantity> for $quantity
        {
            fn add_assign(&mut self, rhs: Quantity) {
                if self.definition().base != rhs.unit.base
                {
                    panic!("Incompatible units subtracted");
                }
                let rhs_value = rhs.convert_unchecked(self.definition());
                self.value += rhs_value;
            }
        }

        impl SubAssign<Quantity> for $quantity
        {
            fn sub_assign(&mut self, rhs: Quantity) {
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
        $(#[$dim_attr:meta])* dimension: $system:ident[$($dimension:expr),+];
        $(kind: $kind:ty;)?
        units {            
            $($(#[$unit_attr:meta])* @$unit:ident: $conversion:expr; $abbreviation:literal,
                $singular:literal, $plural:literal;)+
        }
    ) => {
        #[cfg(feature="utoipa")]
        use utoipa::{ToSchema, schema};
        use static_init::dynamic;
        use $crate::errors::RuntimeUnitError;
        use $crate::Quantity;
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
        impl Default for [<$quantity Unit>]{
            fn default() -> Self {
                [<$quantity:upper _BASE_UNIT>]
            }
        }
        }        

        paste::paste! { 
            #[dynamic]
            pub(crate) static [<$quantity:upper _UNIT_BASE>]: UnitBase = $crate::units_base::to_unit_base(($($dimension,)+));
            $(pub(crate) const [<$quantity:upper _ $unit:upper _conversion:upper>]: f64 = $conversion;)+
        const [<$quantity:upper _BASE_UNIT>]: [<$quantity Unit>] =  [<get_base_unit_ $quantity:lower>]();
        const fn [<get_base_unit_ $quantity:lower>]() -> [<$quantity Unit>] 
        {
            
            $(if [<$quantity Unit>]::$unit.multiplier() == 1.0
            {
                return [<$quantity Unit>]::$unit;
            })+
            panic!("No base unit found");            
        }
        impl [<$quantity Unit>] 
        {
            paste::paste!{
            $(
                #[allow(clippy::eq_op)]
                pub fn [<get_$unit:snake>]() -> UnitDefinition
                {
                    UnitDefinition{ base: *[<$quantity:upper _UNIT_BASE>], multiplier: [<$quantity:upper _ $unit:upper _conversion:upper>] }
                })+
            }        
            #[doc = "Multiplier of unit to its base quantity."]
            #[allow(clippy::eq_op)]            
            pub const fn multiplier(&self) -> f64
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
            
            #[inline]
            pub(crate) fn base() -> UnitBase
            {
                *[<$quantity:upper _UNIT_BASE>]
            }       

            #[inline]
            pub(crate) fn base_unit(&self) -> Self
            {
                [<$quantity:upper _BASE_UNIT>]
            } 
        }
        impl $crate::traits::Unit for [<$quantity Unit>] 
        {
                /// Return unit definition for this Unit Type                
                fn definition(&self) -> UnitDefinition
                {
                    match self
                    {                    
                        $([<$quantity Unit>]::$unit=>[<$quantity Unit>]::[<get_$unit:snake>](),)+
                    }
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
            
            impl TryFrom<UnitDefinition> for [<$quantity Unit>]
            {
                type Error = RuntimeUnitError;                
                #[allow(unreachable_patterns)]
                fn try_from(value: UnitDefinition) -> Result<Self, Self::Error> 
                {
                    if value.base != *[<$quantity:upper _UNIT_BASE>] 
                    {
                        return Err(RuntimeUnitError::IncompatibleUnitConversion(format!("Base mismatch: {:?} vs {}", value, stringify!($quantity))));
                    }
                    match value.multiplier 
                    {
                        $(
                            [<$quantity:upper _ $unit:upper _conversion:upper>] => Ok([<$quantity Unit>]::$unit),
                        )+
                        _ => Err(RuntimeUnitError::IncompatibleUnitConversion(
                            format!("Unknown multiplier: {:?} for {}", value, stringify!($quantity))
                        ))
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
            // Permit creation of `Units` from a `FixedQuantity`
            impl From<[<$quantity>]> for $crate::Units
            {
                fn from(value: [<$quantity>]) -> Self {
                    $crate::Units::$quantity(value.unit)
                }
            }        

            // Permit creation of `Units` from a &`FixedQuantity`
            impl From<&[<$quantity>]> for $crate::Units
            {
                fn from(value: &[<$quantity>]) -> Self {
                    $crate::Units::$quantity(value.unit)
                }
            }      

            // Permit creation of `Units` from a `FixedSliceQuantity`
            impl From<[<$quantity Vec>]> for $crate::Units
            {
                fn from(value:[<$quantity Vec>]) -> Self {
                    $crate::Units::$quantity(value.unit)
                }
            }     

             // Permit creation of `Units` from a &`FixedSliceQuantity`
             impl From<&[<$quantity Vec>]> for $crate::Units
             {
                 fn from(value: &[<$quantity Vec>]) -> Self {
                     $crate::Units::$quantity(value.unit)
                 }
             }     
        }
        
        paste::paste!
        {
            use $crate::traits::FixedQuantity;
            $(#[$quantity_attr])*
            #[derive(Copy, Clone, Debug, Default)]
            #[cfg_attr(feature="serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature="utoipa", derive(ToSchema))]
            #[cfg_attr(feature="utoipa", schema(title = "" [<$quantity>]))]
            #[doc = "Scalar storage of a quantity (f64 and [`" [<$quantity Unit>]"`])."]   
            pub struct $quantity
            {
                pub(crate) value: f64,
                pub(crate) unit: [<$quantity Unit>]   
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

                #[doc = "Retrieve the value associated with this [`" [<$quantity>]"`]."]   
                #[inline]
                pub fn value(&self) -> f64
                {
                    self.value
                }

                #[doc = "Retrieve the mutable value associated with this [`" [<$quantity>]"`]."]   
                #[inline]
                pub fn value_mut(&mut self) -> &mut f64
                {
                    &mut self.value
                }          

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
            }

                        
            impl FixedQuantity<[<$quantity Unit>]> for $quantity
            {                
                #[inline]
                fn unit(&self) -> [<$quantity Unit>]
                {
                    self.unit
                }

                #[inline]
                fn convert(&self, unit: [<$quantity Unit>]) -> Self
                {
                    Self { value: self.convert_unchecked(unit.into()), unit }
                }
                
                #[inline]
                fn convert_mut(&mut self, unit: [<$quantity Unit>])
                {
                    self.value = self.convert_unchecked(unit.into());
                    self.unit = unit;
                }
                #[inline]
                fn unit_mut(&mut self) -> &mut [<$quantity Unit>]
                {
                    &mut self.unit
                }

                #[inline]
                fn try_convert(&self, unit: $crate::Units) -> Result<Self, RuntimeUnitError> where Self: Sized
                {
                    let destination_unit: [<$quantity Unit>] = unit.try_into()?;                   
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
            impl From<$quantity> for Quantity
            {    
                fn from(quantity: $quantity) -> Self {
                    Self { value: quantity.value, unit: quantity.unit.into() }
                }                
            }
            impl From<Quantity> for $quantity
            {
                fn from(quantity: Quantity) -> $quantity {
                    $quantity { value: quantity.value, unit: [<$quantity Unit>]::try_from(quantity.unit).unwrap() }
                } 
            }
            impl $crate::traits::IsScalarQuantity for $quantity
            {
                fn value(&self) -> f64
                {
                    self.value
                }

                fn unit(&self) -> UnitDefinition
                {
                    self.unit.into()
                }
            }
            use $crate::impl_quantity_ops;
            
            use $crate::create_multivalue_quantities;
            use crate::{impl_quantity_vec_ops, impl_quantity_array_ops};
            
            
            impl [<$quantity Vec>]
            {
                #[doc = "Create a new vector of [`" [<$quantity Unit>]"`]."]   
                pub fn new(values: Vec<f64>, unit: [<$quantity Unit>]) -> Self
                {
                    Self{values, unit}
                }
                #[inline]
                #[doc = "Retrieve values associated with this [`" [<$quantity Slice>]"`]."]   
                pub fn values(&self) -> &Vec<f64>
                {
                    &self.values
                }
                #[inline]
                #[doc = "Retrieve the mutable values associated with this [`" [<$quantity Slice>]"`]."]   
                pub fn values_mut(&mut self) -> &mut Vec<f64>
                {
                    &mut self.values
                }

                $(
                    #[doc = "Create a new [`" [<$quantity>] "`] with units of [`" [<$quantity Unit>] "::" [<$unit>] "`]."] 
                    pub fn [<$unit:snake>](values: Vec<f64>) -> Self
                    {
                        Self{ values, unit: [<$quantity Unit>]::$unit.into() }
                    }
                )+

                $(
                    #[doc = "Convert to [`" [<$quantity Unit>] "::" [<$unit>] "`]."]                        
                    #[inline]
                    pub fn [<to_ $unit:snake>](&self) -> Self
                    {                     
                        use crate::traits::FixedSliceQuantity;
                        let mut r = self.clone();
                        r.convert_mut([<$quantity Unit>]::$unit);
                        r
                    }
                )+   
            }

            impl<const D: usize> [<$quantity Array>]<D>
            {
                #[doc = "Create a new vector of [`" [<$quantity Unit>]"`]."]   
                pub fn new(values: [f64; D], unit: [<$quantity Unit>]) -> Self
                {
                    Self{values, unit}
                }
                #[inline]
                #[doc = "Retrieve values associated with this [`" [<$quantity Slice>]"`]."]   
                pub fn values(&self) -> &[f64; D]
                {
                    &self.values
                }
                #[inline]
                #[doc = "Retrieve the mutable values associated with this [`" [<$quantity Slice>]"`]."]   
                pub fn values_mut(&mut self) -> &mut [f64; D]
                {
                    &mut self.values
                }

                $(
                    #[doc = "Create a new [`" [<$quantity>] "`] with units of [`" [<$quantity Unit>] "::" [<$unit>] "`]."] 
                    pub fn [<$unit:snake>](values: [f64; D]) -> Self
                    {
                        Self{ values, unit: [<$quantity Unit>]::$unit.into() }
                    }
                )+

                $(
                    #[doc = "Convert to [`" [<$quantity Unit>] "::" [<$unit>] "`]."]                        
                    #[inline]
                    pub fn [<to_ $unit:snake>](&self) -> Self
                    {                     
                        use crate::traits::FixedSliceQuantity;
                        let mut r = self.clone();
                        r.convert_mut([<$quantity Unit>]::$unit);
                        r
                    }
                )+   
                
                pub fn at(&self, index: usize) -> $quantity
                {
                    $quantity{ unit: self.unit, value: self.values[index] }
                }
            }
            
            impl_quantity_ops!($quantity);
            create_multivalue_quantities!($quantity);
            impl_quantity_vec_ops!($quantity);
            impl_quantity_array_ops!($quantity);
            
            use crate::vector_quantity::VecQuantity;
            impl From<[<$quantity Vec>]> for VecQuantity
            {
                fn from(input: [<$quantity Vec>]) -> Self 
                {
                    VecQuantity{ unit: input.unit.definition(), values: input.values }
                }
            }
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
        use $crate::errors::RuntimeUnitError;
        use $crate::Quantity;
        use $crate::units::*;
        #[cfg(feature="utoipa")]
        use utoipa::{ToSchema, schema};
        use paste::paste;
        paste::paste!{$(            
           paste::paste!{#[cfg(any(feature = "" $quantity, feature="All"))]
           mod [<$quantity:snake>];
            }
           paste::paste!{#[cfg(any(feature = "" $quantity, feature="All"))]       
           pub use $crate::unit_definitions::[<$quantity:snake>]::[<$quantity Array>];}
           paste::paste!{#[cfg(any(feature = "" $quantity, feature="All"))]       
           pub use $crate::unit_definitions::[<$quantity:snake>]::$quantity;}
           paste::paste!{#[cfg(any(feature = "" $quantity, feature="All"))]                                              
           pub use $crate::unit_definitions::[<$quantity:snake>]::[<$quantity Vec>];}
           
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
            pub use $crate::quantity::Quantity;
            
        }
        pub mod quantities_array
        {
            $(  
                paste::paste!{
                    #[allow(clippy::eq_op)]
                    #[cfg(any(feature = "" $quantity, feature="All"))]                    
                    pub use $crate::unit_definitions::[<$quantity:snake>]::$quantity;                         
                }
            )+
            pub use $crate::array_quantity::ArrayQuantity;
        }

        pub mod quantities_vector
        {
            $(  
                paste::paste!{
                    #[allow(clippy::eq_op)]
                    #[cfg(any(feature = "" $quantity, feature="All"))]                    
                    pub use $crate::unit_definitions::[<$quantity:snake>]::$quantity;                         
                }
            )+
            pub use $crate::vector_quantity::VecQuantity;
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
                                match $crate::units::[<$quantity Unit>]::try_from(unit_str)
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
                    #[cfg_attr(feature="utoipa", schema(title = "" $quantity))]
                    $quantity($quantity),
                )+
            }
            impl Quantities
            {                
                /// Get the `Units` enumeration associated with a given `Quantities` enumeration.
                pub fn unit(&self) -> Units
                {
                    match self
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]                               
                            Quantities::$quantity(x)=> $crate::Units::from(x.unit),
                        )+
                    }
                }
                /// Try to convert to the unit specified by a given `Units` enumeration.
                pub fn try_convert(&self, unit: Units) -> Result<Quantities, RuntimeUnitError>
                {   
                    use $crate::traits::FixedQuantity;
                    match self
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            Quantities::$quantity(x)=> {
                                Ok(Quantities::$quantity(x.try_convert(unit)?))
                            },
                        )+
                    }
                }

                /// Create a new quantity from a given value and unit
                pub fn new(value: f64, unit: Units) -> Quantities
                {
                    match unit
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            Units::$quantity(x)=> {                                
                                Quantities::$quantity($quantity::new(value, x))
                            },
                        )+
                    }
                }
                /// Get the value associated with quantity.
                pub fn value(&self) -> f64
                {
                    Quantity::from(*self).value()
                }
            }
            /// A means to create a default quantity with a given set of units.
            impl From<Units> for Quantities
            {
                fn from(value: Units) -> Self 
                {    
                    match value
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            Units::$quantity(x)=>Quantities::$quantity( quantities::$quantity::new(0.0, x)),
                        )+
                    }                
                    
                }
            }
            impl From<Quantities> for Quantity
            {
                fn from(value: Quantities) -> Self {
                    match value
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            Quantities::$quantity(x)=>Quantity { value: x.value, unit: UnitDefinition{multiplier: x.unit.multiplier(), base: *[<$quantity:snake>]::[<$quantity:upper _UNIT_BASE>]} },
                        )+
                    }
                }
            }
            impl ToString for Quantities
            {
                fn to_string(&self) -> String 
                {
                    match self
                    {
                        $(
                            #[cfg(any(feature = "" $quantity, feature="All"))]   
                            Quantities::$quantity(x) => format!("{} {}", x.value(), x.unit.abbreviation()),
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
                /// a given arbitrary unit to the underlying `Quantity` that is used to perform unit conversion calculations.
                /// 
                pub enum Units
                {                               
                    $(
                        #[cfg(any(feature = "" $quantity, feature="All"))]    
                        #[cfg_attr(feature="utoipa", schema(title = "" [<$quantity Unit>]))]                                                     
                        $quantity([<$quantity Unit>]),
                    )+                
                }
                
                impl From<Units> for $crate::units_base::UnitDefinition
                {
                    fn from(value: Units) -> Self {
                        match value
                        {
                            $(
                                #[cfg(any(feature = "" $quantity, feature="All"))]     
                                Units::$quantity(x)=> UnitDefinition{multiplier: x.multiplier(), base: *[<$quantity:snake>]::[<$quantity:upper _UNIT_BASE>]},
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
                /// Convert a given 'value' expressed in the given `Units` into a convertible `Quantity`
                /// 
                pub fn to_quantity(&self, value: f64) -> $crate::quantity::Quantity
                {
                    let unit: UnitDefinition = (*self).into();
                    $crate::quantity::Quantity { unit, value }
                }
                paste!{
     
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
            use $crate::{create_multivalue_quantities_vec_enum, create_multivalue_quantities_array_enum};
            create_multivalue_quantities_vec_enum!($($quantity),+);
            create_multivalue_quantities_array_enum!($($quantity),+);
            
        }
    };
}
