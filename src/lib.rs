use nalgebra::{allocator::Allocator, DefaultAllocator, DimName, Point2, Vector2, VectorN, U1, U2};

pub trait FiniteElementAllocator<GeometryDim, NodalDim>:
    Allocator<f64, GeometryDim> + Allocator<f64, NodalDim, U1>
where
    GeometryDim: DimName,
    NodalDim: DimName,
{ }

impl<GeometryDim, NodalDim> FiniteElementAllocator<GeometryDim, NodalDim> for DefaultAllocator
where
    GeometryDim: DimName,
    NodalDim: DimName,
    DefaultAllocator:
      Allocator<f64, GeometryDim>
      + Allocator<f64, U1, NodalDim>
      + Allocator<f64, NodalDim, U1>,
{ }

pub trait ReferenceFiniteElement {
    type NodalDim: DimName;
}

pub trait FiniteElement<GeometryDim>: ReferenceFiniteElement
where
    GeometryDim: DimName,
    DefaultAllocator: FiniteElementAllocator<GeometryDim, Self::NodalDim>,
{
    fn map_reference_coords(&self) -> VectorN<f64, GeometryDim>;
}

pub trait ElementConnectivity<GeometryDim>
where
    GeometryDim: DimName,
    DefaultAllocator: FiniteElementAllocator<GeometryDim, Self::NodalDim>,
{
    type Element: FiniteElement<GeometryDim, NodalDim = Self::NodalDim>;
    type NodalDim: DimName;
}

pub struct Edge2dElement;

impl ReferenceFiniteElement for Edge2dElement {
    type NodalDim = U1;
}

impl FiniteElement<U2> for Edge2dElement {
    #[allow(non_snake_case)]
    fn map_reference_coords(&self) -> Vector2<f64> {
        loop {}
    }
}

pub trait GeometricFiniteElementSpace
where
    DefaultAllocator: FiniteElementAllocator<Self::GeometryDim, Self::NodalDim>,
{
    type NodalDim: DimName;
    type GeometryDim: DimName;
    type Connectivity: ElementConnectivity<Self::GeometryDim, NodalDim = Self::NodalDim>;
}

pub fn problematic_function<Space>(material_surface_element: &Edge2dElement)
where
    Space: GeometricFiniteElementSpace<GeometryDim = U1/*, NodalDim = U1*/>,
    // TODO: I have no idea why this bound is required. I think that it *should* be implied by
    // the bounds on the associated type in `GeometricFiniteElementSpace`.
    // Possible typechecker bug...? Or am I just misunderstanding how this is supposed to work?
    // Should try to reduce this to a minimal test case somehow?
    Space::Connectivity: ElementConnectivity<U1, NodalDim = Space::NodalDim>,
    DefaultAllocator: FiniteElementAllocator<U1, Space::NodalDim>,
    // Including this trait bound (which should already be satisfied) seems to resolve the ICE
    //+ FiniteElementAllocator<U1, U1>
    //
    // This has the same effect as telling the compiler Space::NodalDim = U1; which was not already
    // known to be satisfied.
{
    // This is the source of one ICE
    let _: Point2<f64> = material_surface_element.map_reference_coords().into();

    // This is the source of another: they appear possibly to be due to the same bug,
    // but I can't say for certain, as certain manipulations of the above code only cause
    // one of them to fail, and not the other.
    material_surface_element.map_reference_coords().magnitude();
}
