use nalgebra::allocator::Allocator;
use nalgebra::{
    DefaultAllocator, DimName, Point2, RealField, Scalar, Vector1, Vector2, VectorN, U1, U2,
};
use std::marker::PhantomData;

pub trait ReferenceFiniteElementAllocator<T, ReferenceDim, NodalDim>:
    Allocator<T, U1, NodalDim> + Allocator<T, ReferenceDim, U1>
where
    T: Scalar,
    ReferenceDim: DimName,
    NodalDim: DimName,
{
}

pub trait FiniteElementAllocator<T, GeometryDim, ReferenceDim, NodalDim>:
    ReferenceFiniteElementAllocator<T, ReferenceDim, NodalDim>
    + Allocator<T, GeometryDim>
    + Allocator<T, NodalDim, U1>
    + Allocator<T, GeometryDim, ReferenceDim>
where
    T: Scalar,
    GeometryDim: DimName,
    ReferenceDim: DimName,
    NodalDim: DimName,
{
}

impl<T, ReferenceDim, NodalDim> ReferenceFiniteElementAllocator<T, ReferenceDim, NodalDim>
    for DefaultAllocator
where
    T: Scalar,
    ReferenceDim: DimName,
    NodalDim: DimName,
    DefaultAllocator: Allocator<T, U1, NodalDim> + Allocator<T, ReferenceDim, U1>,
{
}

impl<T, GeometryDim, ReferenceDim, NodalDim>
    FiniteElementAllocator<T, GeometryDim, ReferenceDim, NodalDim> for DefaultAllocator
where
    T: Scalar,
    GeometryDim: DimName,
    NodalDim: DimName,
    ReferenceDim: DimName,
    DefaultAllocator: ReferenceFiniteElementAllocator<T, ReferenceDim, NodalDim>
        + Allocator<T, GeometryDim>
        + Allocator<T, U1, NodalDim>
        + Allocator<T, NodalDim, U1>
        + Allocator<T, GeometryDim, ReferenceDim>,
{
}

pub trait ReferenceFiniteElement<T, ReferenceDim>
where
    T: Scalar,
    ReferenceDim: DimName,
    DefaultAllocator: ReferenceFiniteElementAllocator<T, ReferenceDim, Self::NodalDim>,
{
    type NodalDim: DimName;
}

pub trait FiniteElement<T, GeometryDim, ReferenceDim = GeometryDim>:
    ReferenceFiniteElement<T, ReferenceDim>
where
    T: Scalar,
    GeometryDim: DimName,
    ReferenceDim: DimName,
    DefaultAllocator: FiniteElementAllocator<T, GeometryDim, ReferenceDim, Self::NodalDim>,
{
    fn map_reference_coords(
        &self,
        reference_coords: &VectorN<T, ReferenceDim>,
    ) -> VectorN<T, GeometryDim>;
}

pub trait ElementConnectivity<T, GeometryDim, ReferenceDim = GeometryDim>
where
    T: Scalar,
    GeometryDim: DimName,
    ReferenceDim: DimName,
    DefaultAllocator: FiniteElementAllocator<T, GeometryDim, ReferenceDim, Self::NodalDim>,
{
    type Element: FiniteElement<T, GeometryDim, ReferenceDim, NodalDim = Self::NodalDim>;
    type NodalDim: DimName;
}

pub struct Edge2dElement<T> {
    segment: LineSegment2d<T>,
}

impl<T> From<LineSegment2d<T>> for Edge2dElement<T>
where
    T: Scalar,
{
    fn from(_segment: LineSegment2d<T>) -> Self {
        loop {}
    }
}

impl<T> ReferenceFiniteElement<T, U1> for Edge2dElement<T>
where
    T: RealField,
{
    type NodalDim = U2;
}

impl<T> FiniteElement<T, U2, U1> for Edge2dElement<T>
where
    T: RealField,
{
    #[allow(non_snake_case)]
    fn map_reference_coords(&self, _xi: &Vector1<T>) -> Vector2<T> {
        loop {}
    }
}

pub trait GeometricFiniteElementSpace<T>
where
    T: Scalar,
    DefaultAllocator:
        FiniteElementAllocator<T, Self::GeometryDim, Self::ReferenceDim, Self::NodalDim>,
{
    type NodalDim: DimName;
    type GeometryDim: DimName;
    type ReferenceDim: DimName;
    type Connectivity: ElementConnectivity<
        T,
        Self::GeometryDim,
        Self::ReferenceDim,
        NodalDim = Self::NodalDim,
    >;
}

pub struct LineSegment2d<T> {
    marker: PhantomData<T>,
}

impl<T> LineSegment2d<T>
where
    T: Scalar,
{
    pub fn new(_from: Point2<T>, _to: Point2<T>) -> Self {
        loop {}
    }
}

pub struct Polygon<T> {
    marker: PhantomData<T>,
}

impl<T> Polygon<T> {
    pub fn get_edge(&self, _index: usize) -> LineSegment2d<T> {
        loop {}
    }
}

fn problematic_function<Space>(_volume_space: &Space, material_surface: &Polygon<f64>)
where
    Space: GeometricFiniteElementSpace<f64, GeometryDim = U2, ReferenceDim = U2>,
    // TODO: I have no idea why this bound is required. I think that it *should* be implied by
    // the bounds on the associated type in `GeometricFiniteElementSpace`.
    // Possible typechecker bug...? Or am I just misunderstanding how this is supposed to work?
    // Should try to reduce this to a minimal test case somehow?
    Space::Connectivity: ElementConnectivity<f64, U2, U2, NodalDim = Space::NodalDim>,
    DefaultAllocator: FiniteElementAllocator<f64, U2, U2, Space::NodalDim>,
    // Including this trait bound (which should already be satisfied)
    // seems to resolve the ICE
    //            + FiniteElementAllocator<f64, U2, U2, U2>
{
    let material_segment = material_surface.get_edge(0);
    let material_surface_element = Edge2dElement::from(material_segment);

    let eta = Vector1::new(0.0);
    // This is the source of one ICE
    let _: Point2<f64> = material_surface_element.map_reference_coords(&eta).into();

    // This is the source of another: they appear possibly to be due to the same bug,
    // but I can't say for certain, as certain manipulations of the above code only cause
    // one of them to fail, and not the other.
    material_surface_element
        .map_reference_coords(&eta)
        .magnitude();
}
