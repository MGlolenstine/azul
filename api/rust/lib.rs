// Copyright 2017 Maps4Print Einzelunternehmung
// 
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
// 
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
// TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
// SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
// #![no_std]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/maps4print/azul/master/assets/images/azul_logo_full_min.svg.png",
    html_favicon_url = "https://raw.githubusercontent.com/maps4print/azul/master/assets/images/favicon.ico",
)]

//! Auto-generated public Rust API for the Azul GUI toolkit version " + version + "

extern crate alloc;

/// Module to re-export common structs (`App`, `AppConfig`, `Css`, `Dom`, `WindowCreateOptions`, `RefAny`, `LayoutInfo`)
pub mod prelude {
    pub use crate::{
        app::{App, AppConfig},
        css::Css,
        dom::Dom,
        style::StyledDom,
        window::WindowCreateOptions,
        callbacks::{RefAny, LayoutInfo},
    };
}

mod dll {
    use core::ffi::c_void;

    impl AzString {
        #[inline]
        pub fn as_str(&self) -> &str {
            unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
        }
        #[inline]
        pub fn as_bytes(&self) -> &[u8] {
            unsafe { core::slice::from_raw_parts(self.vec.ptr, self.vec.len) }
        }
    }

    impl ::core::fmt::Debug for AzCallback                   { fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::core::fmt::Debug for AzLayoutCallback             { fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::core::fmt::Debug for AzGlCallback                 { fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::core::fmt::Debug for AzIFrameCallback             { fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::core::fmt::Debug for AzTimerCallback              { fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::core::fmt::Debug for AzWriteBackCallback          { fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::core::fmt::Debug for AzRefAny                     {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "RefAny {{\r\n")?;
            write!(f, "    is_dead: {:?}\r\n", &self.is_dead)?;
            write!(f, "    sharing_info: {:?}\r\n", &self.sharing_info)?;
            write!(f, "}}\r\n")?;
            Ok(())
        }
    }

    impl PartialEq for AzCallback { fn eq(&self, rhs: &Self) -> bool { (self.cb as usize).eq(&(rhs.cb as usize)) } }
    impl PartialEq for AzLayoutCallback { fn eq(&self, rhs: &Self) -> bool { (self.cb as usize).eq(&(rhs.cb as usize)) } }
    impl PartialEq for AzGlCallback { fn eq(&self, rhs: &Self) -> bool { (self.cb as usize).eq(&(rhs.cb as usize)) } }
    impl PartialEq for AzIFrameCallback { fn eq(&self, rhs: &Self) -> bool { (self.cb as usize).eq(&(rhs.cb as usize)) } }
    impl PartialEq for AzTimerCallback { fn eq(&self, rhs: &Self) -> bool { (self.cb as usize).eq(&(rhs.cb as usize)) } }
    impl PartialEq for AzWriteBackCallback { fn eq(&self, rhs: &Self) -> bool { (self.cb as usize).eq(&(rhs.cb as usize)) } }
    impl PartialEq for AzRefAny { fn eq(&self, rhs: &Self) -> bool { (self._internal_ptr as usize).eq(&(rhs._internal_ptr as usize)) } }

    impl PartialOrd for AzCallback { fn partial_cmp(&self, rhs: &Self) -> Option<::core::cmp::Ordering> { (self.cb as usize).partial_cmp(&(rhs.cb as usize)) } }
    impl PartialOrd for AzLayoutCallback { fn partial_cmp(&self, rhs: &Self) -> Option<::core::cmp::Ordering> { (self.cb as usize).partial_cmp(&(rhs.cb as usize)) } }
    impl PartialOrd for AzGlCallback { fn partial_cmp(&self, rhs: &Self) -> Option<::core::cmp::Ordering> { (self.cb as usize).partial_cmp(&(rhs.cb as usize)) } }
    impl PartialOrd for AzIFrameCallback { fn partial_cmp(&self, rhs: &Self) -> Option<::core::cmp::Ordering> { (self.cb as usize).partial_cmp(&(rhs.cb as usize)) } }
    impl PartialOrd for AzTimerCallback { fn partial_cmp(&self, rhs: &Self) -> Option<::core::cmp::Ordering> { (self.cb as usize).partial_cmp(&(rhs.cb as usize)) } }
    impl PartialOrd for AzWriteBackCallback { fn partial_cmp(&self, rhs: &Self) -> Option<::core::cmp::Ordering> { (self.cb as usize).partial_cmp(&(rhs.cb as usize)) } }
    impl PartialOrd for AzRefAny { fn partial_cmp(&self, rhs: &Self) -> Option<::core::cmp::Ordering> { (self._internal_ptr as usize).partial_cmp(&(rhs._internal_ptr as usize)) } }

    impl ::core::fmt::Debug for AzRefCount {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let ptr = unsafe { &*self.ptr };
            write!(f, "RefAnyRefCount {{\r\n")?;
            write!(f, "    num_copies: {}\r\n", ptr.num_copies)?;
            write!(f, "    num_refs: {}\r\n", ptr.num_refs)?;
            write!(f, "    num_mutable_refs: {}\r\n", ptr.num_mutable_refs)?;
            write!(f, "    _internal_ptr: 0x{:x}\r\n", ptr._internal_ptr as usize)?;
            write!(f, "    _internal_len: {}\r\n", ptr._internal_len)?;
            write!(f, "    _internal_layout_size: {}\r\n", ptr._internal_layout_size)?;
            write!(f, "    _internal_layout_align: {}\r\n", ptr._internal_layout_align)?;
            write!(f, "    type_name: \"{}\"\r\n", ptr.type_name.as_str())?;
            write!(f, "    type_id: {}\r\n", ptr.type_id)?;
            write!(f, "    custom_destructor: 0x{:x}\r\n", ptr.custom_destructor as usize)?;
            write!(f, "}}\r\n")?;
            Ok(())
        }
    }

    #[cfg(not(feature = "link_static"))]    mod __structs {
    /// `AzDomVecDestructorType` struct
    pub type AzDomVecDestructorType = extern "C" fn(&mut AzDomVec);
    /// `AzIdOrClassVecDestructorType` struct
    pub type AzIdOrClassVecDestructorType = extern "C" fn(&mut AzIdOrClassVec);
    /// `AzNodeDataInlineCssPropertyVecDestructorType` struct
    pub type AzNodeDataInlineCssPropertyVecDestructorType = extern "C" fn(&mut AzNodeDataInlineCssPropertyVec);
    /// `AzStyleBackgroundContentVecDestructorType` struct
    pub type AzStyleBackgroundContentVecDestructorType = extern "C" fn(&mut AzStyleBackgroundContentVec);
    /// `AzStyleBackgroundPositionVecDestructorType` struct
    pub type AzStyleBackgroundPositionVecDestructorType = extern "C" fn(&mut AzStyleBackgroundPositionVec);
    /// `AzStyleBackgroundRepeatVecDestructorType` struct
    pub type AzStyleBackgroundRepeatVecDestructorType = extern "C" fn(&mut AzStyleBackgroundRepeatVec);
    /// `AzStyleBackgroundSizeVecDestructorType` struct
    pub type AzStyleBackgroundSizeVecDestructorType = extern "C" fn(&mut AzStyleBackgroundSizeVec);
    /// `AzStyleTransformVecDestructorType` struct
    pub type AzStyleTransformVecDestructorType = extern "C" fn(&mut AzStyleTransformVec);
    /// `AzCssPropertyVecDestructorType` struct
    pub type AzCssPropertyVecDestructorType = extern "C" fn(&mut AzCssPropertyVec);
    /// `AzSvgMultiPolygonVecDestructorType` struct
    pub type AzSvgMultiPolygonVecDestructorType = extern "C" fn(&mut AzSvgMultiPolygonVec);
    /// `AzSvgPathVecDestructorType` struct
    pub type AzSvgPathVecDestructorType = extern "C" fn(&mut AzSvgPathVec);
    /// `AzVertexAttributeVecDestructorType` struct
    pub type AzVertexAttributeVecDestructorType = extern "C" fn(&mut AzVertexAttributeVec);
    /// `AzSvgPathElementVecDestructorType` struct
    pub type AzSvgPathElementVecDestructorType = extern "C" fn(&mut AzSvgPathElementVec);
    /// `AzSvgVertexVecDestructorType` struct
    pub type AzSvgVertexVecDestructorType = extern "C" fn(&mut AzSvgVertexVec);
    /// `AzU32VecDestructorType` struct
    pub type AzU32VecDestructorType = extern "C" fn(&mut AzU32Vec);
    /// `AzXWindowTypeVecDestructorType` struct
    pub type AzXWindowTypeVecDestructorType = extern "C" fn(&mut AzXWindowTypeVec);
    /// `AzVirtualKeyCodeVecDestructorType` struct
    pub type AzVirtualKeyCodeVecDestructorType = extern "C" fn(&mut AzVirtualKeyCodeVec);
    /// `AzCascadeInfoVecDestructorType` struct
    pub type AzCascadeInfoVecDestructorType = extern "C" fn(&mut AzCascadeInfoVec);
    /// `AzScanCodeVecDestructorType` struct
    pub type AzScanCodeVecDestructorType = extern "C" fn(&mut AzScanCodeVec);
    /// `AzCssDeclarationVecDestructorType` struct
    pub type AzCssDeclarationVecDestructorType = extern "C" fn(&mut AzCssDeclarationVec);
    /// `AzCssPathSelectorVecDestructorType` struct
    pub type AzCssPathSelectorVecDestructorType = extern "C" fn(&mut AzCssPathSelectorVec);
    /// `AzStylesheetVecDestructorType` struct
    pub type AzStylesheetVecDestructorType = extern "C" fn(&mut AzStylesheetVec);
    /// `AzCssRuleBlockVecDestructorType` struct
    pub type AzCssRuleBlockVecDestructorType = extern "C" fn(&mut AzCssRuleBlockVec);
    /// `AzU8VecDestructorType` struct
    pub type AzU8VecDestructorType = extern "C" fn(&mut AzU8Vec);
    /// `AzCallbackDataVecDestructorType` struct
    pub type AzCallbackDataVecDestructorType = extern "C" fn(&mut AzCallbackDataVec);
    /// `AzDebugMessageVecDestructorType` struct
    pub type AzDebugMessageVecDestructorType = extern "C" fn(&mut AzDebugMessageVec);
    /// `AzGLuintVecDestructorType` struct
    pub type AzGLuintVecDestructorType = extern "C" fn(&mut AzGLuintVec);
    /// `AzGLintVecDestructorType` struct
    pub type AzGLintVecDestructorType = extern "C" fn(&mut AzGLintVec);
    /// `AzStringVecDestructorType` struct
    pub type AzStringVecDestructorType = extern "C" fn(&mut AzStringVec);
    /// `AzStringPairVecDestructorType` struct
    pub type AzStringPairVecDestructorType = extern "C" fn(&mut AzStringPairVec);
    /// `AzLinearColorStopVecDestructorType` struct
    pub type AzLinearColorStopVecDestructorType = extern "C" fn(&mut AzLinearColorStopVec);
    /// `AzRadialColorStopVecDestructorType` struct
    pub type AzRadialColorStopVecDestructorType = extern "C" fn(&mut AzRadialColorStopVec);
    /// `AzNodeIdVecDestructorType` struct
    pub type AzNodeIdVecDestructorType = extern "C" fn(&mut AzNodeIdVec);
    /// `AzNodeVecDestructorType` struct
    pub type AzNodeVecDestructorType = extern "C" fn(&mut AzNodeVec);
    /// `AzStyledNodeVecDestructorType` struct
    pub type AzStyledNodeVecDestructorType = extern "C" fn(&mut AzStyledNodeVec);
    /// `AzTagIdsToNodeIdsMappingVecDestructorType` struct
    pub type AzTagIdsToNodeIdsMappingVecDestructorType = extern "C" fn(&mut AzTagIdsToNodeIdsMappingVec);
    /// `AzParentWithNodeDepthVecDestructorType` struct
    pub type AzParentWithNodeDepthVecDestructorType = extern "C" fn(&mut AzParentWithNodeDepthVec);
    /// `AzNodeDataVecDestructorType` struct
    pub type AzNodeDataVecDestructorType = extern "C" fn(&mut AzNodeDataVec);
    /// Re-export of rust-allocated (stack based) `InstantPtr` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzInstantPtr {
        pub(crate) ptr: *const c_void,
        pub clone_fn: AzInstantPtrCloneFn,
        pub destructor: AzInstantPtrDestructorFn,
    }
    /// `AzInstantPtrCloneFnType` struct
    pub type AzInstantPtrCloneFnType = extern "C" fn(&c_void) -> AzInstantPtr;
    /// `AzInstantPtrDestructorFnType` struct
    pub type AzInstantPtrDestructorFnType = extern "C" fn(&mut c_void);
    /// Re-export of rust-allocated (stack based) `AppLogLevel` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzAppLogLevel {
        Off,
        Error,
        Warn,
        Info,
        Debug,
        Trace,
    }
    /// Main application class
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzApp {
        pub(crate) ptr: *const c_void,
    }
    /// `AzLayoutCallbackType` struct
    pub type AzLayoutCallbackType = extern "C" fn(&mut AzRefAny, AzLayoutInfo) -> AzStyledDom;
    /// `AzCallbackType` struct
    pub type AzCallbackType = extern "C" fn(&mut AzRefAny, AzCallbackInfo) -> AzUpdateScreen;
    /// Specifies if the screen should be updated after the callback function has returned
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzUpdateScreen {
        DoNothing,
        RegenerateStyledDomForCurrentWindow,
        RegenerateStyledDomForAllWindows,
    }
    /// `AzIFrameCallbackType` struct
    pub type AzIFrameCallbackType = extern "C" fn(&mut AzRefAny, AzIFrameCallbackInfo) -> AzIFrameCallbackReturn;
    /// `AzGlCallbackType` struct
    pub type AzGlCallbackType = extern "C" fn(&mut AzRefAny, AzGlCallbackInfo) -> AzGlCallbackReturn;
    /// `AzTimerCallbackType` struct
    pub type AzTimerCallbackType = extern "C" fn(&mut AzRefAny, &mut AzRefAny, AzTimerCallbackInfo) -> AzTimerCallbackReturn;
    /// `AzWriteBackCallbackType` struct
    pub type AzWriteBackCallbackType = extern "C" fn(&mut AzRefAny, AzRefAny, AzCallbackInfo) -> AzUpdateScreen;
    /// `AzThreadCallbackType` struct
    pub type AzThreadCallbackType = extern "C" fn(AzRefAny, AzThreadSender, AzThreadReceiver);
    /// `AzRefAnyDestructorType` struct
    pub type AzRefAnyDestructorType = extern "C" fn(&mut c_void);
    /// Re-export of rust-allocated (stack based) `RefCount` struct
    #[repr(C)]     pub struct AzRefCount {
        pub(crate) ptr: *const AzRefCountInner,
    }
    /// RefAny is a reference-counted, type-erased pointer, which stores a reference to a struct. `RefAny` can be up- and downcasted (this usually done via generics and can't be expressed in the Rust API)
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzRefAny {
        pub is_dead: bool,
        pub sharing_info: AzRefCount,
    }
    /// Re-export of rust-allocated (stack based) `NodeTypePath` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzNodeTypePath {
        Body,
        Div,
        Br,
        P,
        Img,
        Texture,
        IFrame,
    }
    /// Re-export of rust-allocated (stack based) `CssPropertyType` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzCssPropertyType {
        TextColor,
        FontSize,
        FontFamily,
        TextAlign,
        LetterSpacing,
        LineHeight,
        WordSpacing,
        TabWidth,
        Cursor,
        Display,
        Float,
        BoxSizing,
        Width,
        Height,
        MinWidth,
        MinHeight,
        MaxWidth,
        MaxHeight,
        Position,
        Top,
        Right,
        Left,
        Bottom,
        FlexWrap,
        FlexDirection,
        FlexGrow,
        FlexShrink,
        JustifyContent,
        AlignItems,
        AlignContent,
        OverflowX,
        OverflowY,
        PaddingTop,
        PaddingLeft,
        PaddingRight,
        PaddingBottom,
        MarginTop,
        MarginLeft,
        MarginRight,
        MarginBottom,
        Background,
        BackgroundImage,
        BackgroundColor,
        BackgroundPosition,
        BackgroundSize,
        BackgroundRepeat,
        BorderTopLeftRadius,
        BorderTopRightRadius,
        BorderBottomLeftRadius,
        BorderBottomRightRadius,
        BorderTopColor,
        BorderRightColor,
        BorderLeftColor,
        BorderBottomColor,
        BorderTopStyle,
        BorderRightStyle,
        BorderLeftStyle,
        BorderBottomStyle,
        BorderTopWidth,
        BorderRightWidth,
        BorderLeftWidth,
        BorderBottomWidth,
        BoxShadowLeft,
        BoxShadowRight,
        BoxShadowTop,
        BoxShadowBottom,
        ScrollbarStyle,
        Opacity,
        Transform,
        PerspectiveOrigin,
        TransformOrigin,
        BackfaceVisibility,
    }
    /// Re-export of rust-allocated (stack based) `SizeMetric` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzSizeMetric {
        Px,
        Pt,
        Em,
        Percent,
    }
    /// Re-export of rust-allocated (stack based) `BoxShadowClipMode` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzBoxShadowClipMode {
        Outset,
        Inset,
    }
    /// Re-export of rust-allocated (stack based) `LayoutAlignContent` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutAlignContent {
        Stretch,
        Center,
        Start,
        End,
        SpaceBetween,
        SpaceAround,
    }
    /// Re-export of rust-allocated (stack based) `LayoutAlignItems` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutAlignItems {
        Stretch,
        Center,
        FlexStart,
        FlexEnd,
    }
    /// Re-export of rust-allocated (stack based) `LayoutBoxSizing` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutBoxSizing {
        ContentBox,
        BorderBox,
    }
    /// Re-export of rust-allocated (stack based) `LayoutFlexDirection` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutFlexDirection {
        Row,
        RowReverse,
        Column,
        ColumnReverse,
    }
    /// Re-export of rust-allocated (stack based) `LayoutDisplay` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutDisplay {
        Flex,
        Block,
        InlineBlock,
    }
    /// Re-export of rust-allocated (stack based) `LayoutFloat` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutFloat {
        Left,
        Right,
    }
    /// Re-export of rust-allocated (stack based) `LayoutJustifyContent` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutJustifyContent {
        Start,
        End,
        Center,
        SpaceBetween,
        SpaceAround,
        SpaceEvenly,
    }
    /// Re-export of rust-allocated (stack based) `LayoutPosition` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutPosition {
        Static,
        Relative,
        Absolute,
        Fixed,
    }
    /// Re-export of rust-allocated (stack based) `LayoutFlexWrap` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutFlexWrap {
        Wrap,
        NoWrap,
    }
    /// Re-export of rust-allocated (stack based) `LayoutOverflow` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutOverflow {
        Scroll,
        Auto,
        Hidden,
        Visible,
    }
    /// Re-export of rust-allocated (stack based) `AngleMetric` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzAngleMetric {
        Degree,
        Radians,
        Grad,
        Turn,
        Percent,
    }
    /// Re-export of rust-allocated (stack based) `DirectionCorner` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzDirectionCorner {
        Right,
        Left,
        Top,
        Bottom,
        TopRight,
        TopLeft,
        BottomRight,
        BottomLeft,
    }
    /// Re-export of rust-allocated (stack based) `ExtendMode` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzExtendMode {
        Clamp,
        Repeat,
    }
    /// Re-export of rust-allocated (stack based) `Shape` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzShape {
        Ellipse,
        Circle,
    }
    /// Re-export of rust-allocated (stack based) `RadialGradientSize` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzRadialGradientSize {
        ClosestSide,
        ClosestCorner,
        FarthestSide,
        FarthestCorner,
    }
    /// Re-export of rust-allocated (stack based) `StyleBackgroundRepeat` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleBackgroundRepeat {
        NoRepeat,
        Repeat,
        RepeatX,
        RepeatY,
    }
    /// Re-export of rust-allocated (stack based) `BorderStyle` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzBorderStyle {
        None,
        Solid,
        Double,
        Dotted,
        Dashed,
        Hidden,
        Groove,
        Ridge,
        Inset,
        Outset,
    }
    /// Re-export of rust-allocated (stack based) `StyleCursor` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleCursor {
        Alias,
        AllScroll,
        Cell,
        ColResize,
        ContextMenu,
        Copy,
        Crosshair,
        Default,
        EResize,
        EwResize,
        Grab,
        Grabbing,
        Help,
        Move,
        NResize,
        NsResize,
        NeswResize,
        NwseResize,
        Pointer,
        Progress,
        RowResize,
        SResize,
        SeResize,
        Text,
        Unset,
        VerticalText,
        WResize,
        Wait,
        ZoomIn,
        ZoomOut,
    }
    /// Re-export of rust-allocated (stack based) `StyleBackfaceVisibility` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleBackfaceVisibility {
        Hidden,
        Visible,
    }
    /// Re-export of rust-allocated (stack based) `StyleTextAlignmentHorz` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleTextAlignmentHorz {
        Left,
        Center,
        Right,
    }
    /// Re-export of rust-allocated (stack based) `CssPropertyCache` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzCssPropertyCache {
        pub(crate) ptr: *mut c_void,
    }
    /// When to call a callback action - `On::MouseOver`, `On::MouseOut`, etc.
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOn {
        MouseOver,
        MouseDown,
        LeftMouseDown,
        MiddleMouseDown,
        RightMouseDown,
        MouseUp,
        LeftMouseUp,
        MiddleMouseUp,
        RightMouseUp,
        MouseEnter,
        MouseLeave,
        Scroll,
        TextInput,
        VirtualKeyDown,
        VirtualKeyUp,
        HoveredFile,
        DroppedFile,
        HoveredFileCancelled,
        FocusReceived,
        FocusLost,
    }
    /// Re-export of rust-allocated (stack based) `HoverEventFilter` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzHoverEventFilter {
        MouseOver,
        MouseDown,
        LeftMouseDown,
        RightMouseDown,
        MiddleMouseDown,
        MouseUp,
        LeftMouseUp,
        RightMouseUp,
        MiddleMouseUp,
        MouseEnter,
        MouseLeave,
        Scroll,
        ScrollStart,
        ScrollEnd,
        TextInput,
        VirtualKeyDown,
        VirtualKeyUp,
        HoveredFile,
        DroppedFile,
        HoveredFileCancelled,
        TouchStart,
        TouchMove,
        TouchEnd,
        TouchCancel,
    }
    /// Re-export of rust-allocated (stack based) `FocusEventFilter` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzFocusEventFilter {
        MouseOver,
        MouseDown,
        LeftMouseDown,
        RightMouseDown,
        MiddleMouseDown,
        MouseUp,
        LeftMouseUp,
        RightMouseUp,
        MiddleMouseUp,
        MouseEnter,
        MouseLeave,
        Scroll,
        ScrollStart,
        ScrollEnd,
        TextInput,
        VirtualKeyDown,
        VirtualKeyUp,
        FocusReceived,
        FocusLost,
    }
    /// Re-export of rust-allocated (stack based) `WindowEventFilter` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzWindowEventFilter {
        MouseOver,
        MouseDown,
        LeftMouseDown,
        RightMouseDown,
        MiddleMouseDown,
        MouseUp,
        LeftMouseUp,
        RightMouseUp,
        MiddleMouseUp,
        MouseEnter,
        MouseLeave,
        Scroll,
        ScrollStart,
        ScrollEnd,
        TextInput,
        VirtualKeyDown,
        VirtualKeyUp,
        HoveredFile,
        DroppedFile,
        HoveredFileCancelled,
        Resized,
        Moved,
        TouchStart,
        TouchMove,
        TouchEnd,
        TouchCancel,
        FocusReceived,
        FocusLost,
        CloseRequested,
        ThemeChanged,
    }
    /// Re-export of rust-allocated (stack based) `ComponentEventFilter` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzComponentEventFilter {
        AfterMount,
        BeforeUnmount,
        NodeResized,
    }
    /// Re-export of rust-allocated (stack based) `ApplicationEventFilter` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzApplicationEventFilter {
        DeviceConnected,
        DeviceDisconnected,
    }
    /// Re-export of rust-allocated (stack based) `VertexAttributeType` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzVertexAttributeType {
        Float,
        Double,
        UnsignedByte,
        UnsignedShort,
        UnsignedInt,
    }
    /// Re-export of rust-allocated (stack based) `IndexBufferFormat` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzIndexBufferFormat {
        Points,
        Lines,
        LineStrip,
        Triangles,
        TriangleStrip,
        TriangleFan,
    }
    /// Re-export of rust-allocated (stack based) `GlType` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzGlType {
        Gl,
        Gles,
    }
    /// C-ABI stable reexport of `*const gleam::gl::GLsync`
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzGLsyncPtr {
        pub(crate) ptr: *const c_void,
    }
    /// Re-export of rust-allocated (stack based) `GlContextPtr` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzGlContextPtr {
        pub(crate) ptr: *const c_void,
        pub renderer_type: AzRendererType,
    }
    /// Re-export of rust-allocated (stack based) `Texture` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzTexture {
        pub texture_id: u32,
        pub format: AzRawImageFormat,
        pub flags: AzTextureFlags,
        pub size: AzPhysicalSizeU32,
        pub gl_context: AzGlContextPtr,
    }
    /// Re-export of rust-allocated (stack based) `RawImageFormat` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzRawImageFormat {
        R8,
        R16,
        RG16,
        BGRA8,
        RGBAF32,
        RG8,
        RGBAI32,
        RGBA8,
    }
    /// Re-export of rust-allocated (stack based) `SvgLineCap` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzSvgLineCap {
        Butt,
        Square,
        Round,
    }
    /// Re-export of rust-allocated (stack based) `ShapeRendering` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzShapeRendering {
        OptimizeSpeed,
        CrispEdges,
        GeometricPrecision,
    }
    /// Re-export of rust-allocated (stack based) `TextRendering` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzTextRendering {
        OptimizeSpeed,
        OptimizeLegibility,
        GeometricPrecision,
    }
    /// Re-export of rust-allocated (stack based) `ImageRendering` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzImageRendering {
        OptimizeQuality,
        OptimizeSpeed,
    }
    /// Re-export of rust-allocated (stack based) `FontDatabase` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzFontDatabase {
        Empty,
        System,
    }
    /// Re-export of rust-allocated (stack based) `Svg` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzSvg {
        pub(crate) ptr: *mut c_void,
    }
    /// Re-export of rust-allocated (stack based) `SvgXmlNode` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzSvgXmlNode {
        pub(crate) ptr: *mut c_void,
    }
    /// Re-export of rust-allocated (stack based) `SvgLineJoin` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzSvgLineJoin {
        Miter,
        MiterClip,
        Round,
        Bevel,
    }
    /// Should a timer terminate or not - used to remove active timers
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzTerminateTimer {
        Terminate,
        Continue,
    }
    /// Re-export of rust-allocated (stack based) `ThreadSender` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzThreadSender {
        pub(crate) ptr: *mut c_void,
        pub send_fn: AzThreadSendFn,
        pub destructor: AzThreadSenderDestructorFn,
    }
    /// Re-export of rust-allocated (stack based) `ThreadReceiver` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzThreadReceiver {
        pub(crate) ptr: *mut c_void,
        pub recv_fn: AzThreadRecvFn,
        pub destructor: AzThreadReceiverDestructorFn,
    }
    /// Re-export of rust-allocated (stack based) `ThreadSendMsg` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzThreadSendMsg {
        TerminateThread,
        Tick,
    }
    /// `AzCreateThreadFnType` struct
    pub type AzCreateThreadFnType = extern "C" fn(AzRefAny, AzRefAny, AzThreadCallbackType) -> AzThread;
    /// `AzGetSystemTimeFnType` struct
    pub type AzGetSystemTimeFnType = extern "C" fn() -> AzInstant;
    /// `AzCheckThreadFinishedFnType` struct
    pub type AzCheckThreadFinishedFnType = extern "C" fn(&c_void) -> bool;
    /// `AzLibrarySendThreadMsgFnType` struct
    pub type AzLibrarySendThreadMsgFnType = extern "C" fn(&mut c_void, AzThreadSendMsg) -> bool;
    /// `AzLibraryReceiveThreadMsgFnType` struct
    pub type AzLibraryReceiveThreadMsgFnType = extern "C" fn(&mut c_void) -> AzOptionThreadReceiveMsg;
    /// `AzThreadRecvFnType` struct
    pub type AzThreadRecvFnType = extern "C" fn(&mut c_void) -> AzOptionThreadSendMsg;
    /// `AzThreadSendFnType` struct
    pub type AzThreadSendFnType = extern "C" fn(&mut c_void, AzThreadReceiveMsg) -> bool;
    /// `AzThreadDestructorFnType` struct
    pub type AzThreadDestructorFnType = extern "C" fn(&mut c_void, &mut c_void, &mut c_void, &mut c_void);
    /// `AzThreadReceiverDestructorFnType` struct
    pub type AzThreadReceiverDestructorFnType = extern "C" fn(&mut AzThreadReceiver);
    /// `AzThreadSenderDestructorFnType` struct
    pub type AzThreadSenderDestructorFnType = extern "C" fn(&mut AzThreadSender);
    /// Re-export of rust-allocated (stack based) `Vsync` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzVsync {
        Enabled,
        Disabled,
    }
    /// Re-export of rust-allocated (stack based) `Srgb` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzSrgb {
        Enabled,
        Disabled,
    }
    /// Re-export of rust-allocated (stack based) `HwAcceleration` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzHwAcceleration {
        Enabled,
        Disabled,
    }
    /// Re-export of rust-allocated (stack based) `XWindowType` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzXWindowType {
        Desktop,
        Dock,
        Toolbar,
        Menu,
        Utility,
        Splash,
        Dialog,
        DropdownMenu,
        PopupMenu,
        Tooltip,
        Notification,
        Combo,
        Dnd,
        Normal,
    }
    /// Re-export of rust-allocated (stack based) `VirtualKeyCode` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzVirtualKeyCode {
        Key1,
        Key2,
        Key3,
        Key4,
        Key5,
        Key6,
        Key7,
        Key8,
        Key9,
        Key0,
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
        Escape,
        F1,
        F2,
        F3,
        F4,
        F5,
        F6,
        F7,
        F8,
        F9,
        F10,
        F11,
        F12,
        F13,
        F14,
        F15,
        F16,
        F17,
        F18,
        F19,
        F20,
        F21,
        F22,
        F23,
        F24,
        Snapshot,
        Scroll,
        Pause,
        Insert,
        Home,
        Delete,
        End,
        PageDown,
        PageUp,
        Left,
        Up,
        Right,
        Down,
        Back,
        Return,
        Space,
        Compose,
        Caret,
        Numlock,
        Numpad0,
        Numpad1,
        Numpad2,
        Numpad3,
        Numpad4,
        Numpad5,
        Numpad6,
        Numpad7,
        Numpad8,
        Numpad9,
        NumpadAdd,
        NumpadDivide,
        NumpadDecimal,
        NumpadComma,
        NumpadEnter,
        NumpadEquals,
        NumpadMultiply,
        NumpadSubtract,
        AbntC1,
        AbntC2,
        Apostrophe,
        Apps,
        Asterisk,
        At,
        Ax,
        Backslash,
        Calculator,
        Capital,
        Colon,
        Comma,
        Convert,
        Equals,
        Grave,
        Kana,
        Kanji,
        LAlt,
        LBracket,
        LControl,
        LShift,
        LWin,
        Mail,
        MediaSelect,
        MediaStop,
        Minus,
        Mute,
        MyComputer,
        NavigateForward,
        NavigateBackward,
        NextTrack,
        NoConvert,
        OEM102,
        Period,
        PlayPause,
        Plus,
        Power,
        PrevTrack,
        RAlt,
        RBracket,
        RControl,
        RShift,
        RWin,
        Semicolon,
        Slash,
        Sleep,
        Stop,
        Sysrq,
        Tab,
        Underline,
        Unlabeled,
        VolumeDown,
        VolumeUp,
        Wake,
        WebBack,
        WebFavorites,
        WebForward,
        WebHome,
        WebRefresh,
        WebSearch,
        WebStop,
        Yen,
        Copy,
        Paste,
        Cut,
    }
    /// Re-export of rust-allocated (stack based) `MouseCursorType` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzMouseCursorType {
        Default,
        Crosshair,
        Hand,
        Arrow,
        Move,
        Text,
        Wait,
        Help,
        Progress,
        NotAllowed,
        ContextMenu,
        Cell,
        VerticalText,
        Alias,
        Copy,
        NoDrop,
        Grab,
        Grabbing,
        AllScroll,
        ZoomIn,
        ZoomOut,
        EResize,
        NResize,
        NeResize,
        NwResize,
        SResize,
        SeResize,
        SwResize,
        WResize,
        EwResize,
        NsResize,
        NeswResize,
        NwseResize,
        ColResize,
        RowResize,
    }
    /// Re-export of rust-allocated (stack based) `RendererType` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzRendererType {
        Hardware,
        Software,
    }
    /// Re-export of rust-allocated (stack based) `FullScreenMode` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzFullScreenMode {
        SlowFullScreen,
        FastFullScreen,
        SlowWindowed,
        FastWindowed,
    }
    /// Re-export of rust-allocated (stack based) `WindowTheme` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzWindowTheme {
        DarkMode,
        LightMode,
    }
    /// Re-export of rust-allocated (stack based) `DomVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzDomVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzDomVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `IdOrClassVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzIdOrClassVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzIdOrClassVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `NodeDataInlineCssPropertyVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzNodeDataInlineCssPropertyVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzNodeDataInlineCssPropertyVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `StyleBackgroundContentVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzStyleBackgroundContentVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzStyleBackgroundContentVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `StyleBackgroundPositionVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzStyleBackgroundPositionVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzStyleBackgroundPositionVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `StyleBackgroundRepeatVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzStyleBackgroundRepeatVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzStyleBackgroundRepeatVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `StyleBackgroundSizeVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzStyleBackgroundSizeVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzStyleBackgroundSizeVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `StyleTransformVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzStyleTransformVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzStyleTransformVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `CssPropertyVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzCssPropertyVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzCssPropertyVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `SvgMultiPolygonVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzSvgMultiPolygonVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzSvgMultiPolygonVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `SvgPathVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzSvgPathVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzSvgPathVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `VertexAttributeVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzVertexAttributeVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzVertexAttributeVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `SvgPathElementVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzSvgPathElementVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzSvgPathElementVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `SvgVertexVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzSvgVertexVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzSvgVertexVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `U32VecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzU32VecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzU32VecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `XWindowTypeVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzXWindowTypeVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzXWindowTypeVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `VirtualKeyCodeVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzVirtualKeyCodeVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzVirtualKeyCodeVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `CascadeInfoVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzCascadeInfoVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzCascadeInfoVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `ScanCodeVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzScanCodeVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzScanCodeVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `CssDeclarationVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzCssDeclarationVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzCssDeclarationVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `CssPathSelectorVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzCssPathSelectorVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzCssPathSelectorVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `StylesheetVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzStylesheetVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzStylesheetVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `CssRuleBlockVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzCssRuleBlockVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzCssRuleBlockVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `U8VecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzU8VecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzU8VecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `CallbackDataVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzCallbackDataVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzCallbackDataVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `DebugMessageVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzDebugMessageVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzDebugMessageVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `GLuintVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzGLuintVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzGLuintVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `GLintVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzGLintVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzGLintVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `StringVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzStringVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzStringVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `StringPairVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzStringPairVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzStringPairVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `LinearColorStopVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzLinearColorStopVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzLinearColorStopVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `RadialColorStopVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzRadialColorStopVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzRadialColorStopVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `NodeIdVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzNodeIdVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzNodeIdVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `NodeVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzNodeVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzNodeVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `StyledNodeVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzStyledNodeVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzStyledNodeVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `TagIdsToNodeIdsMappingVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzTagIdsToNodeIdsMappingVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzTagIdsToNodeIdsMappingVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `ParentWithNodeDepthVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzParentWithNodeDepthVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzParentWithNodeDepthVecDestructorType),
    }
    /// Re-export of rust-allocated (stack based) `NodeDataVecDestructor` struct
    #[repr(C, u8)]  #[derive(Clone)]  #[derive(Copy)] pub enum AzNodeDataVecDestructor {
        DefaultRust,
        NoDestructor,
        External(AzNodeDataVecDestructorType),
    }
    /// Wrapper over a Rust-allocated `Vec<StyleBackgroundRepeat>`
    #[repr(C)]     pub struct AzStyleBackgroundRepeatVec {
        pub(crate) ptr: *const AzStyleBackgroundRepeat,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzStyleBackgroundRepeatVecDestructor,
    }
    /// Wrapper over a Rust-allocated `Vec<u32>`
    #[repr(C)]     pub struct AzU32Vec {
        pub(crate) ptr: *const u32,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzU32VecDestructor,
    }
    /// Wrapper over a Rust-allocated `XWindowType`
    #[repr(C)]     pub struct AzXWindowTypeVec {
        pub(crate) ptr: *const AzXWindowType,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzXWindowTypeVecDestructor,
    }
    /// Wrapper over a Rust-allocated `VirtualKeyCode`
    #[repr(C)]     pub struct AzVirtualKeyCodeVec {
        pub(crate) ptr: *const AzVirtualKeyCode,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzVirtualKeyCodeVecDestructor,
    }
    /// Wrapper over a Rust-allocated `ScanCode`
    #[repr(C)]     pub struct AzScanCodeVec {
        pub(crate) ptr: *const u32,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzScanCodeVecDestructor,
    }
    /// Wrapper over a Rust-allocated `U8Vec`
    #[repr(C)]     pub struct AzU8Vec {
        pub(crate) ptr: *const u8,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzU8VecDestructor,
    }
    /// Wrapper over a Rust-allocated `U32Vec`
    #[repr(C)]     pub struct AzGLuintVec {
        pub(crate) ptr: *const u32,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzGLuintVecDestructor,
    }
    /// Wrapper over a Rust-allocated `GLintVec`
    #[repr(C)]     pub struct AzGLintVec {
        pub(crate) ptr: *const i32,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzGLintVecDestructor,
    }
    /// Re-export of rust-allocated (stack based) `OptionGlContextPtr` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzOptionGlContextPtr {
        None,
        Some(AzGlContextPtr),
    }
    /// Re-export of rust-allocated (stack based) `OptionThreadSendMsg` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionThreadSendMsg {
        None,
        Some(AzThreadSendMsg),
    }
    /// Re-export of rust-allocated (stack based) `OptionRefAny` struct
    #[repr(C, u8)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub enum AzOptionRefAny {
        None,
        Some(AzRefAny),
    }
    /// Re-export of rust-allocated (stack based) `OptionWindowTheme` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionWindowTheme {
        None,
        Some(AzWindowTheme),
    }
    /// Re-export of rust-allocated (stack based) `OptionHwndHandle` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionHwndHandle {
        None,
        Some(*mut c_void),
    }
    /// Re-export of rust-allocated (stack based) `OptionX11Visual` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionX11Visual {
        None,
        Some(*const c_void),
    }
    /// Re-export of rust-allocated (stack based) `OptionI32` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionI32 {
        None,
        Some(i32),
    }
    /// Re-export of rust-allocated (stack based) `OptionF32` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionF32 {
        None,
        Some(f32),
    }
    /// Re-export of rust-allocated (stack based) `OptionMouseCursorType` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionMouseCursorType {
        None,
        Some(AzMouseCursorType),
    }
    /// Option<char> but the char is a u32, for C FFI stability reasons
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionChar {
        None,
        Some(u32),
    }
    /// Re-export of rust-allocated (stack based) `OptionVirtualKeyCode` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionVirtualKeyCode {
        None,
        Some(AzVirtualKeyCode),
    }
    /// Re-export of rust-allocated (stack based) `OptionTexture` struct
    #[repr(C, u8)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub enum AzOptionTexture {
        None,
        Some(AzTexture),
    }
    /// Re-export of rust-allocated (stack based) `OptionUsize` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionUsize {
        None,
        Some(usize),
    }
    /// Re-export of rust-allocated (stack based) `SvgParseErrorPosition` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzSvgParseErrorPosition {
        pub row: u32,
        pub col: u32,
    }
    /// Re-export of rust-allocated (stack based) `InstantPtrCloneFn` struct
    #[repr(C)]  #[derive(Clone)]  #[derive(Copy)] pub struct AzInstantPtrCloneFn {
        pub cb: AzInstantPtrCloneFnType,
    }
    /// Re-export of rust-allocated (stack based) `InstantPtrDestructorFn` struct
    #[repr(C)]  #[derive(Clone)]  #[derive(Copy)] pub struct AzInstantPtrDestructorFn {
        pub cb: AzInstantPtrDestructorFnType,
    }
    /// Re-export of rust-allocated (stack based) `SystemTick` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzSystemTick {
        pub tick_counter: u64,
    }
    /// Re-export of rust-allocated (stack based) `SystemTimeDiff` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzSystemTimeDiff {
        pub secs: u64,
        pub nanos: u32,
    }
    /// Re-export of rust-allocated (stack based) `SystemTickDiff` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzSystemTickDiff {
        pub tick_diff: u64,
    }
    /// Re-export of rust-allocated (stack based) `NodeId` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzNodeId {
        pub inner: usize,
    }
    /// Re-export of rust-allocated (stack based) `DomId` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzDomId {
        pub inner: usize,
    }
    /// Re-export of rust-allocated (stack based) `DomNodeId` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzDomNodeId {
        pub dom: AzDomId,
        pub node: AzNodeId,
    }
    /// Re-export of rust-allocated (stack based) `LayoutCallback` struct
    #[repr(C)]  #[derive(Clone)]  #[derive(Copy)] pub struct AzLayoutCallback {
        pub cb: AzLayoutCallbackType,
    }
    /// Re-export of rust-allocated (stack based) `Callback` struct
    #[repr(C)]  #[derive(Clone)]  #[derive(Copy)] pub struct AzCallback {
        pub cb: AzCallbackType,
    }
    /// Re-export of rust-allocated (stack based) `IFrameCallback` struct
    #[repr(C)]  #[derive(Clone)]  #[derive(Copy)] pub struct AzIFrameCallback {
        pub cb: AzIFrameCallbackType,
    }
    /// Re-export of rust-allocated (stack based) `GlCallback` struct
    #[repr(C)]  #[derive(Clone)]  #[derive(Copy)] pub struct AzGlCallback {
        pub cb: AzGlCallbackType,
    }
    /// Re-export of rust-allocated (stack based) `GlCallbackReturn` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzGlCallbackReturn {
        pub texture: AzOptionTexture,
    }
    /// Re-export of rust-allocated (stack based) `TimerCallback` struct
    #[repr(C)]  #[derive(Clone)]  #[derive(Copy)] pub struct AzTimerCallback {
        pub cb: AzTimerCallbackType,
    }
    /// Re-export of rust-allocated (stack based) `TimerCallbackReturn` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzTimerCallbackReturn {
        pub should_update: AzUpdateScreen,
        pub should_terminate: AzTerminateTimer,
    }
    /// Re-export of rust-allocated (stack based) `WriteBackCallback` struct
    #[repr(C)]  #[derive(Clone)]   pub struct AzWriteBackCallback {
        pub cb: AzWriteBackCallbackType,
    }
    /// Re-export of rust-allocated (stack based) `CssNthChildPattern` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzCssNthChildPattern {
        pub repeat: u32,
        pub offset: u32,
    }
    /// Re-export of rust-allocated (stack based) `ColorU` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzColorU {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8,
    }
    /// Re-export of rust-allocated (stack based) `FloatValue` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzFloatValue {
        pub number: isize,
    }
    /// Re-export of rust-allocated (stack based) `PixelValue` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzPixelValue {
        pub metric: AzSizeMetric,
        pub number: AzFloatValue,
    }
    /// Re-export of rust-allocated (stack based) `PixelValueNoPercent` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzPixelValueNoPercent {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleBoxShadow` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleBoxShadow {
        pub offset: [AzPixelValueNoPercent;2],
        pub color: AzColorU,
        pub blur_radius: AzPixelValueNoPercent,
        pub spread_radius: AzPixelValueNoPercent,
        pub clip_mode: AzBoxShadowClipMode,
    }
    /// Re-export of rust-allocated (stack based) `LayoutBottom` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutBottom {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `LayoutFlexGrow` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutFlexGrow {
        pub inner: AzFloatValue,
    }
    /// Re-export of rust-allocated (stack based) `LayoutFlexShrink` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutFlexShrink {
        pub inner: AzFloatValue,
    }
    /// Re-export of rust-allocated (stack based) `LayoutHeight` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutHeight {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `LayoutLeft` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutLeft {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `LayoutMarginBottom` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutMarginBottom {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `LayoutMarginLeft` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutMarginLeft {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `LayoutMarginRight` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutMarginRight {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `LayoutMarginTop` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutMarginTop {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `LayoutMaxHeight` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutMaxHeight {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `LayoutMaxWidth` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutMaxWidth {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `LayoutMinHeight` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutMinHeight {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `LayoutMinWidth` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutMinWidth {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `LayoutPaddingBottom` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutPaddingBottom {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `LayoutPaddingLeft` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutPaddingLeft {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `LayoutPaddingRight` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutPaddingRight {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `LayoutPaddingTop` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutPaddingTop {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `LayoutRight` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutRight {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `LayoutTop` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutTop {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `LayoutWidth` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutWidth {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `PercentageValue` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzPercentageValue {
        pub number: AzFloatValue,
    }
    /// Re-export of rust-allocated (stack based) `AngleValue` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzAngleValue {
        pub metric: AzAngleMetric,
        pub number: AzFloatValue,
    }
    /// Re-export of rust-allocated (stack based) `DirectionCorners` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzDirectionCorners {
        pub from: AzDirectionCorner,
        pub to: AzDirectionCorner,
    }
    /// Re-export of rust-allocated (stack based) `Direction` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzDirection {
        Angle(AzAngleValue),
        FromTo(AzDirectionCorners),
    }
    /// Re-export of rust-allocated (stack based) `BackgroundPositionHorizontal` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzBackgroundPositionHorizontal {
        Left,
        Center,
        Right,
        Exact(AzPixelValue),
    }
    /// Re-export of rust-allocated (stack based) `BackgroundPositionVertical` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzBackgroundPositionVertical {
        Top,
        Center,
        Bottom,
        Exact(AzPixelValue),
    }
    /// Re-export of rust-allocated (stack based) `StyleBackgroundPosition` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleBackgroundPosition {
        pub horizontal: AzBackgroundPositionHorizontal,
        pub vertical: AzBackgroundPositionVertical,
    }
    /// Re-export of rust-allocated (stack based) `StyleBackgroundSize` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleBackgroundSize {
        ExactSize([AzPixelValue;2]),
        Contain,
        Cover,
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderBottomColor` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleBorderBottomColor {
        pub inner: AzColorU,
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderBottomLeftRadius` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleBorderBottomLeftRadius {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderBottomRightRadius` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleBorderBottomRightRadius {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderBottomStyle` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleBorderBottomStyle {
        pub inner: AzBorderStyle,
    }
    /// Re-export of rust-allocated (stack based) `LayoutBorderBottomWidth` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutBorderBottomWidth {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderLeftColor` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleBorderLeftColor {
        pub inner: AzColorU,
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderLeftStyle` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleBorderLeftStyle {
        pub inner: AzBorderStyle,
    }
    /// Re-export of rust-allocated (stack based) `LayoutBorderLeftWidth` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutBorderLeftWidth {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderRightColor` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleBorderRightColor {
        pub inner: AzColorU,
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderRightStyle` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleBorderRightStyle {
        pub inner: AzBorderStyle,
    }
    /// Re-export of rust-allocated (stack based) `LayoutBorderRightWidth` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutBorderRightWidth {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderTopColor` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleBorderTopColor {
        pub inner: AzColorU,
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderTopLeftRadius` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleBorderTopLeftRadius {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderTopRightRadius` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleBorderTopRightRadius {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderTopStyle` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleBorderTopStyle {
        pub inner: AzBorderStyle,
    }
    /// Re-export of rust-allocated (stack based) `LayoutBorderTopWidth` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutBorderTopWidth {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleFontSize` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleFontSize {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleLetterSpacing` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleLetterSpacing {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleLineHeight` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleLineHeight {
        pub inner: AzPercentageValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleTabWidth` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleTabWidth {
        pub inner: AzPercentageValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleOpacity` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleOpacity {
        pub inner: AzFloatValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleTransformOrigin` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleTransformOrigin {
        pub x: AzPixelValue,
        pub y: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `StylePerspectiveOrigin` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStylePerspectiveOrigin {
        pub x: AzPixelValue,
        pub y: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleTransformMatrix2D` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleTransformMatrix2D {
        pub a: AzPixelValue,
        pub b: AzPixelValue,
        pub c: AzPixelValue,
        pub d: AzPixelValue,
        pub tx: AzPixelValue,
        pub ty: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleTransformMatrix3D` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleTransformMatrix3D {
        pub m11: AzPixelValue,
        pub m12: AzPixelValue,
        pub m13: AzPixelValue,
        pub m14: AzPixelValue,
        pub m21: AzPixelValue,
        pub m22: AzPixelValue,
        pub m23: AzPixelValue,
        pub m24: AzPixelValue,
        pub m31: AzPixelValue,
        pub m32: AzPixelValue,
        pub m33: AzPixelValue,
        pub m34: AzPixelValue,
        pub m41: AzPixelValue,
        pub m42: AzPixelValue,
        pub m43: AzPixelValue,
        pub m44: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleTransformTranslate2D` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleTransformTranslate2D {
        pub x: AzPixelValue,
        pub y: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleTransformTranslate3D` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleTransformTranslate3D {
        pub x: AzPixelValue,
        pub y: AzPixelValue,
        pub z: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleTransformRotate3D` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleTransformRotate3D {
        pub x: AzPercentageValue,
        pub y: AzPercentageValue,
        pub z: AzPercentageValue,
        pub angle: AzAngleValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleTransformScale2D` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleTransformScale2D {
        pub x: AzPercentageValue,
        pub y: AzPercentageValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleTransformScale3D` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleTransformScale3D {
        pub x: AzPercentageValue,
        pub y: AzPercentageValue,
        pub z: AzPercentageValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleTransformSkew2D` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleTransformSkew2D {
        pub x: AzPercentageValue,
        pub y: AzPercentageValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleTextColor` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleTextColor {
        pub inner: AzColorU,
    }
    /// Re-export of rust-allocated (stack based) `StyleWordSpacing` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyleWordSpacing {
        pub inner: AzPixelValue,
    }
    /// Re-export of rust-allocated (stack based) `StyleBoxShadowValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleBoxShadowValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleBoxShadow),
    }
    /// Re-export of rust-allocated (stack based) `LayoutAlignContentValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutAlignContentValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutAlignContent),
    }
    /// Re-export of rust-allocated (stack based) `LayoutAlignItemsValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutAlignItemsValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutAlignItems),
    }
    /// Re-export of rust-allocated (stack based) `LayoutBottomValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutBottomValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutBottom),
    }
    /// Re-export of rust-allocated (stack based) `LayoutBoxSizingValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutBoxSizingValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutBoxSizing),
    }
    /// Re-export of rust-allocated (stack based) `LayoutFlexDirectionValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutFlexDirectionValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutFlexDirection),
    }
    /// Re-export of rust-allocated (stack based) `LayoutDisplayValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutDisplayValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutDisplay),
    }
    /// Re-export of rust-allocated (stack based) `LayoutFlexGrowValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutFlexGrowValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutFlexGrow),
    }
    /// Re-export of rust-allocated (stack based) `LayoutFlexShrinkValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutFlexShrinkValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutFlexShrink),
    }
    /// Re-export of rust-allocated (stack based) `LayoutFloatValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutFloatValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutFloat),
    }
    /// Re-export of rust-allocated (stack based) `LayoutHeightValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutHeightValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutHeight),
    }
    /// Re-export of rust-allocated (stack based) `LayoutJustifyContentValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutJustifyContentValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutJustifyContent),
    }
    /// Re-export of rust-allocated (stack based) `LayoutLeftValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutLeftValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutLeft),
    }
    /// Re-export of rust-allocated (stack based) `LayoutMarginBottomValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutMarginBottomValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutMarginBottom),
    }
    /// Re-export of rust-allocated (stack based) `LayoutMarginLeftValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutMarginLeftValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutMarginLeft),
    }
    /// Re-export of rust-allocated (stack based) `LayoutMarginRightValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutMarginRightValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutMarginRight),
    }
    /// Re-export of rust-allocated (stack based) `LayoutMarginTopValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutMarginTopValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutMarginTop),
    }
    /// Re-export of rust-allocated (stack based) `LayoutMaxHeightValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutMaxHeightValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutMaxHeight),
    }
    /// Re-export of rust-allocated (stack based) `LayoutMaxWidthValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutMaxWidthValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutMaxWidth),
    }
    /// Re-export of rust-allocated (stack based) `LayoutMinHeightValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutMinHeightValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutMinHeight),
    }
    /// Re-export of rust-allocated (stack based) `LayoutMinWidthValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutMinWidthValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutMinWidth),
    }
    /// Re-export of rust-allocated (stack based) `LayoutPaddingBottomValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutPaddingBottomValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutPaddingBottom),
    }
    /// Re-export of rust-allocated (stack based) `LayoutPaddingLeftValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutPaddingLeftValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutPaddingLeft),
    }
    /// Re-export of rust-allocated (stack based) `LayoutPaddingRightValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutPaddingRightValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutPaddingRight),
    }
    /// Re-export of rust-allocated (stack based) `LayoutPaddingTopValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutPaddingTopValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutPaddingTop),
    }
    /// Re-export of rust-allocated (stack based) `LayoutPositionValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutPositionValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutPosition),
    }
    /// Re-export of rust-allocated (stack based) `LayoutRightValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutRightValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutRight),
    }
    /// Re-export of rust-allocated (stack based) `LayoutTopValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutTopValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutTop),
    }
    /// Re-export of rust-allocated (stack based) `LayoutWidthValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutWidthValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutWidth),
    }
    /// Re-export of rust-allocated (stack based) `LayoutFlexWrapValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutFlexWrapValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutFlexWrap),
    }
    /// Re-export of rust-allocated (stack based) `LayoutOverflowValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutOverflowValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutOverflow),
    }
    /// Re-export of rust-allocated (stack based) `StyleBackgroundRepeatVecValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzStyleBackgroundRepeatVecValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleBackgroundRepeatVec),
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderBottomColorValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleBorderBottomColorValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleBorderBottomColor),
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderBottomLeftRadiusValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleBorderBottomLeftRadiusValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleBorderBottomLeftRadius),
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderBottomRightRadiusValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleBorderBottomRightRadiusValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleBorderBottomRightRadius),
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderBottomStyleValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleBorderBottomStyleValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleBorderBottomStyle),
    }
    /// Re-export of rust-allocated (stack based) `LayoutBorderBottomWidthValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutBorderBottomWidthValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutBorderBottomWidth),
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderLeftColorValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleBorderLeftColorValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleBorderLeftColor),
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderLeftStyleValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleBorderLeftStyleValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleBorderLeftStyle),
    }
    /// Re-export of rust-allocated (stack based) `LayoutBorderLeftWidthValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutBorderLeftWidthValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutBorderLeftWidth),
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderRightColorValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleBorderRightColorValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleBorderRightColor),
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderRightStyleValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleBorderRightStyleValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleBorderRightStyle),
    }
    /// Re-export of rust-allocated (stack based) `LayoutBorderRightWidthValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutBorderRightWidthValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutBorderRightWidth),
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderTopColorValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleBorderTopColorValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleBorderTopColor),
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderTopLeftRadiusValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleBorderTopLeftRadiusValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleBorderTopLeftRadius),
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderTopRightRadiusValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleBorderTopRightRadiusValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleBorderTopRightRadius),
    }
    /// Re-export of rust-allocated (stack based) `StyleBorderTopStyleValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleBorderTopStyleValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleBorderTopStyle),
    }
    /// Re-export of rust-allocated (stack based) `LayoutBorderTopWidthValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzLayoutBorderTopWidthValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzLayoutBorderTopWidth),
    }
    /// Re-export of rust-allocated (stack based) `StyleCursorValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleCursorValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleCursor),
    }
    /// Re-export of rust-allocated (stack based) `StyleFontSizeValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleFontSizeValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleFontSize),
    }
    /// Re-export of rust-allocated (stack based) `StyleLetterSpacingValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleLetterSpacingValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleLetterSpacing),
    }
    /// Re-export of rust-allocated (stack based) `StyleLineHeightValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleLineHeightValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleLineHeight),
    }
    /// Re-export of rust-allocated (stack based) `StyleTabWidthValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleTabWidthValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleTabWidth),
    }
    /// Re-export of rust-allocated (stack based) `StyleTextAlignmentHorzValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleTextAlignmentHorzValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleTextAlignmentHorz),
    }
    /// Re-export of rust-allocated (stack based) `StyleTextColorValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleTextColorValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleTextColor),
    }
    /// Re-export of rust-allocated (stack based) `StyleWordSpacingValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleWordSpacingValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleWordSpacing),
    }
    /// Re-export of rust-allocated (stack based) `StyleOpacityValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleOpacityValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleOpacity),
    }
    /// Re-export of rust-allocated (stack based) `StyleTransformOriginValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleTransformOriginValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleTransformOrigin),
    }
    /// Re-export of rust-allocated (stack based) `StylePerspectiveOriginValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStylePerspectiveOriginValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStylePerspectiveOrigin),
    }
    /// Re-export of rust-allocated (stack based) `StyleBackfaceVisibilityValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleBackfaceVisibilityValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleBackfaceVisibility),
    }
    /// Re-export of rust-allocated (stack based) `Node` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzNode {
        pub parent: usize,
        pub previous_sibling: usize,
        pub next_sibling: usize,
        pub last_child: usize,
    }
    /// Re-export of rust-allocated (stack based) `CascadeInfo` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzCascadeInfo {
        pub index_in_parent: u32,
        pub is_last_child: bool,
    }
    /// Re-export of rust-allocated (stack based) `StyledNodeState` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzStyledNodeState {
        pub normal: bool,
        pub hover: bool,
        pub active: bool,
        pub focused: bool,
    }
    /// Re-export of rust-allocated (stack based) `TagId` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzTagId {
        pub inner: u64,
    }
    /// Re-export of rust-allocated (stack based) `ParentWithNodeDepth` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzParentWithNodeDepth {
        pub depth: usize,
        pub node_id: AzNodeId,
    }
    /// Re-export of rust-allocated (stack based) `GlTextureNode` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzGlTextureNode {
        pub callback: AzGlCallback,
        pub data: AzRefAny,
    }
    /// Re-export of rust-allocated (stack based) `IFrameNode` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzIFrameNode {
        pub callback: AzIFrameCallback,
        pub data: AzRefAny,
    }
    /// Re-export of rust-allocated (stack based) `NotEventFilter` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzNotEventFilter {
        Hover(AzHoverEventFilter),
        Focus(AzFocusEventFilter),
    }
    /// Re-export of rust-allocated (stack based) `TabIndex` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzTabIndex {
        Auto,
        OverrideInParent(u32),
        NoKeyboardFocus,
    }
    /// Re-export of rust-allocated (stack based) `GlShaderPrecisionFormatReturn` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzGlShaderPrecisionFormatReturn {
        pub _0: i32,
        pub _1: i32,
        pub _2: i32,
    }
    /// C-ABI stable reexport of `&[u8]`
    #[repr(C)]     pub struct AzU8VecRef {
        pub(crate) ptr: *const u8,
        pub len: usize,
    }
    /// C-ABI stable reexport of `&mut [u8]`
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzU8VecRefMut {
        pub(crate) ptr: *mut u8,
        pub len: usize,
    }
    /// C-ABI stable reexport of `&[f32]`
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzF32VecRef {
        pub(crate) ptr: *const f32,
        pub len: usize,
    }
    /// C-ABI stable reexport of `&[i32]`
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzI32VecRef {
        pub(crate) ptr: *const i32,
        pub len: usize,
    }
    /// C-ABI stable reexport of `&[GLuint]` aka `&[u32]`
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzGLuintVecRef {
        pub(crate) ptr: *const u32,
        pub len: usize,
    }
    /// C-ABI stable reexport of `&[GLenum]` aka `&[u32]`
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzGLenumVecRef {
        pub(crate) ptr: *const u32,
        pub len: usize,
    }
    /// C-ABI stable reexport of `&mut [GLint]` aka `&mut [i32]`
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzGLintVecRefMut {
        pub(crate) ptr: *mut i32,
        pub len: usize,
    }
    /// C-ABI stable reexport of `&mut [GLint64]` aka `&mut [i64]`
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzGLint64VecRefMut {
        pub(crate) ptr: *mut i64,
        pub len: usize,
    }
    /// C-ABI stable reexport of `&mut [GLboolean]` aka `&mut [u8]`
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzGLbooleanVecRefMut {
        pub(crate) ptr: *mut u8,
        pub len: usize,
    }
    /// C-ABI stable reexport of `&mut [GLfloat]` aka `&mut [f32]`
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzGLfloatVecRefMut {
        pub(crate) ptr: *mut f32,
        pub len: usize,
    }
    /// C-ABI stable reexport of `&str`
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzRefstr {
        pub(crate) ptr: *const u8,
        pub len: usize,
    }
    /// C-ABI stable reexport of `(U8Vec, u32)`
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzGetProgramBinaryReturn {
        pub _0: AzU8Vec,
        pub _1: u32,
    }
    /// Re-export of rust-allocated (stack based) `TextureFlags` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzTextureFlags {
        pub is_opaque: bool,
        pub is_video_texture: bool,
    }
    /// Re-export of rust-allocated (stack based) `ImageId` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzImageId {
        pub id: usize,
    }
    /// Re-export of rust-allocated (stack based) `FontId` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzFontId {
        pub id: usize,
    }
    /// Re-export of rust-allocated (stack based) `RawImage` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzRawImage {
        pub pixels: AzU8Vec,
        pub width: usize,
        pub height: usize,
        pub data_format: AzRawImageFormat,
    }
    /// Re-export of rust-allocated (stack based) `SvgCircle` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzSvgCircle {
        pub center_x: f32,
        pub center_y: f32,
        pub radius: f32,
    }
    /// Re-export of rust-allocated (stack based) `SvgPoint` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzSvgPoint {
        pub x: f32,
        pub y: f32,
    }
    /// Re-export of rust-allocated (stack based) `SvgVertex` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzSvgVertex {
        pub x: f32,
        pub y: f32,
    }
    /// Re-export of rust-allocated (stack based) `SvgQuadraticCurve` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzSvgQuadraticCurve {
        pub start: AzSvgPoint,
        pub ctrl: AzSvgPoint,
        pub end: AzSvgPoint,
    }
    /// Re-export of rust-allocated (stack based) `SvgCubicCurve` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzSvgCubicCurve {
        pub start: AzSvgPoint,
        pub ctrl_1: AzSvgPoint,
        pub ctrl_2: AzSvgPoint,
        pub end: AzSvgPoint,
    }
    /// Re-export of rust-allocated (stack based) `SvgRect` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzSvgRect {
        pub width: f32,
        pub height: f32,
        pub x: f32,
        pub y: f32,
        pub radius_top_left: f32,
        pub radius_top_right: f32,
        pub radius_bottom_left: f32,
        pub radius_bottom_right: f32,
    }
    /// Re-export of rust-allocated (stack based) `SvgFitTo` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzSvgFitTo {
        Original,
        Width(u32),
        Height(u32),
        Zoom(f32),
    }
    /// Re-export of rust-allocated (stack based) `SvgDashPattern` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzSvgDashPattern {
        pub offset: usize,
        pub length_1: usize,
        pub gap_1: usize,
        pub length_2: usize,
        pub gap_2: usize,
        pub length_3: usize,
        pub gap_3: usize,
    }
    /// Re-export of rust-allocated (stack based) `SvgFillStyle` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzSvgFillStyle {
        pub line_join: AzSvgLineJoin,
        pub miter_limit: usize,
        pub tolerance: usize,
    }
    /// Re-export of rust-allocated (stack based) `TimerId` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzTimerId {
        pub id: usize,
    }
    /// Re-export of rust-allocated (stack based) `ThreadId` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzThreadId {
        pub id: usize,
    }
    /// Re-export of rust-allocated (stack based) `ThreadWriteBackMsg` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzThreadWriteBackMsg {
        pub data: AzRefAny,
        pub callback: AzWriteBackCallback,
    }
    /// Re-export of rust-allocated (stack based) `CreateThreadFn` struct
    #[repr(C)]  #[derive(Clone)]   pub struct AzCreateThreadFn {
        pub cb: AzCreateThreadFnType,
    }
    /// Get the current system time, equivalent to `std::time::Instant::now()`, except it also works on systems that work with "ticks" instead of timers
    #[repr(C)]  #[derive(Clone)]   pub struct AzGetSystemTimeFn {
        pub cb: AzGetSystemTimeFnType,
    }
    /// Function called to check if the thread has finished
    #[repr(C)]  #[derive(Clone)]   pub struct AzCheckThreadFinishedFn {
        pub cb: AzCheckThreadFinishedFnType,
    }
    /// Function to send a message to the thread
    #[repr(C)]  #[derive(Clone)]   pub struct AzLibrarySendThreadMsgFn {
        pub cb: AzLibrarySendThreadMsgFnType,
    }
    /// Function to receive a message from the thread
    #[repr(C)]  #[derive(Clone)]   pub struct AzLibraryReceiveThreadMsgFn {
        pub cb: AzLibraryReceiveThreadMsgFnType,
    }
    /// Function that the running `Thread` can call to receive messages from the main UI thread
    #[repr(C)]  #[derive(Clone)]   pub struct AzThreadRecvFn {
        pub cb: AzThreadRecvFnType,
    }
    /// Function that the running `Thread` can call to receive messages from the main UI thread
    #[repr(C)]  #[derive(Clone)]   pub struct AzThreadSendFn {
        pub cb: AzThreadSendFnType,
    }
    /// Destructor of the `Thread`
    #[repr(C)]  #[derive(Clone)]   pub struct AzThreadDestructorFn {
        pub cb: AzThreadDestructorFnType,
    }
    /// Destructor of the `ThreadReceiver`
    #[repr(C)]  #[derive(Clone)]   pub struct AzThreadReceiverDestructorFn {
        pub cb: AzThreadReceiverDestructorFnType,
    }
    /// Destructor of the `ThreadSender`
    #[repr(C)]  #[derive(Clone)]   pub struct AzThreadSenderDestructorFn {
        pub cb: AzThreadSenderDestructorFnType,
    }
    /// Re-export of rust-allocated (stack based) `RendererOptions` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzRendererOptions {
        pub vsync: AzVsync,
        pub srgb: AzSrgb,
        pub hw_accel: AzHwAcceleration,
    }
    /// Re-export of rust-allocated (stack based) `LayoutPoint` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutPoint {
        pub x: isize,
        pub y: isize,
    }
    /// Re-export of rust-allocated (stack based) `LayoutSize` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutSize {
        pub width: isize,
        pub height: isize,
    }
    /// Re-export of rust-allocated (stack based) `LayoutRect` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLayoutRect {
        pub origin: AzLayoutPoint,
        pub size: AzLayoutSize,
    }
    /// Re-export of rust-allocated (stack based) `IOSHandle` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzIOSHandle {
        pub ui_window: *mut c_void,
        pub ui_view: *mut c_void,
        pub ui_view_controller: *mut c_void,
    }
    /// Re-export of rust-allocated (stack based) `MacOSHandle` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzMacOSHandle {
        pub ns_window: *mut c_void,
        pub ns_view: *mut c_void,
    }
    /// Re-export of rust-allocated (stack based) `XlibHandle` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzXlibHandle {
        pub window: u64,
        pub display: *mut c_void,
    }
    /// Re-export of rust-allocated (stack based) `XcbHandle` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzXcbHandle {
        pub window: u32,
        pub connection: *mut c_void,
    }
    /// Re-export of rust-allocated (stack based) `WaylandHandle` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzWaylandHandle {
        pub surface: *mut c_void,
        pub display: *mut c_void,
    }
    /// Re-export of rust-allocated (stack based) `WindowsHandle` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzWindowsHandle {
        pub hwnd: *mut c_void,
        pub hinstance: *mut c_void,
    }
    /// Re-export of rust-allocated (stack based) `WebHandle` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzWebHandle {
        pub id: u32,
    }
    /// Re-export of rust-allocated (stack based) `AndroidHandle` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzAndroidHandle {
        pub a_native_window: *mut c_void,
    }
    /// Re-export of rust-allocated (stack based) `PhysicalPositionI32` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzPhysicalPositionI32 {
        pub x: i32,
        pub y: i32,
    }
    /// Re-export of rust-allocated (stack based) `PhysicalSizeU32` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzPhysicalSizeU32 {
        pub width: u32,
        pub height: u32,
    }
    /// Re-export of rust-allocated (stack based) `LogicalPosition` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLogicalPosition {
        pub x: f32,
        pub y: f32,
    }
    /// Re-export of rust-allocated (stack based) `IconKey` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzIconKey {
        pub id: usize,
    }
    /// Re-export of rust-allocated (stack based) `SmallWindowIconBytes` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzSmallWindowIconBytes {
        pub key: AzIconKey,
        pub rgba_bytes: AzU8Vec,
    }
    /// Re-export of rust-allocated (stack based) `LargeWindowIconBytes` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzLargeWindowIconBytes {
        pub key: AzIconKey,
        pub rgba_bytes: AzU8Vec,
    }
    /// Re-export of rust-allocated (stack based) `WindowIcon` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzWindowIcon {
        Small(AzSmallWindowIconBytes),
        Large(AzLargeWindowIconBytes),
    }
    /// Re-export of rust-allocated (stack based) `AcceleratorKey` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzAcceleratorKey {
        Ctrl,
        Alt,
        Shift,
        Key(AzVirtualKeyCode),
    }
    /// Re-export of rust-allocated (stack based) `WindowFlags` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzWindowFlags {
        pub is_maximized: bool,
        pub is_minimized: bool,
        pub is_about_to_close: bool,
        pub is_fullscreen: bool,
        pub has_decorations: bool,
        pub is_visible: bool,
        pub is_always_on_top: bool,
        pub is_resizable: bool,
        pub has_focus: bool,
        pub has_blur_behind_window: bool,
    }
    /// Re-export of rust-allocated (stack based) `DebugState` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzDebugState {
        pub profiler_dbg: bool,
        pub render_target_dbg: bool,
        pub texture_cache_dbg: bool,
        pub gpu_time_queries: bool,
        pub gpu_sample_queries: bool,
        pub disable_batching: bool,
        pub epochs: bool,
        pub echo_driver_messages: bool,
        pub show_overdraw: bool,
        pub gpu_cache_dbg: bool,
        pub texture_cache_dbg_clear_evicted: bool,
        pub picture_caching_dbg: bool,
        pub primitive_dbg: bool,
        pub zoom_dbg: bool,
        pub small_screen: bool,
        pub disable_opaque_pass: bool,
        pub disable_alpha_pass: bool,
        pub disable_clip_masks: bool,
        pub disable_text_prims: bool,
        pub disable_gradient_prims: bool,
        pub obscure_images: bool,
        pub glyph_flashing: bool,
        pub smart_profiler: bool,
        pub invalidation_dbg: bool,
        pub tile_cache_logging_dbg: bool,
        pub profiler_capture: bool,
        pub force_picture_invalidation: bool,
    }
    /// Re-export of rust-allocated (stack based) `KeyboardState` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzKeyboardState {
        pub shift_down: bool,
        pub ctrl_down: bool,
        pub alt_down: bool,
        pub super_down: bool,
        pub current_char: AzOptionChar,
        pub current_virtual_keycode: AzOptionVirtualKeyCode,
        pub pressed_virtual_keycodes: AzVirtualKeyCodeVec,
        pub pressed_scancodes: AzScanCodeVec,
    }
    /// Re-export of rust-allocated (stack based) `CursorPosition` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzCursorPosition {
        OutOfWindow,
        Uninitialized,
        InWindow(AzLogicalPosition),
    }
    /// Re-export of rust-allocated (stack based) `MouseState` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzMouseState {
        pub mouse_cursor_type: AzOptionMouseCursorType,
        pub cursor_position: AzCursorPosition,
        pub is_cursor_locked: bool,
        pub left_down: bool,
        pub right_down: bool,
        pub middle_down: bool,
        pub scroll_x: AzOptionF32,
        pub scroll_y: AzOptionF32,
    }
    /// Re-export of rust-allocated (stack based) `WaylandTheme` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzWaylandTheme {
        pub primary_color_active: [u8;4],
        pub primary_color_inactive: [u8;4],
        pub secondary_color_active: [u8;4],
        pub secondary_color_inactive: [u8;4],
        pub close_button_color_idle: [u8;4],
        pub close_button_color_hovered: [u8;4],
        pub close_button_color_disabled: [u8;4],
        pub maximize_button_color_idle: [u8;4],
        pub maximize_button_color_hovered: [u8;4],
        pub maximize_button_color_disabled: [u8;4],
        pub minimize_button_color_idle: [u8;4],
        pub minimize_button_color_hovered: [u8;4],
        pub minimize_button_color_disabled: [u8;4],
    }
    /// Re-export of rust-allocated (stack based) `MacWindowOptions` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzMacWindowOptions {
        pub _reserved: u8,
    }
    /// Re-export of rust-allocated (stack based) `WasmWindowOptions` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzWasmWindowOptions {
        pub _reserved: u8,
    }
    /// Re-export of rust-allocated (stack based) `WindowPosition` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzWindowPosition {
        Uninitialized,
        Initialized(AzPhysicalPositionI32),
    }
    /// Re-export of rust-allocated (stack based) `ImePosition` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzImePosition {
        Uninitialized,
        Initialized(AzLogicalPosition),
    }
    /// Re-export of rust-allocated (stack based) `TouchState` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzTouchState {
        pub unused: u8,
    }
    /// Re-export of rust-allocated (stack based) `LogicalSize` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLogicalSize {
        pub width: f32,
        pub height: f32,
    }
    /// Re-export of rust-allocated (stack based) `String` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzString {
        pub vec: AzU8Vec,
    }
    /// Wrapper over a Rust-allocated `Vec<StyleBackgroundPosition>`
    #[repr(C)]     pub struct AzStyleBackgroundPositionVec {
        pub(crate) ptr: *const AzStyleBackgroundPosition,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzStyleBackgroundPositionVecDestructor,
    }
    /// Wrapper over a Rust-allocated `Vec<StyleBackgroundSize>`
    #[repr(C)]     pub struct AzStyleBackgroundSizeVec {
        pub(crate) ptr: *const AzStyleBackgroundSize,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzStyleBackgroundSizeVecDestructor,
    }
    /// Wrapper over a Rust-allocated `SvgVertex`
    #[repr(C)]     pub struct AzSvgVertexVec {
        pub(crate) ptr: *const AzSvgVertex,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzSvgVertexVecDestructor,
    }
    /// Wrapper over a Rust-allocated `CascadeInfo`
    #[repr(C)]     pub struct AzCascadeInfoVec {
        pub(crate) ptr: *const AzCascadeInfo,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzCascadeInfoVecDestructor,
    }
    /// Wrapper over a Rust-allocated `StringVec`
    #[repr(C)]     pub struct AzStringVec {
        pub(crate) ptr: *const AzString,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzStringVecDestructor,
    }
    /// Wrapper over a Rust-allocated `NodeIdVec`
    #[repr(C)]     pub struct AzNodeIdVec {
        pub(crate) ptr: *const AzNodeId,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzNodeIdVecDestructor,
    }
    /// Wrapper over a Rust-allocated `NodeVec`
    #[repr(C)]     pub struct AzNodeVec {
        pub(crate) ptr: *const AzNode,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzNodeVecDestructor,
    }
    /// Wrapper over a Rust-allocated `ParentWithNodeDepthVec`
    #[repr(C)]     pub struct AzParentWithNodeDepthVec {
        pub(crate) ptr: *const AzParentWithNodeDepth,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzParentWithNodeDepthVecDestructor,
    }
    /// Re-export of rust-allocated (stack based) `OptionPercentageValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionPercentageValue {
        None,
        Some(AzPercentageValue),
    }
    /// Re-export of rust-allocated (stack based) `OptionAngleValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionAngleValue {
        None,
        Some(AzAngleValue),
    }
    /// Re-export of rust-allocated (stack based) `OptionRendererOptions` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionRendererOptions {
        None,
        Some(AzRendererOptions),
    }
    /// Re-export of rust-allocated (stack based) `OptionCallback` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionCallback {
        None,
        Some(AzCallback),
    }
    /// Re-export of rust-allocated (stack based) `OptionLayoutRect` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionLayoutRect {
        None,
        Some(AzLayoutRect),
    }
    /// Re-export of rust-allocated (stack based) `OptionLayoutPoint` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionLayoutPoint {
        None,
        Some(AzLayoutPoint),
    }
    /// Re-export of rust-allocated (stack based) `OptionNodeId` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionNodeId {
        None,
        Some(AzNodeId),
    }
    /// Re-export of rust-allocated (stack based) `OptionDomNodeId` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionDomNodeId {
        None,
        Some(AzDomNodeId),
    }
    /// Re-export of rust-allocated (stack based) `OptionColorU` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionColorU {
        None,
        Some(AzColorU),
    }
    /// Re-export of rust-allocated (stack based) `OptionRawImage` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzOptionRawImage {
        None,
        Some(AzRawImage),
    }
    /// Re-export of rust-allocated (stack based) `OptionSvgDashPattern` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionSvgDashPattern {
        None,
        Some(AzSvgDashPattern),
    }
    /// Re-export of rust-allocated (stack based) `OptionWaylandTheme` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzOptionWaylandTheme {
        None,
        Some(AzWaylandTheme),
    }
    /// Re-export of rust-allocated (stack based) `OptionLogicalPosition` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionLogicalPosition {
        None,
        Some(AzLogicalPosition),
    }
    /// Re-export of rust-allocated (stack based) `OptionPhysicalPositionI32` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionPhysicalPositionI32 {
        None,
        Some(AzPhysicalPositionI32),
    }
    /// Re-export of rust-allocated (stack based) `OptionWindowIcon` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzOptionWindowIcon {
        None,
        Some(AzWindowIcon),
    }
    /// Re-export of rust-allocated (stack based) `OptionString` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzOptionString {
        None,
        Some(AzString),
    }
    /// Re-export of rust-allocated (stack based) `OptionLogicalSize` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionLogicalSize {
        None,
        Some(AzLogicalSize),
    }
    /// Re-export of rust-allocated (stack based) `OptionTabIndex` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionTabIndex {
        None,
        Some(AzTabIndex),
    }
    /// Re-export of rust-allocated (stack based) `OptionTagId` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionTagId {
        None,
        Some(AzTagId),
    }
    /// Re-export of rust-allocated (stack based) `OptionU8VecRef` struct
    #[repr(C, u8)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub enum AzOptionU8VecRef {
        None,
        Some(AzU8VecRef),
    }
    /// Re-export of rust-allocated (stack based) `DuplicatedNamespaceError` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzDuplicatedNamespaceError {
        pub ns: AzString,
        pub pos: AzSvgParseErrorPosition,
    }
    /// Re-export of rust-allocated (stack based) `UnknownNamespaceError` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzUnknownNamespaceError {
        pub ns: AzString,
        pub pos: AzSvgParseErrorPosition,
    }
    /// Re-export of rust-allocated (stack based) `UnexpectedCloseTagError` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzUnexpectedCloseTagError {
        pub expected: AzString,
        pub actual: AzString,
        pub pos: AzSvgParseErrorPosition,
    }
    /// Re-export of rust-allocated (stack based) `UnknownEntityReferenceError` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzUnknownEntityReferenceError {
        pub entity: AzString,
        pub pos: AzSvgParseErrorPosition,
    }
    /// Re-export of rust-allocated (stack based) `DuplicatedAttributeError` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzDuplicatedAttributeError {
        pub attribute: AzString,
        pub pos: AzSvgParseErrorPosition,
    }
    /// Re-export of rust-allocated (stack based) `NonXmlCharError` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzNonXmlCharError {
        pub ch: u32,
        pub pos: AzSvgParseErrorPosition,
    }
    /// Re-export of rust-allocated (stack based) `InvalidCharError` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzInvalidCharError {
        pub expected: u8,
        pub got: u8,
        pub pos: AzSvgParseErrorPosition,
    }
    /// Re-export of rust-allocated (stack based) `InvalidCharMultipleError` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzInvalidCharMultipleError {
        pub expected: u8,
        pub got: AzU8Vec,
        pub pos: AzSvgParseErrorPosition,
    }
    /// Re-export of rust-allocated (stack based) `InvalidQuoteError` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzInvalidQuoteError {
        pub got: u8,
        pub pos: AzSvgParseErrorPosition,
    }
    /// Re-export of rust-allocated (stack based) `InvalidSpaceError` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzInvalidSpaceError {
        pub got: u8,
        pub pos: AzSvgParseErrorPosition,
    }
    /// Re-export of rust-allocated (stack based) `InvalidStringError` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzInvalidStringError {
        pub got: AzString,
        pub pos: AzSvgParseErrorPosition,
    }
    /// Re-export of rust-allocated (stack based) `Instant` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzInstant {
        System(AzInstantPtr),
        Tick(AzSystemTick),
    }
    /// Re-export of rust-allocated (stack based) `Duration` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzDuration {
        System(AzSystemTimeDiff),
        Tick(AzSystemTickDiff),
    }
    /// External system callbacks to get the system time or create / manage threads
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzSystemCallbacks {
        pub create_thread_fn: AzCreateThreadFn,
        pub get_system_time_fn: AzGetSystemTimeFn,
    }
    /// Configuration for optional features, such as whether to enable logging or panic hooks
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzAppConfig {
        pub log_level: AzAppLogLevel,
        pub enable_visual_panic_hook: bool,
        pub enable_logging_on_panic: bool,
        pub enable_tab_navigation: bool,
        pub debug_state: AzDebugState,
        pub system_callbacks: AzSystemCallbacks,
    }
    /// Re-export of rust-allocated (stack based) `HidpiAdjustedBounds` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzHidpiAdjustedBounds {
        pub logical_size: AzLogicalSize,
        pub hidpi_factor: f32,
    }
    /// Re-export of rust-allocated (stack based) `IFrameCallbackInfo` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzIFrameCallbackInfo {
        pub resources: *const c_void,
        pub bounds: AzHidpiAdjustedBounds,
    }
    /// Re-export of rust-allocated (stack based) `GlCallbackInfo` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzGlCallbackInfo {
        pub gl_context: *const AzGlContextPtr,
        pub resources: *const c_void,
        pub node_hierarchy: *const AzNodeVec,
        pub words_cache: *const c_void,
        pub shaped_words_cache: *const c_void,
        pub positioned_words_cache: *const c_void,
        pub positioned_rects: *const c_void,
        pub bounds: AzHidpiAdjustedBounds,
    }
    /// Re-export of rust-allocated (stack based) `RefCountInner` struct
    #[repr(C)]     pub struct AzRefCountInner {
        pub num_copies: usize,
        pub num_refs: usize,
        pub num_mutable_refs: usize,
        pub _internal_ptr: *const c_void,
        pub _internal_len: usize,
        pub _internal_layout_size: usize,
        pub _internal_layout_align: usize,
        pub type_id: u64,
        pub type_name: AzString,
        pub custom_destructor: AzRefAnyDestructorType,
    }
    /// Re-export of rust-allocated (stack based) `CssNthChildSelector` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzCssNthChildSelector {
        Number(u32),
        Even,
        Odd,
        Pattern(AzCssNthChildPattern),
    }
    /// Re-export of rust-allocated (stack based) `LinearColorStop` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLinearColorStop {
        pub offset: AzOptionPercentageValue,
        pub color: AzColorU,
    }
    /// Re-export of rust-allocated (stack based) `RadialColorStop` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzRadialColorStop {
        pub offset: AzOptionAngleValue,
        pub color: AzColorU,
    }
    /// Re-export of rust-allocated (stack based) `CssImageId` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzCssImageId {
        pub inner: AzString,
    }
    /// Re-export of rust-allocated (stack based) `StyleFontFamily` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzStyleFontFamily {
        pub fonts: AzStringVec,
    }
    /// Re-export of rust-allocated (stack based) `StyleTransform` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzStyleTransform {
        Matrix(AzStyleTransformMatrix2D),
        Matrix3D(AzStyleTransformMatrix3D),
        Translate(AzStyleTransformTranslate2D),
        Translate3D(AzStyleTransformTranslate3D),
        TranslateX(AzPixelValue),
        TranslateY(AzPixelValue),
        TranslateZ(AzPixelValue),
        Rotate(AzPercentageValue),
        Rotate3D(AzStyleTransformRotate3D),
        RotateX(AzPercentageValue),
        RotateY(AzPercentageValue),
        RotateZ(AzPercentageValue),
        Scale(AzStyleTransformScale2D),
        Scale3D(AzStyleTransformScale3D),
        ScaleX(AzPercentageValue),
        ScaleY(AzPercentageValue),
        ScaleZ(AzPercentageValue),
        Skew(AzStyleTransformSkew2D),
        SkewX(AzPercentageValue),
        SkewY(AzPercentageValue),
        Perspective(AzPixelValue),
    }
    /// Re-export of rust-allocated (stack based) `StyleBackgroundPositionVecValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzStyleBackgroundPositionVecValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleBackgroundPositionVec),
    }
    /// Re-export of rust-allocated (stack based) `StyleBackgroundSizeVecValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzStyleBackgroundSizeVecValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleBackgroundSizeVec),
    }
    /// Re-export of rust-allocated (stack based) `StyleFontFamilyValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzStyleFontFamilyValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleFontFamily),
    }
    /// Re-export of rust-allocated (stack based) `StyledNode` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzStyledNode {
        pub state: AzStyledNodeState,
        pub tag_id: AzOptionTagId,
    }
    /// Re-export of rust-allocated (stack based) `TagIdToNodeIdMapping` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzTagIdToNodeIdMapping {
        pub tag_id: AzTagId,
        pub node_id: AzNodeId,
        pub tab_index: AzOptionTabIndex,
    }
    /// Re-export of rust-allocated (stack based) `IdOrClass` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzIdOrClass {
        Id(AzString),
        Class(AzString),
    }
    /// List of core DOM node types built-into by `azul`
    #[repr(C, u8)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub enum AzNodeType {
        Div,
        Body,
        Br,
        Label(AzString),
        Image(AzImageId),
        IFrame(AzIFrameNode),
        GlTexture(AzGlTextureNode),
    }
    /// Re-export of rust-allocated (stack based) `EventFilter` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzEventFilter {
        Hover(AzHoverEventFilter),
        Not(AzNotEventFilter),
        Focus(AzFocusEventFilter),
        Window(AzWindowEventFilter),
        Component(AzComponentEventFilter),
        Application(AzApplicationEventFilter),
    }
    /// Re-export of rust-allocated (stack based) `VertexAttribute` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzVertexAttribute {
        pub name: AzString,
        pub layout_location: AzOptionUsize,
        pub attribute_type: AzVertexAttributeType,
        pub item_count: usize,
    }
    /// Re-export of rust-allocated (stack based) `DebugMessage` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzDebugMessage {
        pub message: AzString,
        pub source: u32,
        pub ty: u32,
        pub id: u32,
        pub severity: u32,
    }
    /// C-ABI stable reexport of `&[Refstr]` aka `&mut [&str]`
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzRefstrVecRef {
        pub(crate) ptr: *const AzRefstr,
        pub len: usize,
    }
    /// C-ABI stable reexport of `(i32, u32, AzString)`
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzGetActiveAttribReturn {
        pub _0: i32,
        pub _1: u32,
        pub _2: AzString,
    }
    /// C-ABI stable reexport of `(i32, u32, AzString)`
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzGetActiveUniformReturn {
        pub _0: i32,
        pub _1: u32,
        pub _2: AzString,
    }
    /// Re-export of rust-allocated (stack based) `ImageSource` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzImageSource {
        Embedded(AzU8Vec),
        File(AzString),
        Raw(AzRawImage),
    }
    /// Re-export of rust-allocated (stack based) `FontSource` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzFontSource {
        Embedded(AzU8Vec),
        File(AzString),
        System(AzString),
    }
    /// Re-export of rust-allocated (stack based) `SvgLine` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzSvgLine {
        pub start: AzSvgPoint,
        pub end: AzSvgPoint,
    }
    /// Re-export of rust-allocated (stack based) `TesselatedCPUSvgNode` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzTesselatedCPUSvgNode {
        pub vertices: AzSvgVertexVec,
        pub indices: AzU32Vec,
    }
    /// Re-export of rust-allocated (stack based) `SvgParseOptions` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzSvgParseOptions {
        pub relative_image_path: AzOptionString,
        pub dpi: f32,
        pub default_font_family: AzString,
        pub font_size: f32,
        pub languages: AzStringVec,
        pub shape_rendering: AzShapeRendering,
        pub text_rendering: AzTextRendering,
        pub image_rendering: AzImageRendering,
        pub keep_named_groups: bool,
        pub fontdb: AzFontDatabase,
    }
    /// Re-export of rust-allocated (stack based) `SvgRenderOptions` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzSvgRenderOptions {
        pub background_color: AzOptionColorU,
        pub fit: AzSvgFitTo,
    }
    /// Re-export of rust-allocated (stack based) `SvgStrokeStyle` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzSvgStrokeStyle {
        pub start_cap: AzSvgLineCap,
        pub end_cap: AzSvgLineCap,
        pub line_join: AzSvgLineJoin,
        pub dash_pattern: AzOptionSvgDashPattern,
        pub line_width: usize,
        pub miter_limit: usize,
        pub tolerance: usize,
        pub apply_line_width: bool,
    }
    /// Re-export of rust-allocated (stack based) `Thread` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzThread {
        pub thread_handle: *mut c_void,
        pub sender: *mut c_void,
        pub receiver: *mut c_void,
        pub writeback_data: AzRefAny,
        pub dropcheck: *mut c_void,
        pub check_thread_finished_fn: AzCheckThreadFinishedFn,
        pub send_thread_msg_fn: AzLibrarySendThreadMsgFn,
        pub receive_thread_msg_fn: AzLibraryReceiveThreadMsgFn,
        pub thread_destructor_fn: AzThreadDestructorFn,
    }
    /// Re-export of rust-allocated (stack based) `ThreadReceiveMsg` struct
    #[repr(C, u8)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub enum AzThreadReceiveMsg {
        WriteBack(AzThreadWriteBackMsg),
        Update(AzUpdateScreen),
    }
    /// Re-export of rust-allocated (stack based) `RawWindowHandle` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzRawWindowHandle {
        IOS(AzIOSHandle),
        MacOS(AzMacOSHandle),
        Xlib(AzXlibHandle),
        Xcb(AzXcbHandle),
        Wayland(AzWaylandHandle),
        Windows(AzWindowsHandle),
        Web(AzWebHandle),
        Android(AzAndroidHandle),
        Unsupported,
    }
    /// Re-export of rust-allocated (stack based) `TaskBarIcon` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzTaskBarIcon {
        pub key: AzIconKey,
        pub rgba_bytes: AzU8Vec,
    }
    /// Re-export of rust-allocated (stack based) `LogicalRect` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzLogicalRect {
        pub origin: AzLogicalPosition,
        pub size: AzLogicalSize,
    }
    /// Re-export of rust-allocated (stack based) `WindowSize` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub struct AzWindowSize {
        pub dimensions: AzLogicalSize,
        pub hidpi_factor: f32,
        pub system_hidpi_factor: f32,
        pub min_dimensions: AzOptionLogicalSize,
        pub max_dimensions: AzOptionLogicalSize,
    }
    /// Re-export of rust-allocated (stack based) `StringPair` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzStringPair {
        pub key: AzString,
        pub value: AzString,
    }
    /// Wrapper over a Rust-allocated `Vec<IdOrClass>`
    #[repr(C)]     pub struct AzIdOrClassVec {
        pub(crate) ptr: *const AzIdOrClass,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzIdOrClassVecDestructor,
    }
    /// Wrapper over a Rust-allocated `Vec<StyleTransform>`
    #[repr(C)]     pub struct AzStyleTransformVec {
        pub(crate) ptr: *const AzStyleTransform,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzStyleTransformVecDestructor,
    }
    /// Wrapper over a Rust-allocated `Vec<VertexAttribute>`
    #[repr(C)]     pub struct AzVertexAttributeVec {
        pub(crate) ptr: *const AzVertexAttribute,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzVertexAttributeVecDestructor,
    }
    /// Wrapper over a Rust-allocated `Vec<DebugMessage>`
    #[repr(C)]     pub struct AzDebugMessageVec {
        pub(crate) ptr: *const AzDebugMessage,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzDebugMessageVecDestructor,
    }
    /// Wrapper over a Rust-allocated `StringPairVec`
    #[repr(C)]     pub struct AzStringPairVec {
        pub(crate) ptr: *const AzStringPair,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzStringPairVecDestructor,
    }
    /// Wrapper over a Rust-allocated `LinearColorStopVec`
    #[repr(C)]     pub struct AzLinearColorStopVec {
        pub(crate) ptr: *const AzLinearColorStop,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzLinearColorStopVecDestructor,
    }
    /// Wrapper over a Rust-allocated `RadialColorStopVec`
    #[repr(C)]     pub struct AzRadialColorStopVec {
        pub(crate) ptr: *const AzRadialColorStop,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzRadialColorStopVecDestructor,
    }
    /// Wrapper over a Rust-allocated `StyledNodeVec`
    #[repr(C)]     pub struct AzStyledNodeVec {
        pub(crate) ptr: *const AzStyledNode,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzStyledNodeVecDestructor,
    }
    /// Wrapper over a Rust-allocated `TagIdsToNodeIdsMappingVec`
    #[repr(C)]     pub struct AzTagIdsToNodeIdsMappingVec {
        pub(crate) ptr: *const AzTagIdToNodeIdMapping,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzTagIdsToNodeIdsMappingVecDestructor,
    }
    /// Re-export of rust-allocated (stack based) `OptionThreadReceiveMsg` struct
    #[repr(C, u8)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub enum AzOptionThreadReceiveMsg {
        None,
        Some(AzThreadReceiveMsg),
    }
    /// Re-export of rust-allocated (stack based) `OptionTaskBarIcon` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzOptionTaskBarIcon {
        None,
        Some(AzTaskBarIcon),
    }
    /// Re-export of rust-allocated (stack based) `OptionDuration` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzOptionDuration {
        None,
        Some(AzDuration),
    }
    /// Re-export of rust-allocated (stack based) `OptionInstant` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzOptionInstant {
        None,
        Some(AzInstant),
    }
    /// Re-export of rust-allocated (stack based) `XmlStreamError` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzXmlStreamError {
        UnexpectedEndOfStream,
        InvalidName,
        NonXmlChar(AzNonXmlCharError),
        InvalidChar(AzInvalidCharError),
        InvalidCharMultiple(AzInvalidCharMultipleError),
        InvalidQuote(AzInvalidQuoteError),
        InvalidSpace(AzInvalidSpaceError),
        InvalidString(AzInvalidStringError),
        InvalidReference,
        InvalidExternalID,
        InvalidCommentData,
        InvalidCommentEnd,
        InvalidCharacterData,
    }
    /// Re-export of rust-allocated (stack based) `LayoutInfo` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzLayoutInfo {
        pub window_size: *const AzWindowSize,
        pub window_size_width_stops: *mut c_void,
        pub window_size_height_stops: *mut c_void,
        pub resources: *const c_void,
    }
    /// Re-export of rust-allocated (stack based) `CssPathPseudoSelector` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzCssPathPseudoSelector {
        First,
        Last,
        NthChild(AzCssNthChildSelector),
        Hover,
        Active,
        Focus,
    }
    /// Re-export of rust-allocated (stack based) `LinearGradient` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzLinearGradient {
        pub direction: AzDirection,
        pub extend_mode: AzExtendMode,
        pub stops: AzLinearColorStopVec,
    }
    /// Re-export of rust-allocated (stack based) `RadialGradient` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzRadialGradient {
        pub shape: AzShape,
        pub size: AzRadialGradientSize,
        pub position: AzStyleBackgroundPosition,
        pub extend_mode: AzExtendMode,
        pub stops: AzLinearColorStopVec,
    }
    /// Re-export of rust-allocated (stack based) `ConicGradient` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzConicGradient {
        pub extend_mode: AzExtendMode,
        pub center: AzStyleBackgroundPosition,
        pub angle: AzAngleValue,
        pub stops: AzRadialColorStopVec,
    }
    /// Re-export of rust-allocated (stack based) `StyleBackgroundContent` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzStyleBackgroundContent {
        LinearGradient(AzLinearGradient),
        RadialGradient(AzRadialGradient),
        ConicGradient(AzConicGradient),
        Image(AzCssImageId),
        Color(AzColorU),
    }
    /// Re-export of rust-allocated (stack based) `ScrollbarInfo` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzScrollbarInfo {
        pub width: AzLayoutWidth,
        pub padding_left: AzLayoutPaddingLeft,
        pub padding_right: AzLayoutPaddingRight,
        pub track: AzStyleBackgroundContent,
        pub thumb: AzStyleBackgroundContent,
        pub button: AzStyleBackgroundContent,
        pub corner: AzStyleBackgroundContent,
        pub resizer: AzStyleBackgroundContent,
    }
    /// Re-export of rust-allocated (stack based) `ScrollbarStyle` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzScrollbarStyle {
        pub horizontal: AzScrollbarInfo,
        pub vertical: AzScrollbarInfo,
    }
    /// Re-export of rust-allocated (stack based) `ScrollbarStyleValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzScrollbarStyleValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzScrollbarStyle),
    }
    /// Re-export of rust-allocated (stack based) `StyleTransformVecValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzStyleTransformVecValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleTransformVec),
    }
    /// Re-export of rust-allocated (stack based) `CallbackData` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzCallbackData {
        pub event: AzEventFilter,
        pub callback: AzCallback,
        pub data: AzRefAny,
    }
    /// Re-export of rust-allocated (stack based) `VertexLayout` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzVertexLayout {
        pub fields: AzVertexAttributeVec,
    }
    /// Re-export of rust-allocated (stack based) `VertexArrayObject` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzVertexArrayObject {
        pub vertex_layout: AzVertexLayout,
        pub vao_id: u32,
        pub gl_context: AzGlContextPtr,
    }
    /// Re-export of rust-allocated (stack based) `VertexBuffer` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzVertexBuffer {
        pub vertex_buffer_id: u32,
        pub vertex_buffer_len: usize,
        pub vao: AzVertexArrayObject,
        pub index_buffer_id: u32,
        pub index_buffer_len: usize,
        pub index_buffer_format: AzIndexBufferFormat,
    }
    /// Re-export of rust-allocated (stack based) `ImageMask` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzImageMask {
        pub image: AzImageId,
        pub rect: AzLogicalRect,
        pub repeat: bool,
    }
    /// Re-export of rust-allocated (stack based) `SvgPathElement` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzSvgPathElement {
        Line(AzSvgLine),
        QuadraticCurve(AzSvgQuadraticCurve),
        CubicCurve(AzSvgCubicCurve),
    }
    /// Re-export of rust-allocated (stack based) `SvgStyle` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)] #[derive(Copy)] pub enum AzSvgStyle {
        Fill(AzSvgFillStyle),
        Stroke(AzSvgStrokeStyle),
    }
    /// Re-export of rust-allocated (stack based) `Timer` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzTimer {
        pub data: AzRefAny,
        pub created: AzInstant,
        pub last_run: AzOptionInstant,
        pub run_count: usize,
        pub delay: AzOptionDuration,
        pub interval: AzOptionDuration,
        pub timeout: AzOptionDuration,
        pub callback: AzTimerCallback,
    }
    /// Re-export of rust-allocated (stack based) `WindowsWindowOptions` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzWindowsWindowOptions {
        pub allow_drag_drop: bool,
        pub no_redirection_bitmap: bool,
        pub window_icon: AzOptionWindowIcon,
        pub taskbar_icon: AzOptionTaskBarIcon,
        pub parent_window: AzOptionHwndHandle,
    }
    /// Re-export of rust-allocated (stack based) `LinuxWindowOptions` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzLinuxWindowOptions {
        pub x11_visual: AzOptionX11Visual,
        pub x11_screen: AzOptionI32,
        pub x11_wm_classes: AzStringPairVec,
        pub x11_override_redirect: bool,
        pub x11_window_types: AzXWindowTypeVec,
        pub x11_gtk_theme_variant: AzOptionString,
        pub x11_resize_increments: AzOptionLogicalSize,
        pub x11_base_size: AzOptionLogicalSize,
        pub wayland_app_id: AzOptionString,
        pub wayland_theme: AzOptionWaylandTheme,
        pub request_user_attention: bool,
        pub window_icon: AzOptionWindowIcon,
    }
    /// Wrapper over a Rust-allocated `Vec<StyleBackgroundContent>`
    #[repr(C)]     pub struct AzStyleBackgroundContentVec {
        pub(crate) ptr: *const AzStyleBackgroundContent,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzStyleBackgroundContentVecDestructor,
    }
    /// Wrapper over a Rust-allocated `VertexAttribute`
    #[repr(C)]     pub struct AzSvgPathElementVec {
        pub(crate) ptr: *const AzSvgPathElement,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzSvgPathElementVecDestructor,
    }
    /// Wrapper over a Rust-allocated `CallbackData`
    #[repr(C)]     pub struct AzCallbackDataVec {
        pub(crate) ptr: *const AzCallbackData,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzCallbackDataVecDestructor,
    }
    /// Re-export of rust-allocated (stack based) `OptionImageMask` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzOptionImageMask {
        None,
        Some(AzImageMask),
    }
    /// Re-export of rust-allocated (stack based) `XmlTextError` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzXmlTextError {
        pub stream_error: AzXmlStreamError,
        pub pos: AzSvgParseErrorPosition,
    }
    /// Re-export of rust-allocated (stack based) `CssPathSelector` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzCssPathSelector {
        Global,
        Type(AzNodeTypePath),
        Class(AzString),
        Id(AzString),
        PseudoSelector(AzCssPathPseudoSelector),
        DirectChildren,
        Children,
    }
    /// Re-export of rust-allocated (stack based) `StyleBackgroundContentVecValue` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzStyleBackgroundContentVecValue {
        Auto,
        None,
        Inherit,
        Initial,
        Exact(AzStyleBackgroundContentVec),
    }
    /// Parsed CSS key-value pair
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzCssProperty {
        TextColor(AzStyleTextColorValue),
        FontSize(AzStyleFontSizeValue),
        FontFamily(AzStyleFontFamilyValue),
        TextAlign(AzStyleTextAlignmentHorzValue),
        LetterSpacing(AzStyleLetterSpacingValue),
        LineHeight(AzStyleLineHeightValue),
        WordSpacing(AzStyleWordSpacingValue),
        TabWidth(AzStyleTabWidthValue),
        Cursor(AzStyleCursorValue),
        Display(AzLayoutDisplayValue),
        Float(AzLayoutFloatValue),
        BoxSizing(AzLayoutBoxSizingValue),
        Width(AzLayoutWidthValue),
        Height(AzLayoutHeightValue),
        MinWidth(AzLayoutMinWidthValue),
        MinHeight(AzLayoutMinHeightValue),
        MaxWidth(AzLayoutMaxWidthValue),
        MaxHeight(AzLayoutMaxHeightValue),
        Position(AzLayoutPositionValue),
        Top(AzLayoutTopValue),
        Right(AzLayoutRightValue),
        Left(AzLayoutLeftValue),
        Bottom(AzLayoutBottomValue),
        FlexWrap(AzLayoutFlexWrapValue),
        FlexDirection(AzLayoutFlexDirectionValue),
        FlexGrow(AzLayoutFlexGrowValue),
        FlexShrink(AzLayoutFlexShrinkValue),
        JustifyContent(AzLayoutJustifyContentValue),
        AlignItems(AzLayoutAlignItemsValue),
        AlignContent(AzLayoutAlignContentValue),
        BackgroundContent(AzStyleBackgroundContentVecValue),
        BackgroundPosition(AzStyleBackgroundPositionVecValue),
        BackgroundSize(AzStyleBackgroundSizeVecValue),
        BackgroundRepeat(AzStyleBackgroundRepeatVecValue),
        OverflowX(AzLayoutOverflowValue),
        OverflowY(AzLayoutOverflowValue),
        PaddingTop(AzLayoutPaddingTopValue),
        PaddingLeft(AzLayoutPaddingLeftValue),
        PaddingRight(AzLayoutPaddingRightValue),
        PaddingBottom(AzLayoutPaddingBottomValue),
        MarginTop(AzLayoutMarginTopValue),
        MarginLeft(AzLayoutMarginLeftValue),
        MarginRight(AzLayoutMarginRightValue),
        MarginBottom(AzLayoutMarginBottomValue),
        BorderTopLeftRadius(AzStyleBorderTopLeftRadiusValue),
        BorderTopRightRadius(AzStyleBorderTopRightRadiusValue),
        BorderBottomLeftRadius(AzStyleBorderBottomLeftRadiusValue),
        BorderBottomRightRadius(AzStyleBorderBottomRightRadiusValue),
        BorderTopColor(AzStyleBorderTopColorValue),
        BorderRightColor(AzStyleBorderRightColorValue),
        BorderLeftColor(AzStyleBorderLeftColorValue),
        BorderBottomColor(AzStyleBorderBottomColorValue),
        BorderTopStyle(AzStyleBorderTopStyleValue),
        BorderRightStyle(AzStyleBorderRightStyleValue),
        BorderLeftStyle(AzStyleBorderLeftStyleValue),
        BorderBottomStyle(AzStyleBorderBottomStyleValue),
        BorderTopWidth(AzLayoutBorderTopWidthValue),
        BorderRightWidth(AzLayoutBorderRightWidthValue),
        BorderLeftWidth(AzLayoutBorderLeftWidthValue),
        BorderBottomWidth(AzLayoutBorderBottomWidthValue),
        BoxShadowLeft(AzStyleBoxShadowValue),
        BoxShadowRight(AzStyleBoxShadowValue),
        BoxShadowTop(AzStyleBoxShadowValue),
        BoxShadowBottom(AzStyleBoxShadowValue),
        ScrollbarStyle(AzScrollbarStyleValue),
        Opacity(AzStyleOpacityValue),
        Transform(AzStyleTransformVecValue),
        TransformOrigin(AzStyleTransformOriginValue),
        PerspectiveOrigin(AzStylePerspectiveOriginValue),
        BackfaceVisibility(AzStyleBackfaceVisibilityValue),
    }
    /// Re-export of rust-allocated (stack based) `NodeDataInlineCssProperty` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzNodeDataInlineCssProperty {
        Normal(AzCssProperty),
        Active(AzCssProperty),
        Focus(AzCssProperty),
        Hover(AzCssProperty),
    }
    /// Re-export of rust-allocated (stack based) `SvgPath` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzSvgPath {
        pub items: AzSvgPathElementVec,
    }
    /// Re-export of rust-allocated (stack based) `PlatformSpecificOptions` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzPlatformSpecificOptions {
        pub windows_options: AzWindowsWindowOptions,
        pub linux_options: AzLinuxWindowOptions,
        pub mac_options: AzMacWindowOptions,
        pub wasm_options: AzWasmWindowOptions,
    }
    /// Re-export of rust-allocated (stack based) `WindowState` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzWindowState {
        pub theme: AzWindowTheme,
        pub title: AzString,
        pub size: AzWindowSize,
        pub position: AzWindowPosition,
        pub flags: AzWindowFlags,
        pub debug_state: AzDebugState,
        pub keyboard_state: AzKeyboardState,
        pub mouse_state: AzMouseState,
        pub touch_state: AzTouchState,
        pub ime_position: AzImePosition,
        pub platform_specific_options: AzPlatformSpecificOptions,
        pub renderer_options: AzRendererOptions,
        pub background_color: AzColorU,
        pub layout_callback: AzLayoutCallback,
        pub close_callback: AzOptionCallback,
    }
    /// Re-export of rust-allocated (stack based) `WindowCreateOptions` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzWindowCreateOptions {
        pub state: AzWindowState,
        pub renderer_type: AzOptionRendererOptions,
        pub theme: AzOptionWindowTheme,
        pub create_callback: AzOptionCallback,
    }
    /// Wrapper over a Rust-allocated `Vec<NodeDataInlineCssProperty>`
    #[repr(C)]     pub struct AzNodeDataInlineCssPropertyVec {
        pub(crate) ptr: *const AzNodeDataInlineCssProperty,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzNodeDataInlineCssPropertyVecDestructor,
    }
    /// Wrapper over a Rust-allocated `Vec<CssProperty>`
    #[repr(C)]     pub struct AzCssPropertyVec {
        pub(crate) ptr: *const AzCssProperty,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzCssPropertyVecDestructor,
    }
    /// Wrapper over a Rust-allocated `Vec<SvgPath>`
    #[repr(C)]     pub struct AzSvgPathVec {
        pub(crate) ptr: *const AzSvgPath,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzSvgPathVecDestructor,
    }
    /// Wrapper over a Rust-allocated `CssPathSelector`
    #[repr(C)]     pub struct AzCssPathSelectorVec {
        pub(crate) ptr: *const AzCssPathSelector,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzCssPathSelectorVecDestructor,
    }
    /// Re-export of rust-allocated (stack based) `XmlParseError` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzXmlParseError {
        InvalidDeclaration(AzXmlTextError),
        InvalidComment(AzXmlTextError),
        InvalidPI(AzXmlTextError),
        InvalidDoctype(AzXmlTextError),
        InvalidEntity(AzXmlTextError),
        InvalidElement(AzXmlTextError),
        InvalidAttribute(AzXmlTextError),
        InvalidCdata(AzXmlTextError),
        InvalidCharData(AzXmlTextError),
        UnknownToken(AzSvgParseErrorPosition),
    }
    /// Re-export of rust-allocated (stack based) `CallbackInfo` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzCallbackInfo {
        pub current_window_state: *const c_void,
        pub modifiable_window_state: *mut AzWindowState,
        pub gl_context: *const AzGlContextPtr,
        pub resources: *mut c_void,
        pub timers: *mut c_void,
        pub threads: *mut c_void,
        pub new_windows: *mut c_void,
        pub current_window_handle: *const AzRawWindowHandle,
        pub node_hierarchy: *const c_void,
        pub system_callbacks: *const AzSystemCallbacks,
        pub datasets: *mut c_void,
        pub stop_propagation: *mut bool,
        pub focus_target: *mut c_void,
        pub words_cache: *const c_void,
        pub shaped_words_cache: *const c_void,
        pub positioned_words_cache: *const c_void,
        pub positioned_rects: *const c_void,
        pub words_changed_in_callbacks: *mut c_void,
        pub images_changed_in_callbacks: *mut c_void,
        pub image_masks_changed_in_callbacks: *mut c_void,
        pub css_properties_changed_in_callbacks: *mut c_void,
        pub current_scroll_states: *const c_void,
        pub nodes_scrolled_in_callback: *mut c_void,
        pub hit_dom_node: AzDomNodeId,
        pub cursor_relative_to_item: AzOptionLayoutPoint,
        pub cursor_in_viewport: AzOptionLayoutPoint,
    }
    /// Re-export of rust-allocated (stack based) `TimerCallbackInfo` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzTimerCallbackInfo {
        pub callback_info: AzCallbackInfo,
        pub frame_start: AzInstant,
        pub call_count: usize,
        pub is_about_to_finish: bool,
    }
    /// Re-export of rust-allocated (stack based) `DynamicCssProperty` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzDynamicCssProperty {
        pub dynamic_id: AzString,
        pub default_value: AzCssProperty,
    }
    /// Re-export of rust-allocated (stack based) `CssPath` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzCssPath {
        pub selectors: AzCssPathSelectorVec,
    }
    /// Re-export of rust-allocated (stack based) `CssPropertySource` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzCssPropertySource {
        Css(AzCssPath),
        Inline,
    }
    /// Represents one single DOM node (node type, classes, ids and callbacks are stored here)
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzNodeData {
        pub node_type: AzNodeType,
        pub dataset: AzOptionRefAny,
        pub ids_and_classes: AzIdOrClassVec,
        pub callbacks: AzCallbackDataVec,
        pub inline_css_props: AzNodeDataInlineCssPropertyVec,
        pub clip_mask: AzOptionImageMask,
        pub tab_index: AzOptionTabIndex,
    }
    /// Re-export of rust-allocated (stack based) `SvgMultiPolygon` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzSvgMultiPolygon {
        pub rings: AzSvgPathVec,
    }
    /// Wrapper over a Rust-allocated `Vec<SvgMultiPolygon>`
    #[repr(C)]     pub struct AzSvgMultiPolygonVec {
        pub(crate) ptr: *const AzSvgMultiPolygon,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzSvgMultiPolygonVecDestructor,
    }
    /// Wrapper over a Rust-allocated `NodeDataVec`
    #[repr(C)]     pub struct AzNodeDataVec {
        pub(crate) ptr: *const AzNodeData,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzNodeDataVecDestructor,
    }
    /// Re-export of rust-allocated (stack based) `XmlError` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzXmlError {
        InvalidXmlPrefixUri(AzSvgParseErrorPosition),
        UnexpectedXmlUri(AzSvgParseErrorPosition),
        UnexpectedXmlnsUri(AzSvgParseErrorPosition),
        InvalidElementNamePrefix(AzSvgParseErrorPosition),
        DuplicatedNamespace(AzDuplicatedNamespaceError),
        UnknownNamespace(AzUnknownNamespaceError),
        UnexpectedCloseTag(AzUnexpectedCloseTagError),
        UnexpectedEntityCloseTag(AzSvgParseErrorPosition),
        UnknownEntityReference(AzUnknownEntityReferenceError),
        MalformedEntityReference(AzSvgParseErrorPosition),
        EntityReferenceLoop(AzSvgParseErrorPosition),
        InvalidAttributeValue(AzSvgParseErrorPosition),
        DuplicatedAttribute(AzDuplicatedAttributeError),
        NoRootNode,
        SizeLimit,
        ParserError(AzXmlParseError),
    }
    /// Re-export of rust-allocated (stack based) `FocusTargetPath` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzFocusTargetPath {
        pub dom: AzDomId,
        pub css_path: AzCssPath,
    }
    /// Re-export of rust-allocated (stack based) `CssDeclaration` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzCssDeclaration {
        Static(AzCssProperty),
        Dynamic(AzDynamicCssProperty),
    }
    /// Re-export of rust-allocated (stack based) `StyledDom` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzStyledDom {
        pub root: AzNodeId,
        pub node_hierarchy: AzNodeVec,
        pub node_data: AzNodeDataVec,
        pub styled_nodes: AzStyledNodeVec,
        pub cascade_info: AzCascadeInfoVec,
        pub tag_ids_to_node_ids: AzTagIdsToNodeIdsMappingVec,
        pub non_leaf_nodes: AzParentWithNodeDepthVec,
        pub css_property_cache: AzCssPropertyCache,
    }
    /// Re-export of rust-allocated (stack based) `Dom` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzDom {
        pub root: AzNodeData,
        pub children: AzDomVec,
        pub estimated_total_children: usize,
    }
    /// Re-export of rust-allocated (stack based) `SvgNode` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzSvgNode {
        MultiPolygonCollection(AzSvgMultiPolygonVec),
        MultiPolygon(AzSvgMultiPolygon),
        Path(AzSvgPath),
        Circle(AzSvgCircle),
        Rect(AzSvgRect),
    }
    /// Re-export of rust-allocated (stack based) `SvgStyledNode` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzSvgStyledNode {
        pub geometry: AzSvgNode,
        pub style: AzSvgStyle,
    }
    /// Wrapper over a Rust-allocated `Vec<Dom>`
    #[repr(C)]     pub struct AzDomVec {
        pub(crate) ptr: *const AzDom,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzDomVecDestructor,
    }
    /// Wrapper over a Rust-allocated `CssDeclaration`
    #[repr(C)]     pub struct AzCssDeclarationVec {
        pub(crate) ptr: *const AzCssDeclaration,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzCssDeclarationVecDestructor,
    }
    /// Re-export of rust-allocated (stack based) `OptionDom` struct
    #[repr(C, u8)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub enum AzOptionDom {
        None,
        Some(AzDom),
    }
    /// Re-export of rust-allocated (stack based) `SvgParseError` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzSvgParseError {
        InvalidFileSuffix,
        FileOpenFailed,
        NotAnUtf8Str,
        MalformedGZip,
        InvalidSize,
        ParsingFailed(AzXmlError),
    }
    /// Defines the focus target for the next frame
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzFocusTarget {
        Id(AzDomNodeId),
        Path(AzFocusTargetPath),
        Previous,
        Next,
        First,
        Last,
        NoFocus,
    }
    /// Re-export of rust-allocated (stack based) `IFrameCallbackReturn` struct
    #[repr(C)] #[derive(Debug)]  #[derive(PartialEq, PartialOrd)]  pub struct AzIFrameCallbackReturn {
        pub dom: AzStyledDom,
        pub size: AzLayoutRect,
        pub virtual_size: AzOptionLayoutRect,
    }
    /// Re-export of rust-allocated (stack based) `CssRuleBlock` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzCssRuleBlock {
        pub path: AzCssPath,
        pub declarations: AzCssDeclarationVec,
    }
    /// Wrapper over a Rust-allocated `CssRuleBlock`
    #[repr(C)]     pub struct AzCssRuleBlockVec {
        pub(crate) ptr: *const AzCssRuleBlock,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzCssRuleBlockVecDestructor,
    }
    /// Re-export of rust-allocated (stack based) `ResultSvgSvgParseError` struct
    #[repr(C, u8)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub enum AzResultSvgSvgParseError {
        Ok(AzSvg),
        Err(AzSvgParseError),
    }
    /// Re-export of rust-allocated (stack based) `Stylesheet` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzStylesheet {
        pub rules: AzCssRuleBlockVec,
    }
    /// Wrapper over a Rust-allocated `Stylesheet`
    #[repr(C)]     pub struct AzStylesheetVec {
        pub(crate) ptr: *const AzStylesheet,
        pub len: usize,
        pub cap: usize,
        pub destructor: AzStylesheetVecDestructor,
    }
    /// Re-export of rust-allocated (stack based) `Css` struct
    #[repr(C)] #[derive(Debug)] #[derive(Clone)] #[derive(PartialEq, PartialOrd)]  pub struct AzCss {
        pub stylesheets: AzStylesheetVec,
    }
    }

    #[cfg(not(feature = "link_static"))]    pub use self::structs::*;


    #[cfg(not(feature = "link_static"))]
    #[cfg_attr(target_os = "windows", link(name="azul.dll"))] // https://github.com/rust-lang/cargo/issues/9082
    #[cfg_attr(not(target_os = "windows"), link(name="azul"))] // https://github.com/rust-lang/cargo/issues/9082
    extern "C" {
        pub(crate) fn az_dom_vec_delete(_:  &mut AzDomVec);
        pub(crate) fn az_id_or_class_vec_delete(_:  &mut AzIdOrClassVec);
        pub(crate) fn az_node_data_inline_css_property_vec_delete(_:  &mut AzNodeDataInlineCssPropertyVec);
        pub(crate) fn az_style_background_content_vec_delete(_:  &mut AzStyleBackgroundContentVec);
        pub(crate) fn az_style_background_position_vec_delete(_:  &mut AzStyleBackgroundPositionVec);
        pub(crate) fn az_style_background_repeat_vec_delete(_:  &mut AzStyleBackgroundRepeatVec);
        pub(crate) fn az_style_background_size_vec_delete(_:  &mut AzStyleBackgroundSizeVec);
        pub(crate) fn az_style_transform_vec_delete(_:  &mut AzStyleTransformVec);
        pub(crate) fn az_css_property_vec_delete(_:  &mut AzCssPropertyVec);
        pub(crate) fn az_svg_multi_polygon_vec_delete(_:  &mut AzSvgMultiPolygonVec);
        pub(crate) fn az_svg_path_vec_delete(_:  &mut AzSvgPathVec);
        pub(crate) fn az_vertex_attribute_vec_delete(_:  &mut AzVertexAttributeVec);
        pub(crate) fn az_svg_path_element_vec_delete(_:  &mut AzSvgPathElementVec);
        pub(crate) fn az_svg_vertex_vec_delete(_:  &mut AzSvgVertexVec);
        pub(crate) fn az_u32_vec_delete(_:  &mut AzU32Vec);
        pub(crate) fn az_x_window_type_vec_delete(_:  &mut AzXWindowTypeVec);
        pub(crate) fn az_virtual_key_code_vec_delete(_:  &mut AzVirtualKeyCodeVec);
        pub(crate) fn az_cascade_info_vec_delete(_:  &mut AzCascadeInfoVec);
        pub(crate) fn az_scan_code_vec_delete(_:  &mut AzScanCodeVec);
        pub(crate) fn az_css_declaration_vec_delete(_:  &mut AzCssDeclarationVec);
        pub(crate) fn az_css_path_selector_vec_delete(_:  &mut AzCssPathSelectorVec);
        pub(crate) fn az_stylesheet_vec_delete(_:  &mut AzStylesheetVec);
        pub(crate) fn az_css_rule_block_vec_delete(_:  &mut AzCssRuleBlockVec);
        pub(crate) fn az_u8_vec_delete(_:  &mut AzU8Vec);
        pub(crate) fn az_callback_data_vec_delete(_:  &mut AzCallbackDataVec);
        pub(crate) fn az_debug_message_vec_delete(_:  &mut AzDebugMessageVec);
        pub(crate) fn az_g_luint_vec_delete(_:  &mut AzGLuintVec);
        pub(crate) fn az_g_lint_vec_delete(_:  &mut AzGLintVec);
        pub(crate) fn az_string_vec_delete(_:  &mut AzStringVec);
        pub(crate) fn az_string_pair_vec_delete(_:  &mut AzStringPairVec);
        pub(crate) fn az_linear_color_stop_vec_delete(_:  &mut AzLinearColorStopVec);
        pub(crate) fn az_radial_color_stop_vec_delete(_:  &mut AzRadialColorStopVec);
        pub(crate) fn az_node_id_vec_delete(_:  &mut AzNodeIdVec);
        pub(crate) fn az_node_vec_delete(_:  &mut AzNodeVec);
        pub(crate) fn az_styled_node_vec_delete(_:  &mut AzStyledNodeVec);
        pub(crate) fn az_tag_ids_to_node_ids_mapping_vec_delete(_:  &mut AzTagIdsToNodeIdsMappingVec);
        pub(crate) fn az_parent_with_node_depth_vec_delete(_:  &mut AzParentWithNodeDepthVec);
        pub(crate) fn az_node_data_vec_delete(_:  &mut AzNodeDataVec);
        pub(crate) fn az_instant_ptr_delete(_:  &mut AzInstantPtr);
        pub(crate) fn az_instant_ptr_deep_copy(_:  &AzInstantPtr) -> AzInstantPtr;
        pub(crate) fn az_app_config_default() -> AzAppConfig;
        pub(crate) fn az_app_new(_:  AzRefAny, _:  AzAppConfig) -> AzApp;
        pub(crate) fn az_app_add_window(_:  &mut AzApp, _:  AzWindowCreateOptions);
        pub(crate) fn az_app_run(_:  AzApp, _:  AzWindowCreateOptions);
        pub(crate) fn az_app_delete(_:  &mut AzApp);
        pub(crate) fn az_hidpi_adjusted_bounds_get_logical_size(_:  &AzHidpiAdjustedBounds) -> AzLogicalSize;
        pub(crate) fn az_hidpi_adjusted_bounds_get_physical_size(_:  &AzHidpiAdjustedBounds) -> AzPhysicalSizeU32;
        pub(crate) fn az_hidpi_adjusted_bounds_get_hidpi_factor(_:  &AzHidpiAdjustedBounds) -> f32;
        pub(crate) fn az_callback_info_get_hit_node(_:  &AzCallbackInfo) -> AzDomNodeId;
        pub(crate) fn az_callback_info_get_cursor_relative_to_viewport(_:  &AzCallbackInfo) -> AzOptionLayoutPoint;
        pub(crate) fn az_callback_info_get_cursor_relative_to_node(_:  &AzCallbackInfo) -> AzOptionLayoutPoint;
        pub(crate) fn az_callback_info_get_parent(_:  &mut AzCallbackInfo, _:  AzDomNodeId) -> AzOptionDomNodeId;
        pub(crate) fn az_callback_info_get_previous_sibling(_:  &mut AzCallbackInfo, _:  AzDomNodeId) -> AzOptionDomNodeId;
        pub(crate) fn az_callback_info_get_next_sibling(_:  &mut AzCallbackInfo, _:  AzDomNodeId) -> AzOptionDomNodeId;
        pub(crate) fn az_callback_info_get_first_child(_:  &mut AzCallbackInfo, _:  AzDomNodeId) -> AzOptionDomNodeId;
        pub(crate) fn az_callback_info_get_last_child(_:  &mut AzCallbackInfo, _:  AzDomNodeId) -> AzOptionDomNodeId;
        pub(crate) fn az_callback_info_get_window_state(_:  &AzCallbackInfo) -> AzWindowState;
        pub(crate) fn az_callback_info_get_keyboard_state(_:  &AzCallbackInfo) -> AzKeyboardState;
        pub(crate) fn az_callback_info_get_mouse_state(_:  &AzCallbackInfo) -> AzMouseState;
        pub(crate) fn az_callback_info_get_current_window_handle(_:  &AzCallbackInfo) -> AzRawWindowHandle;
        pub(crate) fn az_callback_info_get_gl_context(_:  &AzCallbackInfo) -> AzOptionGlContextPtr;
        pub(crate) fn az_callback_info_set_window_state(_:  &mut AzCallbackInfo, _:  AzWindowState);
        pub(crate) fn az_callback_info_set_focus(_:  &mut AzCallbackInfo, _:  AzFocusTarget);
        pub(crate) fn az_callback_info_set_css_property(_:  &mut AzCallbackInfo, _:  AzDomNodeId, _:  AzCssProperty);
        pub(crate) fn az_callback_info_stop_propagation(_:  &mut AzCallbackInfo);
        pub(crate) fn az_callback_info_create_window(_:  &mut AzCallbackInfo, _:  AzWindowCreateOptions);
        pub(crate) fn az_callback_info_start_thread(_:  &mut AzCallbackInfo, _:  AzThreadId, _:  AzRefAny, _:  AzRefAny, _:  AzThreadCallbackType);
        pub(crate) fn az_callback_info_start_timer(_:  &mut AzCallbackInfo, _:  AzTimerId, _:  AzTimer);
        pub(crate) fn az_gl_callback_info_get_gl_context(_:  &AzGlCallbackInfo) -> AzOptionGlContextPtr;
        pub(crate) fn az_gl_callback_info_get_bounds(_:  &AzGlCallbackInfo) -> AzHidpiAdjustedBounds;
        pub(crate) fn az_ref_count_can_be_shared(_:  &AzRefCount) -> bool;
        pub(crate) fn az_ref_count_can_be_shared_mut(_:  &AzRefCount) -> bool;
        pub(crate) fn az_ref_count_increase_ref(_:  &mut AzRefCount);
        pub(crate) fn az_ref_count_decrease_ref(_:  &mut AzRefCount);
        pub(crate) fn az_ref_count_increase_refmut(_:  &mut AzRefCount);
        pub(crate) fn az_ref_count_decrease_refmut(_:  &mut AzRefCount);
        pub(crate) fn az_ref_count_delete(_:  &mut AzRefCount);
        pub(crate) fn az_ref_count_deep_copy(_:  &AzRefCount) -> AzRefCount;
        pub(crate) fn az_ref_any_new_c(_:  *const c_void, _:  usize, _:  u64, _:  AzString, _:  AzRefAnyDestructorType) -> AzRefAny;
        pub(crate) fn az_ref_any_is_type(_:  &AzRefAny, _:  u64) -> bool;
        pub(crate) fn az_ref_any_get_type_name(_:  &AzRefAny) -> AzString;
        pub(crate) fn az_ref_any_clone(_:  &mut AzRefAny) -> AzRefAny;
        pub(crate) fn az_ref_any_delete(_:  &mut AzRefAny);
        pub(crate) fn az_layout_info_window_width_larger_than(_:  &mut AzLayoutInfo, _:  f32) -> bool;
        pub(crate) fn az_layout_info_window_width_smaller_than(_:  &mut AzLayoutInfo, _:  f32) -> bool;
        pub(crate) fn az_layout_info_window_height_larger_than(_:  &mut AzLayoutInfo, _:  f32) -> bool;
        pub(crate) fn az_layout_info_window_height_smaller_than(_:  &mut AzLayoutInfo, _:  f32) -> bool;
        pub(crate) fn az_css_empty() -> AzCss;
        pub(crate) fn az_css_from_string(_:  AzString) -> AzCss;
        pub(crate) fn az_color_u_from_str(_:  AzString) -> AzColorU;
        pub(crate) fn az_color_u_to_hash(_:  &AzColorU) -> AzString;
        pub(crate) fn az_css_property_cache_delete(_:  &mut AzCssPropertyCache);
        pub(crate) fn az_css_property_cache_deep_copy(_:  &AzCssPropertyCache) -> AzCssPropertyCache;
        pub(crate) fn az_styled_dom_new(_:  AzDom, _:  AzCss) -> AzStyledDom;
        pub(crate) fn az_styled_dom_append(_:  &mut AzStyledDom, _:  AzStyledDom);
        pub(crate) fn az_styled_dom_node_count(_:  &AzStyledDom) -> usize;
        pub(crate) fn az_dom_node_count(_:  &AzDom) -> usize;
        pub(crate) fn az_on_into_event_filter(_:  AzOn) -> AzEventFilter;
        pub(crate) fn az_g_lsync_ptr_delete(_:  &mut AzGLsyncPtr);
        pub(crate) fn az_gl_context_ptr_get_type(_:  &AzGlContextPtr) -> AzGlType;
        pub(crate) fn az_gl_context_ptr_buffer_data_untyped(_:  &AzGlContextPtr, _:  u32, _:  isize, _:  *const c_void, _:  u32);
        pub(crate) fn az_gl_context_ptr_buffer_sub_data_untyped(_:  &AzGlContextPtr, _:  u32, _:  isize, _:  isize, _:  *const c_void);
        pub(crate) fn az_gl_context_ptr_map_buffer(_:  &AzGlContextPtr, _:  u32, _:  u32) -> *mut c_void;
        pub(crate) fn az_gl_context_ptr_map_buffer_range(_:  &AzGlContextPtr, _:  u32, _:  isize, _:  isize, _:  u32) -> *mut c_void;
        pub(crate) fn az_gl_context_ptr_unmap_buffer(_:  &AzGlContextPtr, _:  u32) -> u8;
        pub(crate) fn az_gl_context_ptr_tex_buffer(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_shader_source(_:  &AzGlContextPtr, _:  u32, _:  AzStringVec);
        pub(crate) fn az_gl_context_ptr_read_buffer(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_read_pixels_into_buffer(_:  &AzGlContextPtr, _:  i32, _:  i32, _:  i32, _:  i32, _:  u32, _:  u32, _:  AzU8VecRefMut);
        pub(crate) fn az_gl_context_ptr_read_pixels(_:  &AzGlContextPtr, _:  i32, _:  i32, _:  i32, _:  i32, _:  u32, _:  u32) -> AzU8Vec;
        pub(crate) fn az_gl_context_ptr_read_pixels_into_pbo(_:  &AzGlContextPtr, _:  i32, _:  i32, _:  i32, _:  i32, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_sample_coverage(_:  &AzGlContextPtr, _:  f32, _:  bool);
        pub(crate) fn az_gl_context_ptr_polygon_offset(_:  &AzGlContextPtr, _:  f32, _:  f32);
        pub(crate) fn az_gl_context_ptr_pixel_store_i(_:  &AzGlContextPtr, _:  u32, _:  i32);
        pub(crate) fn az_gl_context_ptr_gen_buffers(_:  &AzGlContextPtr, _:  i32) -> AzGLuintVec;
        pub(crate) fn az_gl_context_ptr_gen_renderbuffers(_:  &AzGlContextPtr, _:  i32) -> AzGLuintVec;
        pub(crate) fn az_gl_context_ptr_gen_framebuffers(_:  &AzGlContextPtr, _:  i32) -> AzGLuintVec;
        pub(crate) fn az_gl_context_ptr_gen_textures(_:  &AzGlContextPtr, _:  i32) -> AzGLuintVec;
        pub(crate) fn az_gl_context_ptr_gen_vertex_arrays(_:  &AzGlContextPtr, _:  i32) -> AzGLuintVec;
        pub(crate) fn az_gl_context_ptr_gen_queries(_:  &AzGlContextPtr, _:  i32) -> AzGLuintVec;
        pub(crate) fn az_gl_context_ptr_begin_query(_:  &AzGlContextPtr, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_end_query(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_query_counter(_:  &AzGlContextPtr, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_get_query_object_iv(_:  &AzGlContextPtr, _:  u32, _:  u32) -> i32;
        pub(crate) fn az_gl_context_ptr_get_query_object_uiv(_:  &AzGlContextPtr, _:  u32, _:  u32) -> u32;
        pub(crate) fn az_gl_context_ptr_get_query_object_i64v(_:  &AzGlContextPtr, _:  u32, _:  u32) -> i64;
        pub(crate) fn az_gl_context_ptr_get_query_object_ui64v(_:  &AzGlContextPtr, _:  u32, _:  u32) -> u64;
        pub(crate) fn az_gl_context_ptr_delete_queries(_:  &AzGlContextPtr, _:  AzGLuintVecRef);
        pub(crate) fn az_gl_context_ptr_delete_vertex_arrays(_:  &AzGlContextPtr, _:  AzGLuintVecRef);
        pub(crate) fn az_gl_context_ptr_delete_buffers(_:  &AzGlContextPtr, _:  AzGLuintVecRef);
        pub(crate) fn az_gl_context_ptr_delete_renderbuffers(_:  &AzGlContextPtr, _:  AzGLuintVecRef);
        pub(crate) fn az_gl_context_ptr_delete_framebuffers(_:  &AzGlContextPtr, _:  AzGLuintVecRef);
        pub(crate) fn az_gl_context_ptr_delete_textures(_:  &AzGlContextPtr, _:  AzGLuintVecRef);
        pub(crate) fn az_gl_context_ptr_framebuffer_renderbuffer(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_renderbuffer_storage(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  i32, _:  i32);
        pub(crate) fn az_gl_context_ptr_depth_func(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_active_texture(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_attach_shader(_:  &AzGlContextPtr, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_bind_attrib_location(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  AzRefstr);
        pub(crate) fn az_gl_context_ptr_get_uniform_iv(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  AzGLintVecRefMut);
        pub(crate) fn az_gl_context_ptr_get_uniform_fv(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  AzGLfloatVecRefMut);
        pub(crate) fn az_gl_context_ptr_get_uniform_block_index(_:  &AzGlContextPtr, _:  u32, _:  AzRefstr) -> u32;
        pub(crate) fn az_gl_context_ptr_get_uniform_indices(_:  &AzGlContextPtr, _:  u32, _:  AzRefstrVecRef) -> AzGLuintVec;
        pub(crate) fn az_gl_context_ptr_bind_buffer_base(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_bind_buffer_range(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  u32, _:  isize, _:  isize);
        pub(crate) fn az_gl_context_ptr_uniform_block_binding(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_bind_buffer(_:  &AzGlContextPtr, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_bind_vertex_array(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_bind_renderbuffer(_:  &AzGlContextPtr, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_bind_framebuffer(_:  &AzGlContextPtr, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_bind_texture(_:  &AzGlContextPtr, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_draw_buffers(_:  &AzGlContextPtr, _:  AzGLenumVecRef);
        pub(crate) fn az_gl_context_ptr_tex_image_2d(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  u32, _:  u32, _:  AzOptionU8VecRef);
        pub(crate) fn az_gl_context_ptr_compressed_tex_image_2d(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  u32, _:  i32, _:  i32, _:  i32, _:  AzU8VecRef);
        pub(crate) fn az_gl_context_ptr_compressed_tex_sub_image_2d(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  u32, _:  AzU8VecRef);
        pub(crate) fn az_gl_context_ptr_tex_image_3d(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  u32, _:  u32, _:  AzOptionU8VecRef);
        pub(crate) fn az_gl_context_ptr_copy_tex_image_2d(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  u32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32);
        pub(crate) fn az_gl_context_ptr_copy_tex_sub_image_2d(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32);
        pub(crate) fn az_gl_context_ptr_copy_tex_sub_image_3d(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32);
        pub(crate) fn az_gl_context_ptr_tex_sub_image_2d(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  u32, _:  u32, _:  AzU8VecRef);
        pub(crate) fn az_gl_context_ptr_tex_sub_image_2d_pbo(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  u32, _:  u32, _:  usize);
        pub(crate) fn az_gl_context_ptr_tex_sub_image_3d(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  u32, _:  u32, _:  AzU8VecRef);
        pub(crate) fn az_gl_context_ptr_tex_sub_image_3d_pbo(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  u32, _:  u32, _:  usize);
        pub(crate) fn az_gl_context_ptr_tex_storage_2d(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  u32, _:  i32, _:  i32);
        pub(crate) fn az_gl_context_ptr_tex_storage_3d(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  u32, _:  i32, _:  i32, _:  i32);
        pub(crate) fn az_gl_context_ptr_get_tex_image_into_buffer(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  u32, _:  u32, _:  AzU8VecRefMut);
        pub(crate) fn az_gl_context_ptr_copy_image_sub_data(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  i32, _:  i32, _:  i32, _:  i32, _:  u32, _:  u32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32);
        pub(crate) fn az_gl_context_ptr_invalidate_framebuffer(_:  &AzGlContextPtr, _:  u32, _:  AzGLenumVecRef);
        pub(crate) fn az_gl_context_ptr_invalidate_sub_framebuffer(_:  &AzGlContextPtr, _:  u32, _:  AzGLenumVecRef, _:  i32, _:  i32, _:  i32, _:  i32);
        pub(crate) fn az_gl_context_ptr_get_integer_v(_:  &AzGlContextPtr, _:  u32, _:  AzGLintVecRefMut);
        pub(crate) fn az_gl_context_ptr_get_integer_64v(_:  &AzGlContextPtr, _:  u32, _:  AzGLint64VecRefMut);
        pub(crate) fn az_gl_context_ptr_get_integer_iv(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  AzGLintVecRefMut);
        pub(crate) fn az_gl_context_ptr_get_integer_64iv(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  AzGLint64VecRefMut);
        pub(crate) fn az_gl_context_ptr_get_boolean_v(_:  &AzGlContextPtr, _:  u32, _:  AzGLbooleanVecRefMut);
        pub(crate) fn az_gl_context_ptr_get_float_v(_:  &AzGlContextPtr, _:  u32, _:  AzGLfloatVecRefMut);
        pub(crate) fn az_gl_context_ptr_get_framebuffer_attachment_parameter_iv(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  u32) -> i32;
        pub(crate) fn az_gl_context_ptr_get_renderbuffer_parameter_iv(_:  &AzGlContextPtr, _:  u32, _:  u32) -> i32;
        pub(crate) fn az_gl_context_ptr_get_tex_parameter_iv(_:  &AzGlContextPtr, _:  u32, _:  u32) -> i32;
        pub(crate) fn az_gl_context_ptr_get_tex_parameter_fv(_:  &AzGlContextPtr, _:  u32, _:  u32) -> f32;
        pub(crate) fn az_gl_context_ptr_tex_parameter_i(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  i32);
        pub(crate) fn az_gl_context_ptr_tex_parameter_f(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  f32);
        pub(crate) fn az_gl_context_ptr_framebuffer_texture_2d(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  u32, _:  u32, _:  i32);
        pub(crate) fn az_gl_context_ptr_framebuffer_texture_layer(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  u32, _:  i32, _:  i32);
        pub(crate) fn az_gl_context_ptr_blit_framebuffer(_:  &AzGlContextPtr, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_vertex_attrib_4f(_:  &AzGlContextPtr, _:  u32, _:  f32, _:  f32, _:  f32, _:  f32);
        pub(crate) fn az_gl_context_ptr_vertex_attrib_pointer_f32(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  bool, _:  i32, _:  u32);
        pub(crate) fn az_gl_context_ptr_vertex_attrib_pointer(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  u32, _:  bool, _:  i32, _:  u32);
        pub(crate) fn az_gl_context_ptr_vertex_attrib_i_pointer(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  u32, _:  i32, _:  u32);
        pub(crate) fn az_gl_context_ptr_vertex_attrib_divisor(_:  &AzGlContextPtr, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_viewport(_:  &AzGlContextPtr, _:  i32, _:  i32, _:  i32, _:  i32);
        pub(crate) fn az_gl_context_ptr_scissor(_:  &AzGlContextPtr, _:  i32, _:  i32, _:  i32, _:  i32);
        pub(crate) fn az_gl_context_ptr_line_width(_:  &AzGlContextPtr, _:  f32);
        pub(crate) fn az_gl_context_ptr_use_program(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_validate_program(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_draw_arrays(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  i32);
        pub(crate) fn az_gl_context_ptr_draw_arrays_instanced(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  i32, _:  i32);
        pub(crate) fn az_gl_context_ptr_draw_elements(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_draw_elements_instanced(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  u32, _:  u32, _:  i32);
        pub(crate) fn az_gl_context_ptr_blend_color(_:  &AzGlContextPtr, _:  f32, _:  f32, _:  f32, _:  f32);
        pub(crate) fn az_gl_context_ptr_blend_func(_:  &AzGlContextPtr, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_blend_func_separate(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_blend_equation(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_blend_equation_separate(_:  &AzGlContextPtr, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_color_mask(_:  &AzGlContextPtr, _:  bool, _:  bool, _:  bool, _:  bool);
        pub(crate) fn az_gl_context_ptr_cull_face(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_front_face(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_enable(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_disable(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_hint(_:  &AzGlContextPtr, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_is_enabled(_:  &AzGlContextPtr, _:  u32) -> u8;
        pub(crate) fn az_gl_context_ptr_is_shader(_:  &AzGlContextPtr, _:  u32) -> u8;
        pub(crate) fn az_gl_context_ptr_is_texture(_:  &AzGlContextPtr, _:  u32) -> u8;
        pub(crate) fn az_gl_context_ptr_is_framebuffer(_:  &AzGlContextPtr, _:  u32) -> u8;
        pub(crate) fn az_gl_context_ptr_is_renderbuffer(_:  &AzGlContextPtr, _:  u32) -> u8;
        pub(crate) fn az_gl_context_ptr_check_frame_buffer_status(_:  &AzGlContextPtr, _:  u32) -> u32;
        pub(crate) fn az_gl_context_ptr_enable_vertex_attrib_array(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_disable_vertex_attrib_array(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_uniform_1f(_:  &AzGlContextPtr, _:  i32, _:  f32);
        pub(crate) fn az_gl_context_ptr_uniform_1fv(_:  &AzGlContextPtr, _:  i32, _:  AzF32VecRef);
        pub(crate) fn az_gl_context_ptr_uniform_1i(_:  &AzGlContextPtr, _:  i32, _:  i32);
        pub(crate) fn az_gl_context_ptr_uniform_1iv(_:  &AzGlContextPtr, _:  i32, _:  AzI32VecRef);
        pub(crate) fn az_gl_context_ptr_uniform_1ui(_:  &AzGlContextPtr, _:  i32, _:  u32);
        pub(crate) fn az_gl_context_ptr_uniform_2f(_:  &AzGlContextPtr, _:  i32, _:  f32, _:  f32);
        pub(crate) fn az_gl_context_ptr_uniform_2fv(_:  &AzGlContextPtr, _:  i32, _:  AzF32VecRef);
        pub(crate) fn az_gl_context_ptr_uniform_2i(_:  &AzGlContextPtr, _:  i32, _:  i32, _:  i32);
        pub(crate) fn az_gl_context_ptr_uniform_2iv(_:  &AzGlContextPtr, _:  i32, _:  AzI32VecRef);
        pub(crate) fn az_gl_context_ptr_uniform_2ui(_:  &AzGlContextPtr, _:  i32, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_uniform_3f(_:  &AzGlContextPtr, _:  i32, _:  f32, _:  f32, _:  f32);
        pub(crate) fn az_gl_context_ptr_uniform_3fv(_:  &AzGlContextPtr, _:  i32, _:  AzF32VecRef);
        pub(crate) fn az_gl_context_ptr_uniform_3i(_:  &AzGlContextPtr, _:  i32, _:  i32, _:  i32, _:  i32);
        pub(crate) fn az_gl_context_ptr_uniform_3iv(_:  &AzGlContextPtr, _:  i32, _:  AzI32VecRef);
        pub(crate) fn az_gl_context_ptr_uniform_3ui(_:  &AzGlContextPtr, _:  i32, _:  u32, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_uniform_4f(_:  &AzGlContextPtr, _:  i32, _:  f32, _:  f32, _:  f32, _:  f32);
        pub(crate) fn az_gl_context_ptr_uniform_4i(_:  &AzGlContextPtr, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32);
        pub(crate) fn az_gl_context_ptr_uniform_4iv(_:  &AzGlContextPtr, _:  i32, _:  AzI32VecRef);
        pub(crate) fn az_gl_context_ptr_uniform_4ui(_:  &AzGlContextPtr, _:  i32, _:  u32, _:  u32, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_uniform_4fv(_:  &AzGlContextPtr, _:  i32, _:  AzF32VecRef);
        pub(crate) fn az_gl_context_ptr_uniform_matrix_2fv(_:  &AzGlContextPtr, _:  i32, _:  bool, _:  AzF32VecRef);
        pub(crate) fn az_gl_context_ptr_uniform_matrix_3fv(_:  &AzGlContextPtr, _:  i32, _:  bool, _:  AzF32VecRef);
        pub(crate) fn az_gl_context_ptr_uniform_matrix_4fv(_:  &AzGlContextPtr, _:  i32, _:  bool, _:  AzF32VecRef);
        pub(crate) fn az_gl_context_ptr_depth_mask(_:  &AzGlContextPtr, _:  bool);
        pub(crate) fn az_gl_context_ptr_depth_range(_:  &AzGlContextPtr, _:  f64, _:  f64);
        pub(crate) fn az_gl_context_ptr_get_active_attrib(_:  &AzGlContextPtr, _:  u32, _:  u32) -> AzGetActiveAttribReturn;
        pub(crate) fn az_gl_context_ptr_get_active_uniform(_:  &AzGlContextPtr, _:  u32, _:  u32) -> AzGetActiveUniformReturn;
        pub(crate) fn az_gl_context_ptr_get_active_uniforms_iv(_:  &AzGlContextPtr, _:  u32, _:  AzGLuintVec, _:  u32) -> AzGLintVec;
        pub(crate) fn az_gl_context_ptr_get_active_uniform_block_i(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  u32) -> i32;
        pub(crate) fn az_gl_context_ptr_get_active_uniform_block_iv(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  u32) -> AzGLintVec;
        pub(crate) fn az_gl_context_ptr_get_active_uniform_block_name(_:  &AzGlContextPtr, _:  u32, _:  u32) -> AzString;
        pub(crate) fn az_gl_context_ptr_get_attrib_location(_:  &AzGlContextPtr, _:  u32, _:  AzRefstr) -> i32;
        pub(crate) fn az_gl_context_ptr_get_frag_data_location(_:  &AzGlContextPtr, _:  u32, _:  AzRefstr) -> i32;
        pub(crate) fn az_gl_context_ptr_get_uniform_location(_:  &AzGlContextPtr, _:  u32, _:  AzRefstr) -> i32;
        pub(crate) fn az_gl_context_ptr_get_program_info_log(_:  &AzGlContextPtr, _:  u32) -> AzString;
        pub(crate) fn az_gl_context_ptr_get_program_iv(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  AzGLintVecRefMut);
        pub(crate) fn az_gl_context_ptr_get_program_binary(_:  &AzGlContextPtr, _:  u32) -> AzGetProgramBinaryReturn;
        pub(crate) fn az_gl_context_ptr_program_binary(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  AzU8VecRef);
        pub(crate) fn az_gl_context_ptr_program_parameter_i(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  i32);
        pub(crate) fn az_gl_context_ptr_get_vertex_attrib_iv(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  AzGLintVecRefMut);
        pub(crate) fn az_gl_context_ptr_get_vertex_attrib_fv(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  AzGLfloatVecRefMut);
        pub(crate) fn az_gl_context_ptr_get_vertex_attrib_pointer_v(_:  &AzGlContextPtr, _:  u32, _:  u32) -> isize;
        pub(crate) fn az_gl_context_ptr_get_buffer_parameter_iv(_:  &AzGlContextPtr, _:  u32, _:  u32) -> i32;
        pub(crate) fn az_gl_context_ptr_get_shader_info_log(_:  &AzGlContextPtr, _:  u32) -> AzString;
        pub(crate) fn az_gl_context_ptr_get_string(_:  &AzGlContextPtr, _:  u32) -> AzString;
        pub(crate) fn az_gl_context_ptr_get_string_i(_:  &AzGlContextPtr, _:  u32, _:  u32) -> AzString;
        pub(crate) fn az_gl_context_ptr_get_shader_iv(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  AzGLintVecRefMut);
        pub(crate) fn az_gl_context_ptr_get_shader_precision_format(_:  &AzGlContextPtr, _:  u32, _:  u32) -> AzGlShaderPrecisionFormatReturn;
        pub(crate) fn az_gl_context_ptr_compile_shader(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_create_program(_:  &AzGlContextPtr) -> u32;
        pub(crate) fn az_gl_context_ptr_delete_program(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_create_shader(_:  &AzGlContextPtr, _:  u32) -> u32;
        pub(crate) fn az_gl_context_ptr_delete_shader(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_detach_shader(_:  &AzGlContextPtr, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_link_program(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_clear_color(_:  &AzGlContextPtr, _:  f32, _:  f32, _:  f32, _:  f32);
        pub(crate) fn az_gl_context_ptr_clear(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_clear_depth(_:  &AzGlContextPtr, _:  f64);
        pub(crate) fn az_gl_context_ptr_clear_stencil(_:  &AzGlContextPtr, _:  i32);
        pub(crate) fn az_gl_context_ptr_flush(_:  &AzGlContextPtr);
        pub(crate) fn az_gl_context_ptr_finish(_:  &AzGlContextPtr);
        pub(crate) fn az_gl_context_ptr_get_error(_:  &AzGlContextPtr) -> u32;
        pub(crate) fn az_gl_context_ptr_stencil_mask(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_stencil_mask_separate(_:  &AzGlContextPtr, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_stencil_func(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  u32);
        pub(crate) fn az_gl_context_ptr_stencil_func_separate(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  i32, _:  u32);
        pub(crate) fn az_gl_context_ptr_stencil_op(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_stencil_op_separate(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_egl_image_target_texture2d_oes(_:  &AzGlContextPtr, _:  u32, _:  *const c_void);
        pub(crate) fn az_gl_context_ptr_generate_mipmap(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_insert_event_marker_ext(_:  &AzGlContextPtr, _:  AzRefstr);
        pub(crate) fn az_gl_context_ptr_push_group_marker_ext(_:  &AzGlContextPtr, _:  AzRefstr);
        pub(crate) fn az_gl_context_ptr_pop_group_marker_ext(_:  &AzGlContextPtr);
        pub(crate) fn az_gl_context_ptr_debug_message_insert_khr(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  u32, _:  u32, _:  AzRefstr);
        pub(crate) fn az_gl_context_ptr_push_debug_group_khr(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  AzRefstr);
        pub(crate) fn az_gl_context_ptr_pop_debug_group_khr(_:  &AzGlContextPtr);
        pub(crate) fn az_gl_context_ptr_fence_sync(_:  &AzGlContextPtr, _:  u32, _:  u32) -> AzGLsyncPtr;
        pub(crate) fn az_gl_context_ptr_client_wait_sync(_:  &AzGlContextPtr, _:  AzGLsyncPtr, _:  u32, _:  u64) -> u32;
        pub(crate) fn az_gl_context_ptr_wait_sync(_:  &AzGlContextPtr, _:  AzGLsyncPtr, _:  u32, _:  u64);
        pub(crate) fn az_gl_context_ptr_delete_sync(_:  &AzGlContextPtr, _:  AzGLsyncPtr);
        pub(crate) fn az_gl_context_ptr_texture_range_apple(_:  &AzGlContextPtr, _:  u32, _:  AzU8VecRef);
        pub(crate) fn az_gl_context_ptr_gen_fences_apple(_:  &AzGlContextPtr, _:  i32) -> AzGLuintVec;
        pub(crate) fn az_gl_context_ptr_delete_fences_apple(_:  &AzGlContextPtr, _:  AzGLuintVecRef);
        pub(crate) fn az_gl_context_ptr_set_fence_apple(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_finish_fence_apple(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_test_fence_apple(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_test_object_apple(_:  &AzGlContextPtr, _:  u32, _:  u32) -> u8;
        pub(crate) fn az_gl_context_ptr_finish_object_apple(_:  &AzGlContextPtr, _:  u32, _:  u32);
        pub(crate) fn az_gl_context_ptr_get_frag_data_index(_:  &AzGlContextPtr, _:  u32, _:  AzRefstr) -> i32;
        pub(crate) fn az_gl_context_ptr_blend_barrier_khr(_:  &AzGlContextPtr);
        pub(crate) fn az_gl_context_ptr_bind_frag_data_location_indexed(_:  &AzGlContextPtr, _:  u32, _:  u32, _:  u32, _:  AzRefstr);
        pub(crate) fn az_gl_context_ptr_get_debug_messages(_:  &AzGlContextPtr) -> AzDebugMessageVec;
        pub(crate) fn az_gl_context_ptr_provoking_vertex_angle(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_gen_vertex_arrays_apple(_:  &AzGlContextPtr, _:  i32) -> AzGLuintVec;
        pub(crate) fn az_gl_context_ptr_bind_vertex_array_apple(_:  &AzGlContextPtr, _:  u32);
        pub(crate) fn az_gl_context_ptr_delete_vertex_arrays_apple(_:  &AzGlContextPtr, _:  AzGLuintVecRef);
        pub(crate) fn az_gl_context_ptr_copy_texture_chromium(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  u32, _:  u32, _:  i32, _:  i32, _:  u32, _:  u8, _:  u8, _:  u8);
        pub(crate) fn az_gl_context_ptr_copy_sub_texture_chromium(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  u32, _:  u32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  u8, _:  u8, _:  u8);
        pub(crate) fn az_gl_context_ptr_egl_image_target_renderbuffer_storage_oes(_:  &AzGlContextPtr, _:  u32, _:  *const c_void);
        pub(crate) fn az_gl_context_ptr_copy_texture_3d_angle(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  u32, _:  u32, _:  i32, _:  i32, _:  u32, _:  u8, _:  u8, _:  u8);
        pub(crate) fn az_gl_context_ptr_copy_sub_texture_3d_angle(_:  &AzGlContextPtr, _:  u32, _:  i32, _:  u32, _:  u32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  i32, _:  u8, _:  u8, _:  u8);
        pub(crate) fn az_gl_context_ptr_buffer_storage(_:  &AzGlContextPtr, _:  u32, _:  isize, _:  *const c_void, _:  u32);
        pub(crate) fn az_gl_context_ptr_flush_mapped_buffer_range(_:  &AzGlContextPtr, _:  u32, _:  isize, _:  isize);
        pub(crate) fn az_gl_context_ptr_delete(_:  &mut AzGlContextPtr);
        pub(crate) fn az_gl_context_ptr_deep_copy(_:  &AzGlContextPtr) -> AzGlContextPtr;
        pub(crate) fn az_texture_delete(_:  &mut AzTexture);
        pub(crate) fn az_texture_flags_default() -> AzTextureFlags;
        pub(crate) fn az_image_id_new() -> AzImageId;
        pub(crate) fn az_font_id_new() -> AzFontId;
        pub(crate) fn az_raw_image_new(_:  AzU8Vec, _:  usize, _:  usize, _:  AzRawImageFormat) -> AzRawImage;
        pub(crate) fn az_svg_parse_options_default() -> AzSvgParseOptions;
        pub(crate) fn az_svg_render_options_default() -> AzSvgRenderOptions;
        pub(crate) fn az_svg_parse(_:  AzU8VecRef, _:  AzSvgParseOptions) -> AzResultSvgSvgParseError;
        pub(crate) fn az_svg_delete(_:  &mut AzSvg);
        pub(crate) fn az_svg_deep_copy(_:  &AzSvg) -> AzSvg;
        pub(crate) fn az_svg_xml_node_delete(_:  &mut AzSvgXmlNode);
        pub(crate) fn az_svg_xml_node_deep_copy(_:  &AzSvgXmlNode) -> AzSvgXmlNode;
        pub(crate) fn az_timer_id_unique() -> AzTimerId;
        pub(crate) fn az_timer_new(_:  AzRefAny, _:  AzTimerCallbackType, _:  AzGetSystemTimeFn) -> AzTimer;
        pub(crate) fn az_timer_with_delay(_:  AzTimer, _:  AzDuration) -> AzTimer;
        pub(crate) fn az_timer_with_interval(_:  AzTimer, _:  AzDuration) -> AzTimer;
        pub(crate) fn az_timer_with_timeout(_:  AzTimer, _:  AzDuration) -> AzTimer;
        pub(crate) fn az_thread_sender_send(_:  &mut AzThreadSender, _:  AzThreadReceiveMsg) -> bool;
        pub(crate) fn az_thread_sender_delete(_:  &mut AzThreadSender);
        pub(crate) fn az_thread_receiver_receive(_:  &mut AzThreadReceiver) -> AzOptionThreadSendMsg;
        pub(crate) fn az_thread_receiver_delete(_:  &mut AzThreadReceiver);
        pub(crate) fn az_window_state_new(_:  AzLayoutCallbackType) -> AzWindowState;
        pub(crate) fn az_window_state_default() -> AzWindowState;
        pub(crate) fn az_window_create_options_new(_:  AzLayoutCallbackType) -> AzWindowCreateOptions;
        pub(crate) fn az_window_create_options_default() -> AzWindowCreateOptions;
    }

    #[cfg(not(feature = "link_static"))] {
        extern crate azul_dll;
        use azul_dll::*;
    }

}

pub mod str {
    #![allow(dead_code, unused_imports)]
    //! Definition of azuls internal `String` wrappers
    use crate::dll::*;
    use core::ffi::c_void;

    use alloc::string;


    impl From<&'static str> for crate::str::String {
        fn from(v: &'static str) -> crate::str::String {
            crate::str::String::from_const_str(v)
        }
    }

    impl From<string::String> for crate::str::String {
        fn from(s: string::String) -> crate::str::String {
            crate::str::String::from_string(s)
        }
    }

    impl AsRef<str> for crate::str::String {
        fn as_ref(&self) -> &str {
            self.as_str()
        }
    }

    impl core::fmt::Display for crate::str::String {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            self.as_str().fmt(f)
        }
    }

    impl crate::str::String {

        #[inline(always)]
        pub fn from_string(s: string::String) -> crate::str::String {
            crate::str::String {
                vec: crate::vec::U8Vec::from_vec(s.into_bytes())
            }
        }

        #[inline(always)]
        pub const fn from_const_str(s: &'static str) -> crate::str::String {
            crate::str::String {
                vec: crate::vec::U8Vec::from_const_slice(s.as_bytes())
            }
        }
    }    /// `String` struct
    
#[doc(inline)] pub use crate::dll::AzString as String;
}

pub mod vec {
    #![allow(dead_code, unused_imports)]
    //! Definition of azuls internal `Vec<*>` wrappers
    use crate::dll::*;
    use core::ffi::c_void;
    use core::iter;
    use core::fmt;
    use core::cmp;

    use alloc::vec::{self, Vec};
    use alloc::slice;
    use alloc::string;

    use crate::gl::{
        GLint as AzGLint,
        GLuint as AzGLuint,
    };

    macro_rules! impl_vec {($struct_type:ident, $struct_name:ident, $destructor_name:ident, $c_destructor_fn_name:ident, $crate_dll_delete_fn:ident) => (

        unsafe impl Send for $struct_name { }

        impl fmt::Debug for $destructor_name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $destructor_name::DefaultRust => write!(f, "DefaultRust"),
                    $destructor_name::NoDestructor => write!(f, "NoDestructor"),
                    $destructor_name::External(_) => write!(f, "External"),
                }
            }
        }

        impl PartialEq for $destructor_name {
            fn eq(&self, rhs: &Self) -> bool {
                match (self, rhs) {
                    ($destructor_name::DefaultRust, $destructor_name::DefaultRust) => true,
                    ($destructor_name::NoDestructor, $destructor_name::NoDestructor) => true,
                    ($destructor_name::External(a), $destructor_name::External(b)) => (a as *const _ as usize).eq(&(b as *const _ as usize)),
                    _ => false,
                }
            }
        }

        impl PartialOrd for $destructor_name {
            fn partial_cmp(&self, _rhs: &Self) -> Option<cmp::Ordering> {
                None
            }
        }

        impl $struct_name {

            #[inline]
            pub fn iter(&self) -> slice::Iter<$struct_type> {
                self.as_ref().iter()
            }

            #[inline]
            pub fn ptr_as_usize(&self) -> usize {
                self.ptr as usize
            }

            #[inline]
            pub fn len(&self) -> usize {
                self.len
            }

            #[inline]
            pub fn capacity(&self) -> usize {
                self.cap
            }

            #[inline]
            pub fn is_empty(&self) -> bool {
                self.len == 0
            }

            pub fn get(&self, index: usize) -> Option<&$struct_type> {
                let v1: &[$struct_type] = self.as_ref();
                let res = v1.get(index);
                res
            }

            #[inline]
            unsafe fn get_unchecked(&self, index: usize) -> &$struct_type {
                let v1: &[$struct_type] = self.as_ref();
                let res = v1.get_unchecked(index);
                res
            }

            pub fn as_slice(&self) -> &[$struct_type] {
                self.as_ref()
            }

            #[inline(always)]
            pub const fn from_const_slice(input: &'static [$struct_type]) -> Self {
                Self {
                    ptr: input.as_ptr(),
                    len: input.len(),
                    cap: input.len(),
                    destructor: $destructor_name::NoDestructor, // because of &'static
                }
            }

            #[inline(always)]
            pub fn from_vec(input: Vec<$struct_type>) -> Self {

                extern "C" fn $c_destructor_fn_name(s: &mut $struct_name) {
                    let _ = unsafe { Vec::from_raw_parts(s.ptr as *mut $struct_type, s.len, s.cap) };
                }

                let ptr = input.as_ptr();
                let len = input.len();
                let cap = input.capacity();

                let _ = ::core::mem::ManuallyDrop::new(input);

                Self {
                    ptr,
                    len,
                    cap,
                    destructor: $destructor_name::External($c_destructor_fn_name),
                }
            }
        }

        impl AsRef<[$struct_type]> for $struct_name {
            fn as_ref(&self) -> &[$struct_type] {
                unsafe { slice::from_raw_parts(self.ptr, self.len) }
            }
        }

        impl iter::FromIterator<$struct_type> for $struct_name {
            fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = $struct_type> {
                Self::from_vec(Vec::from_iter(iter))
            }
        }

        impl From<Vec<$struct_type>> for $struct_name {
            fn from(input: Vec<$struct_type>) -> $struct_name {
                Self::from_vec(input)
            }
        }

        impl From<&'static [$struct_type]> for $struct_name {
            fn from(input: &'static [$struct_type]) -> $struct_name {
                Self::from_const_slice(input)
            }
        }

        impl Drop for $struct_name {
            fn drop(&mut self) {
                match self.destructor {
                    $destructor_name::DefaultRust => { unsafe { crate::dll::$crate_dll_delete_fn(self); } },
                    $destructor_name::NoDestructor => { },
                    $destructor_name::External(f) => { f(self); }
                }
            }
        }

        impl fmt::Debug for $struct_name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.as_ref().fmt(f)
            }
        }

        impl PartialOrd for $struct_name {
            fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
                self.as_ref().partial_cmp(rhs.as_ref())
            }
        }

        impl PartialEq for $struct_name {
            fn eq(&self, rhs: &Self) -> bool {
                self.as_ref().eq(rhs.as_ref())
            }
        }
    )}

    #[macro_export]
    macro_rules! impl_vec_clone {($struct_type:ident, $struct_name:ident, $destructor_name:ident) => (
        impl $struct_name {
            /// NOTE: CLONES the memory if the memory is external or &'static
            /// Moves the memory out if the memory is library-allocated
            #[inline(always)]
            pub fn clone_self(&self) -> Self {
                match self.destructor {
                    $destructor_name::NoDestructor => {
                        Self {
                            ptr: self.ptr,
                            len: self.len,
                            cap: self.cap,
                            destructor: $destructor_name::NoDestructor,
                        }
                    }
                    $destructor_name::External(_) | $destructor_name::DefaultRust => {
                        Self::from_vec(self.as_ref().to_vec())
                    }
                }
            }
        }

        impl Clone for $struct_name {
            fn clone(&self) -> Self {
                self.clone_self()
            }
        }
    )}

    impl_vec!(u8,  AzU8Vec,  AzU8VecDestructor, az_u8_vec_destructor, az_u8_vec_delete);
    impl_vec_clone!(u8,  AzU8Vec,  AzU8VecDestructor);
    impl_vec!(u32, AzU32Vec, AzU32VecDestructor, az_u32_vec_destructor, az_u32_vec_delete);
    impl_vec_clone!(u32, AzU32Vec, AzU32VecDestructor);
    impl_vec!(u32, AzScanCodeVec, AzScanCodeVecDestructor, az_scan_code_vec_destructor, az_scan_code_vec_delete);
    impl_vec_clone!(u32, AzScanCodeVec, AzScanCodeVecDestructor);
    impl_vec!(u32, AzGLuintVec, AzGLuintVecDestructor, az_g_luint_vec_destructor, az_g_luint_vec_delete);
    impl_vec_clone!(u32, AzGLuintVec, AzGLuintVecDestructor);
    impl_vec!(i32, AzGLintVec, AzGLintVecDestructor, az_g_lint_vec_destructor, az_g_lint_vec_delete);
    impl_vec_clone!(i32, AzGLintVec, AzGLintVecDestructor);
    impl_vec!(AzNodeDataInlineCssProperty, AzNodeDataInlineCssPropertyVec, NodeDataInlineCssPropertyVecDestructor, az_node_data_inline_css_property_vec_destructor, az_node_data_inline_css_property_vec_delete);
    impl_vec_clone!(AzNodeDataInlineCssProperty, AzNodeDataInlineCssPropertyVec, NodeDataInlineCssPropertyVecDestructor);
    impl_vec!(AzIdOrClass, AzIdOrClassVec, IdOrClassVecDestructor, az_id_or_class_vec_destructor, az_id_or_class_vec_delete);
    impl_vec_clone!(AzIdOrClass, AzIdOrClassVec, IdOrClassVecDestructor);
    impl_vec!(AzStyleTransform, AzStyleTransformVec, AzStyleTransformVecDestructor, az_style_transform_vec_destructor, az_style_transform_vec_delete);
    impl_vec_clone!(AzStyleTransform, AzStyleTransformVec, AzStyleTransformVecDestructor);
    impl_vec!(AzCssProperty, AzCssPropertyVec, AzCssPropertyVecDestructor, az_css_property_vec_destructor, az_css_property_vec_delete);
    impl_vec_clone!(AzCssProperty, AzCssPropertyVec, AzCssPropertyVecDestructor);
    impl_vec!(AzSvgMultiPolygon, AzSvgMultiPolygonVec, AzSvgMultiPolygonVecDestructor, az_svg_multi_polygon_vec_destructor, az_svg_multi_polygon_vec_delete);
    impl_vec_clone!(AzSvgMultiPolygon, AzSvgMultiPolygonVec, AzSvgMultiPolygonVecDestructor);
    impl_vec!(AzSvgPath, AzSvgPathVec, AzSvgPathVecDestructor, az_svg_path_vec_destructor, az_svg_path_vec_delete);
    impl_vec_clone!(AzSvgPath, AzSvgPathVec, AzSvgPathVecDestructor);
    impl_vec!(AzVertexAttribute, AzVertexAttributeVec, AzVertexAttributeVecDestructor, az_vertex_attribute_vec_destructor, az_vertex_attribute_vec_delete);
    impl_vec_clone!(AzVertexAttribute, AzVertexAttributeVec, AzVertexAttributeVecDestructor);
    impl_vec!(AzSvgPathElement, AzSvgPathElementVec, AzSvgPathElementVecDestructor, az_svg_path_element_vec_destructor, az_svg_path_element_vec_delete);
    impl_vec_clone!(AzSvgPathElement, AzSvgPathElementVec, AzSvgPathElementVecDestructor);
    impl_vec!(AzSvgVertex, AzSvgVertexVec, AzSvgVertexVecDestructor, az_svg_vertex_vec_destructor, az_svg_vertex_vec_delete);
    impl_vec_clone!(AzSvgVertex, AzSvgVertexVec, AzSvgVertexVecDestructor);
    impl_vec!(AzXWindowType, AzXWindowTypeVec, AzXWindowTypeVecDestructor, az_x_window_type_vec_destructor, az_x_window_type_vec_delete);
    impl_vec_clone!(AzXWindowType, AzXWindowTypeVec, AzXWindowTypeVecDestructor);
    impl_vec!(AzVirtualKeyCode, AzVirtualKeyCodeVec, AzVirtualKeyCodeVecDestructor, az_virtual_key_code_vec_destructor, az_virtual_key_code_vec_delete);
    impl_vec_clone!(AzVirtualKeyCode, AzVirtualKeyCodeVec, AzVirtualKeyCodeVecDestructor);
    impl_vec!(AzCascadeInfo, AzCascadeInfoVec, AzCascadeInfoVecDestructor, az_cascade_info_vec_destructor, az_cascade_info_vec_delete);
    impl_vec_clone!(AzCascadeInfo, AzCascadeInfoVec, AzCascadeInfoVecDestructor);
    impl_vec!(AzCssDeclaration, AzCssDeclarationVec, AzCssDeclarationVecDestructor, az_css_declaration_vec_destructor, az_css_declaration_vec_delete);
    impl_vec_clone!(AzCssDeclaration, AzCssDeclarationVec, AzCssDeclarationVecDestructor);
    impl_vec!(AzCssPathSelector, AzCssPathSelectorVec, AzCssPathSelectorVecDestructor, az_css_path_selector_vec_destructor, az_css_path_selector_vec_delete);
    impl_vec_clone!(AzCssPathSelector, AzCssPathSelectorVec, AzCssPathSelectorVecDestructor);
    impl_vec!(AzStylesheet, AzStylesheetVec, AzStylesheetVecDestructor, az_stylesheet_vec_destructor, az_stylesheet_vec_delete);
    impl_vec_clone!(AzStylesheet, AzStylesheetVec, AzStylesheetVecDestructor);
    impl_vec!(AzCssRuleBlock, AzCssRuleBlockVec, AzCssRuleBlockVecDestructor, az_css_rule_block_vec_destructor, az_css_rule_block_vec_delete);
    impl_vec_clone!(AzCssRuleBlock, AzCssRuleBlockVec, AzCssRuleBlockVecDestructor);
    impl_vec!(AzCallbackData, AzCallbackDataVec, AzCallbackDataVecDestructor, az_callback_data_vec_destructor, az_callback_data_vec_delete);
    // impl_vec_clone!(AzCallbackData, AzCallbackDataVec, AzCallbackDataVecDestructor);
    impl_vec!(AzDebugMessage, AzDebugMessageVec, AzDebugMessageVecDestructor, az_debug_message_vec_destructor, az_debug_message_vec_delete);
    impl_vec_clone!(AzDebugMessage, AzDebugMessageVec, AzDebugMessageVecDestructor);
    impl_vec!(AzDom, AzDomVec, AzDomVecDestructor, az_dom_vec_destructor, az_dom_vec_delete);
    // impl_vec_clone!(AzDom, AzDomVec, AzDomVecDestructor);
    impl_vec!(AzString, AzStringVec, AzStringVecDestructor, az_string_vec_destructor, az_string_vec_delete);
    impl_vec_clone!(AzString, AzStringVec, AzStringVecDestructor);
    impl_vec!(AzStringPair, AzStringPairVec, AzStringPairVecDestructor, az_string_pair_vec_destructor, az_string_pair_vec_delete);
    impl_vec_clone!(AzStringPair, AzStringPairVec, AzStringPairVecDestructor);
    impl_vec!(AzLinearColorStop, AzLinearColorStopVec, AzLinearColorStopVecDestructor, az_linear_color_stop_vec_destructor, az_linear_color_stop_vec_delete);
    impl_vec_clone!(AzLinearColorStop, AzLinearColorStopVec, AzLinearColorStopVecDestructor);
    impl_vec!(AzRadialColorStop, AzRadialColorStopVec, AzRadialColorStopVecDestructor, az_radial_color_stop_vec_destructor, az_radial_color_stop_vec_delete);
    impl_vec_clone!(AzRadialColorStop, AzRadialColorStopVec, AzRadialColorStopVecDestructor);
    impl_vec!(AzNodeId, AzNodeIdVec, AzNodeIdVecDestructor, az_node_id_vec_destructor, az_node_id_vec_delete);
    impl_vec_clone!(AzNodeId, AzNodeIdVec, AzNodeIdVecDestructor);
    impl_vec!(AzNode, AzNodeVec, AzNodeVecDestructor, az_node_vec_destructor, az_node_vec_delete);
    impl_vec_clone!(AzNode, AzNodeVec, AzNodeVecDestructor);
    impl_vec!(AzStyledNode, AzStyledNodeVec, AzStyledNodeVecDestructor, az_styled_node_vec_destructor, az_styled_node_vec_delete);
    impl_vec_clone!(AzStyledNode, AzStyledNodeVec, AzStyledNodeVecDestructor);
    impl_vec!(AzTagIdToNodeIdMapping, AzTagIdsToNodeIdsMappingVec, AzTagIdsToNodeIdsMappingVecDestructor, az_tag_ids_to_node_ids_mapping_vec_destructor, az_tag_ids_to_node_ids_mapping_vec_delete);
    impl_vec_clone!(AzTagIdToNodeIdMapping, AzTagIdsToNodeIdsMappingVec, AzTagIdsToNodeIdsMappingVecDestructor);
    impl_vec!(AzParentWithNodeDepth, AzParentWithNodeDepthVec, AzParentWithNodeDepthVecDestructor, az_parent_with_node_depth_vec_destructor, az_parent_with_node_depth_vec_delete);
    impl_vec_clone!(AzParentWithNodeDepth, AzParentWithNodeDepthVec, AzParentWithNodeDepthVecDestructor);
    impl_vec!(AzNodeData, AzNodeDataVec, AzNodeDataVecDestructor, az_node_data_vec_destructor, az_node_data_vec_delete);
    // impl_vec_clone!(AzNodeData, AzNodeDataVec, AzNodeDataVecDestructor);
    impl_vec!(AzStyleBackgroundRepeat, AzStyleBackgroundRepeatVec, AzStyleBackgroundRepeatVecDestructor, az_style_background_repeat_vec_destructor, az_style_background_repeat_vec_delete);
    impl_vec_clone!(AzStyleBackgroundRepeat, AzStyleBackgroundRepeatVec, AzStyleBackgroundRepeatVecDestructor);
    impl_vec!(AzStyleBackgroundPosition, AzStyleBackgroundPositionVec, AzStyleBackgroundPositionVecDestructor, az_style_background_position_vec_destructor, az_style_background_position_vec_delete);
    impl_vec_clone!(AzStyleBackgroundPosition, AzStyleBackgroundPositionVec, AzStyleBackgroundPositionVecDestructor);
    impl_vec!(AzStyleBackgroundSize, AzStyleBackgroundSizeVec, AzStyleBackgroundSizeVecDestructor, az_style_background_size_vec_destructor, az_style_background_size_vec_delete);
    impl_vec_clone!(AzStyleBackgroundSize, AzStyleBackgroundSizeVec, AzStyleBackgroundSizeVecDestructor);
    impl_vec!(AzStyleBackgroundContent, AzStyleBackgroundContentVec, AzStyleBackgroundContentVecDestructor, az_style_background_content_vec_destructor, az_style_background_content_vec_delete);
    impl_vec_clone!(AzStyleBackgroundContent, AzStyleBackgroundContentVec, AzStyleBackgroundContentVecDestructor);

    impl From<vec::Vec<string::String>> for crate::vec::StringVec {
        fn from(v: vec::Vec<string::String>) -> crate::vec::StringVec {
            let vec: Vec<AzString> = v.into_iter().map(Into::into).collect();
            vec.into()
            // v dropped here
        }
    }    /// `DomVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzDomVecDestructor as DomVecDestructor;
    /// `DomVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzDomVecDestructorType as DomVecDestructorType;
    /// `IdOrClassVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzIdOrClassVecDestructor as IdOrClassVecDestructor;
    /// `IdOrClassVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzIdOrClassVecDestructorType as IdOrClassVecDestructorType;
    /// `NodeDataInlineCssPropertyVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzNodeDataInlineCssPropertyVecDestructor as NodeDataInlineCssPropertyVecDestructor;
    /// `NodeDataInlineCssPropertyVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzNodeDataInlineCssPropertyVecDestructorType as NodeDataInlineCssPropertyVecDestructorType;
    /// `StyleBackgroundContentVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundContentVecDestructor as StyleBackgroundContentVecDestructor;
    /// `StyleBackgroundContentVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundContentVecDestructorType as StyleBackgroundContentVecDestructorType;
    /// `StyleBackgroundPositionVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundPositionVecDestructor as StyleBackgroundPositionVecDestructor;
    /// `StyleBackgroundPositionVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundPositionVecDestructorType as StyleBackgroundPositionVecDestructorType;
    /// `StyleBackgroundRepeatVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundRepeatVecDestructor as StyleBackgroundRepeatVecDestructor;
    /// `StyleBackgroundRepeatVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundRepeatVecDestructorType as StyleBackgroundRepeatVecDestructorType;
    /// `StyleBackgroundSizeVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundSizeVecDestructor as StyleBackgroundSizeVecDestructor;
    /// `StyleBackgroundSizeVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundSizeVecDestructorType as StyleBackgroundSizeVecDestructorType;
    /// `StyleTransformVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTransformVecDestructor as StyleTransformVecDestructor;
    /// `StyleTransformVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTransformVecDestructorType as StyleTransformVecDestructorType;
    /// `CssPropertyVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzCssPropertyVecDestructor as CssPropertyVecDestructor;
    /// `CssPropertyVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzCssPropertyVecDestructorType as CssPropertyVecDestructorType;
    /// `SvgMultiPolygonVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzSvgMultiPolygonVecDestructor as SvgMultiPolygonVecDestructor;
    /// `SvgMultiPolygonVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzSvgMultiPolygonVecDestructorType as SvgMultiPolygonVecDestructorType;
    /// `SvgPathVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzSvgPathVecDestructor as SvgPathVecDestructor;
    /// `SvgPathVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzSvgPathVecDestructorType as SvgPathVecDestructorType;
    /// `VertexAttributeVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzVertexAttributeVecDestructor as VertexAttributeVecDestructor;
    /// `VertexAttributeVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzVertexAttributeVecDestructorType as VertexAttributeVecDestructorType;
    /// `SvgPathElementVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzSvgPathElementVecDestructor as SvgPathElementVecDestructor;
    /// `SvgPathElementVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzSvgPathElementVecDestructorType as SvgPathElementVecDestructorType;
    /// `SvgVertexVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzSvgVertexVecDestructor as SvgVertexVecDestructor;
    /// `SvgVertexVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzSvgVertexVecDestructorType as SvgVertexVecDestructorType;
    /// `U32VecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzU32VecDestructor as U32VecDestructor;
    /// `U32VecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzU32VecDestructorType as U32VecDestructorType;
    /// `XWindowTypeVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzXWindowTypeVecDestructor as XWindowTypeVecDestructor;
    /// `XWindowTypeVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzXWindowTypeVecDestructorType as XWindowTypeVecDestructorType;
    /// `VirtualKeyCodeVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzVirtualKeyCodeVecDestructor as VirtualKeyCodeVecDestructor;
    /// `VirtualKeyCodeVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzVirtualKeyCodeVecDestructorType as VirtualKeyCodeVecDestructorType;
    /// `CascadeInfoVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzCascadeInfoVecDestructor as CascadeInfoVecDestructor;
    /// `CascadeInfoVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzCascadeInfoVecDestructorType as CascadeInfoVecDestructorType;
    /// `ScanCodeVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzScanCodeVecDestructor as ScanCodeVecDestructor;
    /// `ScanCodeVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzScanCodeVecDestructorType as ScanCodeVecDestructorType;
    /// `CssDeclarationVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzCssDeclarationVecDestructor as CssDeclarationVecDestructor;
    /// `CssDeclarationVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzCssDeclarationVecDestructorType as CssDeclarationVecDestructorType;
    /// `CssPathSelectorVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzCssPathSelectorVecDestructor as CssPathSelectorVecDestructor;
    /// `CssPathSelectorVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzCssPathSelectorVecDestructorType as CssPathSelectorVecDestructorType;
    /// `StylesheetVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzStylesheetVecDestructor as StylesheetVecDestructor;
    /// `StylesheetVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzStylesheetVecDestructorType as StylesheetVecDestructorType;
    /// `CssRuleBlockVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzCssRuleBlockVecDestructor as CssRuleBlockVecDestructor;
    /// `CssRuleBlockVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzCssRuleBlockVecDestructorType as CssRuleBlockVecDestructorType;
    /// `U8VecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzU8VecDestructor as U8VecDestructor;
    /// `U8VecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzU8VecDestructorType as U8VecDestructorType;
    /// `CallbackDataVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzCallbackDataVecDestructor as CallbackDataVecDestructor;
    /// `CallbackDataVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzCallbackDataVecDestructorType as CallbackDataVecDestructorType;
    /// `DebugMessageVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzDebugMessageVecDestructor as DebugMessageVecDestructor;
    /// `DebugMessageVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzDebugMessageVecDestructorType as DebugMessageVecDestructorType;
    /// `GLuintVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzGLuintVecDestructor as GLuintVecDestructor;
    /// `GLuintVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzGLuintVecDestructorType as GLuintVecDestructorType;
    /// `GLintVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzGLintVecDestructor as GLintVecDestructor;
    /// `GLintVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzGLintVecDestructorType as GLintVecDestructorType;
    /// `StringVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzStringVecDestructor as StringVecDestructor;
    /// `StringVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzStringVecDestructorType as StringVecDestructorType;
    /// `StringPairVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzStringPairVecDestructor as StringPairVecDestructor;
    /// `StringPairVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzStringPairVecDestructorType as StringPairVecDestructorType;
    /// `LinearColorStopVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzLinearColorStopVecDestructor as LinearColorStopVecDestructor;
    /// `LinearColorStopVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzLinearColorStopVecDestructorType as LinearColorStopVecDestructorType;
    /// `RadialColorStopVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzRadialColorStopVecDestructor as RadialColorStopVecDestructor;
    /// `RadialColorStopVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzRadialColorStopVecDestructorType as RadialColorStopVecDestructorType;
    /// `NodeIdVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzNodeIdVecDestructor as NodeIdVecDestructor;
    /// `NodeIdVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzNodeIdVecDestructorType as NodeIdVecDestructorType;
    /// `NodeVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzNodeVecDestructor as NodeVecDestructor;
    /// `NodeVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzNodeVecDestructorType as NodeVecDestructorType;
    /// `StyledNodeVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzStyledNodeVecDestructor as StyledNodeVecDestructor;
    /// `StyledNodeVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzStyledNodeVecDestructorType as StyledNodeVecDestructorType;
    /// `TagIdsToNodeIdsMappingVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzTagIdsToNodeIdsMappingVecDestructor as TagIdsToNodeIdsMappingVecDestructor;
    /// `TagIdsToNodeIdsMappingVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzTagIdsToNodeIdsMappingVecDestructorType as TagIdsToNodeIdsMappingVecDestructorType;
    /// `ParentWithNodeDepthVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzParentWithNodeDepthVecDestructor as ParentWithNodeDepthVecDestructor;
    /// `ParentWithNodeDepthVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzParentWithNodeDepthVecDestructorType as ParentWithNodeDepthVecDestructorType;
    /// `NodeDataVecDestructor` struct
    
#[doc(inline)] pub use crate::dll::AzNodeDataVecDestructor as NodeDataVecDestructor;
    /// `NodeDataVecDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzNodeDataVecDestructorType as NodeDataVecDestructorType;
    /// Wrapper over a Rust-allocated `Vec<Dom>`
    
#[doc(inline)] pub use crate::dll::AzDomVec as DomVec;
    /// Wrapper over a Rust-allocated `Vec<IdOrClass>`
    
#[doc(inline)] pub use crate::dll::AzIdOrClassVec as IdOrClassVec;
    /// Wrapper over a Rust-allocated `Vec<NodeDataInlineCssProperty>`
    
#[doc(inline)] pub use crate::dll::AzNodeDataInlineCssPropertyVec as NodeDataInlineCssPropertyVec;
    /// Wrapper over a Rust-allocated `Vec<StyleBackgroundContent>`
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundContentVec as StyleBackgroundContentVec;
    /// Wrapper over a Rust-allocated `Vec<StyleBackgroundPosition>`
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundPositionVec as StyleBackgroundPositionVec;
    /// Wrapper over a Rust-allocated `Vec<StyleBackgroundRepeat>`
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundRepeatVec as StyleBackgroundRepeatVec;
    /// Wrapper over a Rust-allocated `Vec<StyleBackgroundSize>`
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundSizeVec as StyleBackgroundSizeVec;
    /// Wrapper over a Rust-allocated `Vec<StyleTransform>`
    
#[doc(inline)] pub use crate::dll::AzStyleTransformVec as StyleTransformVec;
    /// Wrapper over a Rust-allocated `Vec<CssProperty>`
    
#[doc(inline)] pub use crate::dll::AzCssPropertyVec as CssPropertyVec;
    /// Wrapper over a Rust-allocated `Vec<SvgMultiPolygon>`
    
#[doc(inline)] pub use crate::dll::AzSvgMultiPolygonVec as SvgMultiPolygonVec;
    /// Wrapper over a Rust-allocated `Vec<SvgPath>`
    
#[doc(inline)] pub use crate::dll::AzSvgPathVec as SvgPathVec;
    /// Wrapper over a Rust-allocated `Vec<VertexAttribute>`
    
#[doc(inline)] pub use crate::dll::AzVertexAttributeVec as VertexAttributeVec;
    /// Wrapper over a Rust-allocated `VertexAttribute`
    
#[doc(inline)] pub use crate::dll::AzSvgPathElementVec as SvgPathElementVec;
    /// Wrapper over a Rust-allocated `SvgVertex`
    
#[doc(inline)] pub use crate::dll::AzSvgVertexVec as SvgVertexVec;
    /// Wrapper over a Rust-allocated `Vec<u32>`
    
#[doc(inline)] pub use crate::dll::AzU32Vec as U32Vec;
    /// Wrapper over a Rust-allocated `XWindowType`
    
#[doc(inline)] pub use crate::dll::AzXWindowTypeVec as XWindowTypeVec;
    /// Wrapper over a Rust-allocated `VirtualKeyCode`
    
#[doc(inline)] pub use crate::dll::AzVirtualKeyCodeVec as VirtualKeyCodeVec;
    /// Wrapper over a Rust-allocated `CascadeInfo`
    
#[doc(inline)] pub use crate::dll::AzCascadeInfoVec as CascadeInfoVec;
    /// Wrapper over a Rust-allocated `ScanCode`
    
#[doc(inline)] pub use crate::dll::AzScanCodeVec as ScanCodeVec;
    /// Wrapper over a Rust-allocated `CssDeclaration`
    
#[doc(inline)] pub use crate::dll::AzCssDeclarationVec as CssDeclarationVec;
    /// Wrapper over a Rust-allocated `CssPathSelector`
    
#[doc(inline)] pub use crate::dll::AzCssPathSelectorVec as CssPathSelectorVec;
    /// Wrapper over a Rust-allocated `Stylesheet`
    
#[doc(inline)] pub use crate::dll::AzStylesheetVec as StylesheetVec;
    /// Wrapper over a Rust-allocated `CssRuleBlock`
    
#[doc(inline)] pub use crate::dll::AzCssRuleBlockVec as CssRuleBlockVec;
    /// Wrapper over a Rust-allocated `U8Vec`
    
#[doc(inline)] pub use crate::dll::AzU8Vec as U8Vec;
    /// Wrapper over a Rust-allocated `CallbackData`
    
#[doc(inline)] pub use crate::dll::AzCallbackDataVec as CallbackDataVec;
    /// Wrapper over a Rust-allocated `Vec<DebugMessage>`
    
#[doc(inline)] pub use crate::dll::AzDebugMessageVec as DebugMessageVec;
    /// Wrapper over a Rust-allocated `U32Vec`
    
#[doc(inline)] pub use crate::dll::AzGLuintVec as GLuintVec;
    /// Wrapper over a Rust-allocated `GLintVec`
    
#[doc(inline)] pub use crate::dll::AzGLintVec as GLintVec;
    /// Wrapper over a Rust-allocated `StringVec`
    
#[doc(inline)] pub use crate::dll::AzStringVec as StringVec;
    /// Wrapper over a Rust-allocated `StringPairVec`
    
#[doc(inline)] pub use crate::dll::AzStringPairVec as StringPairVec;
    /// Wrapper over a Rust-allocated `LinearColorStopVec`
    
#[doc(inline)] pub use crate::dll::AzLinearColorStopVec as LinearColorStopVec;
    /// Wrapper over a Rust-allocated `RadialColorStopVec`
    
#[doc(inline)] pub use crate::dll::AzRadialColorStopVec as RadialColorStopVec;
    /// Wrapper over a Rust-allocated `NodeIdVec`
    
#[doc(inline)] pub use crate::dll::AzNodeIdVec as NodeIdVec;
    /// Wrapper over a Rust-allocated `NodeVec`
    
#[doc(inline)] pub use crate::dll::AzNodeVec as NodeVec;
    /// Wrapper over a Rust-allocated `StyledNodeVec`
    
#[doc(inline)] pub use crate::dll::AzStyledNodeVec as StyledNodeVec;
    /// Wrapper over a Rust-allocated `TagIdsToNodeIdsMappingVec`
    
#[doc(inline)] pub use crate::dll::AzTagIdsToNodeIdsMappingVec as TagIdsToNodeIdsMappingVec;
    /// Wrapper over a Rust-allocated `ParentWithNodeDepthVec`
    
#[doc(inline)] pub use crate::dll::AzParentWithNodeDepthVec as ParentWithNodeDepthVec;
    /// Wrapper over a Rust-allocated `NodeDataVec`
    
#[doc(inline)] pub use crate::dll::AzNodeDataVec as NodeDataVec;
}

pub mod option {
    #![allow(dead_code, unused_imports)]
    //! Definition of azuls internal `Option<*>` wrappers
    use crate::dll::*;
    use core::ffi::c_void;
    use crate::dll::*;

    macro_rules! impl_option_inner {
        ($struct_type:ident, $struct_name:ident) => (

        impl Default for $struct_name {
            fn default() -> $struct_name { $struct_name::None }
        }

        impl $struct_name {
            pub fn as_option(&self) -> Option<&$struct_type> {
                match self {
                    $struct_name::None => None,
                    $struct_name::Some(t) => Some(t),
                }
            }
            pub fn replace(&mut self, value: $struct_type) -> $struct_name {
                ::core::mem::replace(self, $struct_name::Some(value))
            }
            pub const fn is_some(&self) -> bool {
                match self {
                    $struct_name::None => false,
                    $struct_name::Some(_) => true,
                }
            }
            pub const fn is_none(&self) -> bool {
                !self.is_some()
            }
            pub const fn as_ref(&self) -> Option<&$struct_type> {
                match *self {
                    $struct_name::Some(ref x) => Some(x),
                    $struct_name::None => None,
                }
            }
        }
    )}

    macro_rules! impl_option {
        ($struct_type:ident, $struct_name:ident, copy = false, clone = false, [$($derive:meta),* ]) => (
            impl_option_inner!($struct_type, $struct_name);
        );
        ($struct_type:ident, $struct_name:ident, copy = false, [$($derive:meta),* ]) => (
            impl_option_inner!($struct_type, $struct_name);

            impl From<$struct_name> for Option<$struct_type> {
                fn from(o: $struct_name) -> Option<$struct_type> {
                    match &o {
                        $struct_name::None => None,
                        $struct_name::Some(t) => Some(t.clone()),
                    }
                }
            }

            impl From<Option<$struct_type>> for $struct_name {
                fn from(o: Option<$struct_type>) -> $struct_name {
                    match &o {
                        None => $struct_name::None,
                        Some(t) => $struct_name::Some(t.clone()),
                    }
                }
            }

            impl $struct_name {
                pub fn into_option(self) -> Option<$struct_type> {
                    self.into()
                }
                pub fn map<U, F: FnOnce($struct_type) -> U>(self, f: F) -> Option<U> {
                    match self.into_option() {
                        None => None,
                        Some(s) => Some(f(s)),
                    }
                }

                pub fn and_then<U, F>(self, f: F) -> Option<U> where F: FnOnce($struct_type) -> Option<U> {
                    match self.into_option() {
                        None => None,
                        Some(s) => f(s),
                    }
                }
            }
        );
        ($struct_type:ident, $struct_name:ident, [$($derive:meta),* ]) => (
            impl_option_inner!($struct_type, $struct_name);

            impl From<$struct_name> for Option<$struct_type> {
                fn from(o: $struct_name) -> Option<$struct_type> {
                    match o {
                        $struct_name::None => None,
                        $struct_name::Some(t) => Some(t),
                    }
                }
            }

            impl From<Option<$struct_type>> for $struct_name {
                fn from(o: Option<$struct_type>) -> $struct_name {
                    match o {
                        None => $struct_name::None,
                        Some(t) => $struct_name::Some(t),
                    }
                }
            }

            impl $struct_name {
                pub fn into_option(self) -> Option<$struct_type> {
                    self.into()
                }
                pub fn map<U, F: FnOnce($struct_type) -> U>(self, f: F) -> Option<U> {
                    match self.into_option() {
                        None => None,
                        Some(s) => Some(f(s)),
                    }
                }

                pub fn and_then<U, F>(self, f: F) -> Option<U> where F: FnOnce($struct_type) -> Option<U> {
                    match self.into_option() {
                        None => None,
                        Some(s) => f(s),
                    }
                }
            }
        );
    }

    pub type AzX11Visual = *const c_void;
    pub type AzHwndHandle = *mut c_void;

    impl_option!(i32, AzOptionI32, [Debug, Copy, Clone]);
    impl_option!(f32, AzOptionF32, [Debug, Copy, Clone]);
    impl_option!(usize, AzOptionUsize, [Debug, Copy, Clone]);
    impl_option!(u32, AzOptionChar, [Debug, Copy, Clone]);

    impl_option!(AzThreadSendMsg, AzOptionThreadSendMsg, [Debug, Copy, Clone]);
    impl_option!(AzLayoutRect, AzOptionLayoutRect, [Debug, Copy, Clone]);
    impl_option!(AzRefAny, AzOptionRefAny, copy = false, clone = false, [Debug, Clone]);
    impl_option!(AzLayoutPoint, AzOptionLayoutPoint, [Debug, Copy, Clone]);
    impl_option!(AzWindowTheme, AzOptionWindowTheme, [Debug, Copy, Clone]);
    impl_option!(AzNodeId, AzOptionNodeId, [Debug, Copy, Clone]);
    impl_option!(AzDomNodeId, AzOptionDomNodeId, [Debug, Copy, Clone]);
    impl_option!(AzColorU, AzOptionColorU, [Debug, Copy, Clone]);
    impl_option!(AzRawImage, AzOptionRawImage, copy = false, [Debug, Clone]);
    impl_option!(AzSvgDashPattern, AzOptionSvgDashPattern, [Debug, Copy, Clone]);
    impl_option!(AzWaylandTheme, AzOptionWaylandTheme, copy = false, [Debug, Clone]);
    impl_option!(AzTaskBarIcon, AzOptionTaskBarIcon, copy = false, [Debug, Clone]);
    impl_option!(AzLogicalPosition, AzOptionLogicalPosition, [Debug, Copy, Clone]);
    impl_option!(AzPhysicalPositionI32, AzOptionPhysicalPositionI32, [Debug, Copy, Clone]);
    impl_option!(AzWindowIcon, AzOptionWindowIcon, copy = false, [Debug, Clone]);
    impl_option!(AzString, AzOptionString, copy = false, [Debug, Clone]);
    impl_option!(AzMouseCursorType, AzOptionMouseCursorType, [Debug, Copy, Clone]);
    impl_option!(AzLogicalSize, AzOptionLogicalSize, [Debug, Copy, Clone]);
    impl_option!(AzVirtualKeyCode, AzOptionVirtualKeyCode, [Debug, Copy, Clone]);
    impl_option!(AzPercentageValue, AzOptionPercentageValue, [Debug, Copy, Clone]);
    impl_option!(AzDom, AzOptionDom, copy = false, clone = false, [Debug, Clone]);
    impl_option!(AzTexture, AzOptionTexture, copy = false, clone = false, [Debug]);
    impl_option!(AzImageMask, AzOptionImageMask, copy = false, [Debug, Clone]);
    impl_option!(AzTabIndex, AzOptionTabIndex, [Debug, Copy, Clone]);
    impl_option!(AzCallback, AzOptionCallback, [Debug, Copy, Clone]);
    impl_option!(AzTagId, AzOptionTagId, [Debug, Copy, Clone]);
    impl_option!(AzDuration, AzOptionDuration, [Debug, Copy, Clone]);
    impl_option!(AzInstant, AzOptionInstant, copy = false, clone = false, [Debug]); // TODO: impl clone!
    impl_option!(AzU8VecRef, AzOptionU8VecRef, copy = false, clone = false, [Debug]);
    /// `OptionGlContextPtr` struct
    
#[doc(inline)] pub use crate::dll::AzOptionGlContextPtr as OptionGlContextPtr;
    /// `OptionThreadReceiveMsg` struct
    
#[doc(inline)] pub use crate::dll::AzOptionThreadReceiveMsg as OptionThreadReceiveMsg;
    /// `OptionPercentageValue` struct
    
#[doc(inline)] pub use crate::dll::AzOptionPercentageValue as OptionPercentageValue;
    /// `OptionAngleValue` struct
    
#[doc(inline)] pub use crate::dll::AzOptionAngleValue as OptionAngleValue;
    /// `OptionRendererOptions` struct
    
#[doc(inline)] pub use crate::dll::AzOptionRendererOptions as OptionRendererOptions;
    /// `OptionCallback` struct
    
#[doc(inline)] pub use crate::dll::AzOptionCallback as OptionCallback;
    /// `OptionThreadSendMsg` struct
    
#[doc(inline)] pub use crate::dll::AzOptionThreadSendMsg as OptionThreadSendMsg;
    /// `OptionLayoutRect` struct
    
#[doc(inline)] pub use crate::dll::AzOptionLayoutRect as OptionLayoutRect;
    /// `OptionRefAny` struct
    
#[doc(inline)] pub use crate::dll::AzOptionRefAny as OptionRefAny;
    /// `OptionLayoutPoint` struct
    
#[doc(inline)] pub use crate::dll::AzOptionLayoutPoint as OptionLayoutPoint;
    /// `OptionWindowTheme` struct
    
#[doc(inline)] pub use crate::dll::AzOptionWindowTheme as OptionWindowTheme;
    /// `OptionNodeId` struct
    
#[doc(inline)] pub use crate::dll::AzOptionNodeId as OptionNodeId;
    /// `OptionDomNodeId` struct
    
#[doc(inline)] pub use crate::dll::AzOptionDomNodeId as OptionDomNodeId;
    /// `OptionColorU` struct
    
#[doc(inline)] pub use crate::dll::AzOptionColorU as OptionColorU;
    /// `OptionRawImage` struct
    
#[doc(inline)] pub use crate::dll::AzOptionRawImage as OptionRawImage;
    /// `OptionSvgDashPattern` struct
    
#[doc(inline)] pub use crate::dll::AzOptionSvgDashPattern as OptionSvgDashPattern;
    /// `OptionWaylandTheme` struct
    
#[doc(inline)] pub use crate::dll::AzOptionWaylandTheme as OptionWaylandTheme;
    /// `OptionTaskBarIcon` struct
    
#[doc(inline)] pub use crate::dll::AzOptionTaskBarIcon as OptionTaskBarIcon;
    /// `OptionHwndHandle` struct
    
#[doc(inline)] pub use crate::dll::AzOptionHwndHandle as OptionHwndHandle;
    /// `OptionLogicalPosition` struct
    
#[doc(inline)] pub use crate::dll::AzOptionLogicalPosition as OptionLogicalPosition;
    /// `OptionPhysicalPositionI32` struct
    
#[doc(inline)] pub use crate::dll::AzOptionPhysicalPositionI32 as OptionPhysicalPositionI32;
    /// `OptionWindowIcon` struct
    
#[doc(inline)] pub use crate::dll::AzOptionWindowIcon as OptionWindowIcon;
    /// `OptionString` struct
    
#[doc(inline)] pub use crate::dll::AzOptionString as OptionString;
    /// `OptionX11Visual` struct
    
#[doc(inline)] pub use crate::dll::AzOptionX11Visual as OptionX11Visual;
    /// `OptionI32` struct
    
#[doc(inline)] pub use crate::dll::AzOptionI32 as OptionI32;
    /// `OptionF32` struct
    
#[doc(inline)] pub use crate::dll::AzOptionF32 as OptionF32;
    /// `OptionMouseCursorType` struct
    
#[doc(inline)] pub use crate::dll::AzOptionMouseCursorType as OptionMouseCursorType;
    /// `OptionLogicalSize` struct
    
#[doc(inline)] pub use crate::dll::AzOptionLogicalSize as OptionLogicalSize;
    /// Option<char> but the char is a u32, for C FFI stability reasons
    
#[doc(inline)] pub use crate::dll::AzOptionChar as OptionChar;
    /// `OptionVirtualKeyCode` struct
    
#[doc(inline)] pub use crate::dll::AzOptionVirtualKeyCode as OptionVirtualKeyCode;
    /// `OptionDom` struct
    
#[doc(inline)] pub use crate::dll::AzOptionDom as OptionDom;
    /// `OptionTexture` struct
    
#[doc(inline)] pub use crate::dll::AzOptionTexture as OptionTexture;
    /// `OptionImageMask` struct
    
#[doc(inline)] pub use crate::dll::AzOptionImageMask as OptionImageMask;
    /// `OptionTabIndex` struct
    
#[doc(inline)] pub use crate::dll::AzOptionTabIndex as OptionTabIndex;
    /// `OptionTagId` struct
    
#[doc(inline)] pub use crate::dll::AzOptionTagId as OptionTagId;
    /// `OptionDuration` struct
    
#[doc(inline)] pub use crate::dll::AzOptionDuration as OptionDuration;
    /// `OptionInstant` struct
    
#[doc(inline)] pub use crate::dll::AzOptionInstant as OptionInstant;
    /// `OptionUsize` struct
    
#[doc(inline)] pub use crate::dll::AzOptionUsize as OptionUsize;
    /// `OptionU8VecRef` struct
    
#[doc(inline)] pub use crate::dll::AzOptionU8VecRef as OptionU8VecRef;
}

pub mod error {
    #![allow(dead_code, unused_imports)]
    //! Definition of error and `Result<T, E>`  types
    use crate::dll::*;
    use core::ffi::c_void;
    /// `ResultSvgSvgParseError` struct
    
#[doc(inline)] pub use crate::dll::AzResultSvgSvgParseError as ResultSvgSvgParseError;
    /// `SvgParseError` struct
    
#[doc(inline)] pub use crate::dll::AzSvgParseError as SvgParseError;
    /// `XmlError` struct
    
#[doc(inline)] pub use crate::dll::AzXmlError as XmlError;
    /// `DuplicatedNamespaceError` struct
    
#[doc(inline)] pub use crate::dll::AzDuplicatedNamespaceError as DuplicatedNamespaceError;
    /// `UnknownNamespaceError` struct
    
#[doc(inline)] pub use crate::dll::AzUnknownNamespaceError as UnknownNamespaceError;
    /// `UnexpectedCloseTagError` struct
    
#[doc(inline)] pub use crate::dll::AzUnexpectedCloseTagError as UnexpectedCloseTagError;
    /// `UnknownEntityReferenceError` struct
    
#[doc(inline)] pub use crate::dll::AzUnknownEntityReferenceError as UnknownEntityReferenceError;
    /// `DuplicatedAttributeError` struct
    
#[doc(inline)] pub use crate::dll::AzDuplicatedAttributeError as DuplicatedAttributeError;
    /// `XmlParseError` struct
    
#[doc(inline)] pub use crate::dll::AzXmlParseError as XmlParseError;
    /// `XmlTextError` struct
    
#[doc(inline)] pub use crate::dll::AzXmlTextError as XmlTextError;
    /// `XmlStreamError` struct
    
#[doc(inline)] pub use crate::dll::AzXmlStreamError as XmlStreamError;
    /// `NonXmlCharError` struct
    
#[doc(inline)] pub use crate::dll::AzNonXmlCharError as NonXmlCharError;
    /// `InvalidCharError` struct
    
#[doc(inline)] pub use crate::dll::AzInvalidCharError as InvalidCharError;
    /// `InvalidCharMultipleError` struct
    
#[doc(inline)] pub use crate::dll::AzInvalidCharMultipleError as InvalidCharMultipleError;
    /// `InvalidQuoteError` struct
    
#[doc(inline)] pub use crate::dll::AzInvalidQuoteError as InvalidQuoteError;
    /// `InvalidSpaceError` struct
    
#[doc(inline)] pub use crate::dll::AzInvalidSpaceError as InvalidSpaceError;
    /// `InvalidStringError` struct
    
#[doc(inline)] pub use crate::dll::AzInvalidStringError as InvalidStringError;
    /// `SvgParseErrorPosition` struct
    
#[doc(inline)] pub use crate::dll::AzSvgParseErrorPosition as SvgParseErrorPosition;
}

pub mod time {
    #![allow(dead_code, unused_imports)]
    //! Rust wrappers for `Instant` / `Duration` classes
    use crate::dll::*;
    use core::ffi::c_void;
    /// `Instant` struct
    
#[doc(inline)] pub use crate::dll::AzInstant as Instant;
    /// `InstantPtr` struct
    
#[doc(inline)] pub use crate::dll::AzInstantPtr as InstantPtr;
    impl Clone for InstantPtr { fn clone(&self) -> Self { unsafe { crate::dll::az_instant_ptr_deep_copy(self) } } }
    impl Drop for InstantPtr { fn drop(&mut self) { unsafe { crate::dll::az_instant_ptr_delete(self) } } }
    /// `InstantPtrCloneFnType` struct
    
#[doc(inline)] pub use crate::dll::AzInstantPtrCloneFnType as InstantPtrCloneFnType;
    /// `InstantPtrCloneFn` struct
    
#[doc(inline)] pub use crate::dll::AzInstantPtrCloneFn as InstantPtrCloneFn;
    /// `InstantPtrDestructorFnType` struct
    
#[doc(inline)] pub use crate::dll::AzInstantPtrDestructorFnType as InstantPtrDestructorFnType;
    /// `InstantPtrDestructorFn` struct
    
#[doc(inline)] pub use crate::dll::AzInstantPtrDestructorFn as InstantPtrDestructorFn;
    /// `SystemTick` struct
    
#[doc(inline)] pub use crate::dll::AzSystemTick as SystemTick;
    /// `Duration` struct
    
#[doc(inline)] pub use crate::dll::AzDuration as Duration;
    /// `SystemTimeDiff` struct
    
#[doc(inline)] pub use crate::dll::AzSystemTimeDiff as SystemTimeDiff;
    /// `SystemTickDiff` struct
    
#[doc(inline)] pub use crate::dll::AzSystemTickDiff as SystemTickDiff;
}

pub mod app {
    #![allow(dead_code, unused_imports)]
    //! `App` construction and configuration
    use crate::dll::*;
    use core::ffi::c_void;
    use crate::callbacks::RefAny;
    use crate::window::WindowCreateOptions;
    /// `AppLogLevel` struct
    
#[doc(inline)] pub use crate::dll::AzAppLogLevel as AppLogLevel;
    /// External system callbacks to get the system time or create / manage threads
    
#[doc(inline)] pub use crate::dll::AzSystemCallbacks as SystemCallbacks;
    /// Configuration for optional features, such as whether to enable logging or panic hooks
    
#[doc(inline)] pub use crate::dll::AzAppConfig as AppConfig;
    impl AppConfig {
        /// Creates a new AppConfig with default values
        pub fn default() -> Self { unsafe { crate::dll::az_app_config_default() } }
    }

    /// Main application class
    
#[doc(inline)] pub use crate::dll::AzApp as App;
    impl App {
        /// Creates a new App instance from the given `AppConfig`
        pub fn new(data: RefAny, config: AppConfig) -> Self { unsafe { crate::dll::az_app_new(data, config) } }
        /// Spawn a new window on the screen when the app is run.
        pub fn add_window(&mut self, window: WindowCreateOptions)  { unsafe { crate::dll::az_app_add_window(self, window) } }
        /// Runs the application. Due to platform restrictions (specifically `WinMain` on Windows), this function never returns.
        pub fn run(self, window: WindowCreateOptions)  { unsafe { crate::dll::az_app_run(self, window) } }
    }

    impl Drop for App { fn drop(&mut self) { unsafe { crate::dll::az_app_delete(self) } } }
}

pub mod callbacks {
    #![allow(dead_code, unused_imports)]
    //! Callback type definitions + struct definitions of `CallbackInfo`s
    use crate::dll::*;
    use core::ffi::c_void;

    #[derive(Debug)]
    #[repr(C)]
    pub struct Ref<'a, T> {
        ptr: &'a T,
        sharing_info: RefCount,
    }

    impl<'a, T> Drop for Ref<'a, T> {
        fn drop(&mut self) {
            self.sharing_info.decrease_ref();
        }
    }

    impl<'a, T> core::ops::Deref for Ref<'a, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            self.ptr
        }
    }

    #[derive(Debug)]
    #[repr(C)]
    pub struct RefMut<'a, T> {
        ptr: &'a mut T,
        sharing_info: RefCount,
    }

    impl<'a, T> Drop for RefMut<'a, T> {
        fn drop(&mut self) {
            self.sharing_info.decrease_refmut();
        }
    }

    impl<'a, T> core::ops::Deref for RefMut<'a, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &*self.ptr
        }
    }

    impl<'a, T> core::ops::DerefMut for RefMut<'a, T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.ptr
        }
    }

    impl RefAny {

        /// Creates a new, type-erased pointer by casting the `T` value into a `Vec<u8>` and saving the length + type ID
        pub fn new<T: 'static>(value: T) -> Self {
            use crate::dll::*;

            extern "C" fn default_custom_destructor<U: 'static>(ptr: &mut c_void) {
                use core::{mem, ptr};

                // note: in the default constructor, we do not need to check whether U == T

                unsafe {
                    // copy the struct from the heap to the stack and call mem::drop on U to run the destructor
                    let mut stack_mem = mem::zeroed::<U>();
                    ptr::copy_nonoverlapping((ptr as *mut c_void) as *const U, &mut stack_mem as *mut U, mem::size_of::<U>());
                    mem::drop(stack_mem);
                }
            }

            let type_name_str = ::core::any::type_name::<T>();
            let st = crate::str::String::from_const_str(type_name_str);
            let s = unsafe { crate::dll::az_ref_any_new_c(
                (&value as *const T) as *const c_void,
                ::core::mem::size_of::<T>(),
                Self::get_type_id::<T>(),
                st,
                default_custom_destructor::<T>,
            ) };
            ::core::mem::forget(value); // do not run the destructor of T here!
            s
        }

        /// Downcasts the type-erased pointer to a type `&U`, returns `None` if the types don't match
        #[inline]
        pub fn downcast_ref<'a, U: 'static>(&'a mut self) -> Option<Ref<'a, U>> {
            let is_same_type = self.is_type(Self::get_type_id::<U>());
            if !is_same_type { return None; }

            let can_be_shared = self.sharing_info.can_be_shared();
            if !can_be_shared { return None; }

            self.sharing_info.increase_ref();
            Some(Ref {
                ptr: unsafe { &*(self._internal_ptr as *const U) },
                sharing_info: self.sharing_info.clone(),
            })
        }

        /// Downcasts the type-erased pointer to a type `&mut U`, returns `None` if the types don't match
        #[inline]
        pub fn downcast_mut<'a, U: 'static>(&'a mut self) -> Option<RefMut<'a, U>> {
            let is_same_type = self.is_type(Self::get_type_id::<U>());
            if !is_same_type { return None; }

            let can_be_shared_mut = self.sharing_info.can_be_shared_mut();
            if !can_be_shared_mut { return None; }

            self.sharing_info.increase_refmut();

            Some(RefMut {
                ptr: unsafe { &mut *(self._internal_ptr as *mut U) },
                sharing_info: self.sharing_info.clone(),
            })
        }

        // Returns the typeid of `T` as a u64 (necessary because `core::any::TypeId` is not C-ABI compatible)
        #[inline]
        pub fn get_type_id<T: 'static>() -> u64 {
            use core::any::TypeId;
            use core::mem;

            // fast method to serialize the type id into a u64
            let t_id = TypeId::of::<T>();
            let struct_as_bytes = unsafe { ::core::slice::from_raw_parts((&t_id as *const TypeId) as *const u8, mem::size_of::<TypeId>()) };
            struct_as_bytes.into_iter().enumerate().map(|(s_pos, s)| ((*s as u64) << s_pos)).sum()
        }
    }    use crate::window::{WindowCreateOptions, WindowState};
    use crate::css::CssProperty;
    use crate::task::{ThreadId, Timer, TimerId};
    use crate::str::String;
    /// `NodeId` struct
    
#[doc(inline)] pub use crate::dll::AzNodeId as NodeId;
    /// `DomId` struct
    
#[doc(inline)] pub use crate::dll::AzDomId as DomId;
    /// `DomNodeId` struct
    
#[doc(inline)] pub use crate::dll::AzDomNodeId as DomNodeId;
    /// `HidpiAdjustedBounds` struct
    
#[doc(inline)] pub use crate::dll::AzHidpiAdjustedBounds as HidpiAdjustedBounds;
    impl HidpiAdjustedBounds {
        /// Returns the size of the bounds in logical units
        pub fn get_logical_size(&self)  -> crate::window::LogicalSize { unsafe { crate::dll::az_hidpi_adjusted_bounds_get_logical_size(self) } }
        /// Returns the size of the bounds in physical units
        pub fn get_physical_size(&self)  -> crate::window::PhysicalSizeU32 { unsafe { crate::dll::az_hidpi_adjusted_bounds_get_physical_size(self) } }
        /// Returns the hidpi factor of the bounds
        pub fn get_hidpi_factor(&self)  -> f32 { unsafe { crate::dll::az_hidpi_adjusted_bounds_get_hidpi_factor(self) } }
    }

    /// `LayoutCallback` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutCallback as LayoutCallback;
    /// `LayoutCallbackType` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutCallbackType as LayoutCallbackType;
    /// `Callback` struct
    
#[doc(inline)] pub use crate::dll::AzCallback as Callback;
    /// Defines the focus target for the next frame
    
#[doc(inline)] pub use crate::dll::AzFocusTarget as FocusTarget;
    /// `FocusTargetPath` struct
    
#[doc(inline)] pub use crate::dll::AzFocusTargetPath as FocusTargetPath;
    /// `CallbackType` struct
    
#[doc(inline)] pub use crate::dll::AzCallbackType as CallbackType;
    /// `CallbackInfo` struct
    
#[doc(inline)] pub use crate::dll::AzCallbackInfo as CallbackInfo;
    impl CallbackInfo {
        /// Returns the `DomNodeId` of the element that the callback was attached to.
        pub fn get_hit_node(&self)  -> crate::callbacks::DomNodeId { unsafe { crate::dll::az_callback_info_get_hit_node(self) } }
        /// Returns the `LayoutPoint` of the cursor in the viewport (relative to the origin of the `Dom`). Set to `None` if the cursor is not in the current window.
        pub fn get_cursor_relative_to_viewport(&self)  -> crate::option::OptionLayoutPoint { unsafe { crate::dll::az_callback_info_get_cursor_relative_to_viewport(self) } }
        /// Returns the `LayoutPoint` of the cursor in the viewport (relative to the origin of the `Dom`). Set to `None` if the cursor is not hovering over the current node.
        pub fn get_cursor_relative_to_node(&self)  -> crate::option::OptionLayoutPoint { unsafe { crate::dll::az_callback_info_get_cursor_relative_to_node(self) } }
        /// Returns the parent `DomNodeId` of the given `DomNodeId`. Returns `None` on an invalid NodeId.
        pub fn get_parent(&mut self, node_id: DomNodeId)  -> crate::option::OptionDomNodeId { unsafe { crate::dll::az_callback_info_get_parent(self, node_id) } }
        /// Returns the previous siblings `DomNodeId` of the given `DomNodeId`. Returns `None` on an invalid NodeId.
        pub fn get_previous_sibling(&mut self, node_id: DomNodeId)  -> crate::option::OptionDomNodeId { unsafe { crate::dll::az_callback_info_get_previous_sibling(self, node_id) } }
        /// Returns the next siblings `DomNodeId` of the given `DomNodeId`. Returns `None` on an invalid NodeId.
        pub fn get_next_sibling(&mut self, node_id: DomNodeId)  -> crate::option::OptionDomNodeId { unsafe { crate::dll::az_callback_info_get_next_sibling(self, node_id) } }
        /// Returns the next siblings `DomNodeId` of the given `DomNodeId`. Returns `None` on an invalid NodeId.
        pub fn get_first_child(&mut self, node_id: DomNodeId)  -> crate::option::OptionDomNodeId { unsafe { crate::dll::az_callback_info_get_first_child(self, node_id) } }
        /// Returns the next siblings `DomNodeId` of the given `DomNodeId`. Returns `None` on an invalid NodeId.
        pub fn get_last_child(&mut self, node_id: DomNodeId)  -> crate::option::OptionDomNodeId { unsafe { crate::dll::az_callback_info_get_last_child(self, node_id) } }
        /// Returns a copy of the current windows `WindowState`.
        pub fn get_window_state(&self)  -> crate::window::WindowState { unsafe { crate::dll::az_callback_info_get_window_state(self) } }
        /// Returns a copy of the internal `KeyboardState`. Same as `self.get_window_state().keyboard_state`
        pub fn get_keyboard_state(&self)  -> crate::window::KeyboardState { unsafe { crate::dll::az_callback_info_get_keyboard_state(self) } }
        /// Returns a copy of the internal `MouseState`. Same as `self.get_window_state().mouse_state`
        pub fn get_mouse_state(&self)  -> crate::window::MouseState { unsafe { crate::dll::az_callback_info_get_mouse_state(self) } }
        /// Returns a copy of the current windows `RawWindowHandle`.
        pub fn get_current_window_handle(&self)  -> crate::window::RawWindowHandle { unsafe { crate::dll::az_callback_info_get_current_window_handle(self) } }
        /// Returns a **reference-counted copy** of the current windows `GlContextPtr`. You can use this to render OpenGL textures.
        pub fn get_gl_context(&self)  -> crate::option::OptionGlContextPtr { unsafe { crate::dll::az_callback_info_get_gl_context(self) } }
        /// Sets the new `WindowState` for the next frame. The window is updated after all callbacks are run.
        pub fn set_window_state(&mut self, new_state: WindowState)  { unsafe { crate::dll::az_callback_info_set_window_state(self, new_state) } }
        /// Sets the new `FocusTarget` for the next frame. Note that this will emit a `On::FocusLost` and `On::FocusReceived` event, if the focused node has changed.
        pub fn set_focus(&mut self, target: FocusTarget)  { unsafe { crate::dll::az_callback_info_set_focus(self, target) } }
        /// Sets a `CssProperty` on a given ndoe to its new value. If this property change affects the layout, this will automatically trigger a relayout and redraw of the screen.
        pub fn set_css_property(&mut self, node_id: DomNodeId, new_property: CssProperty)  { unsafe { crate::dll::az_callback_info_set_css_property(self, node_id, new_property) } }
        /// Stops the propagation of the current callback event type to the parent. Events are bubbled from the inside out (children first, then parents), this event stops the propagation of the event to the parent.
        pub fn stop_propagation(&mut self)  { unsafe { crate::dll::az_callback_info_stop_propagation(self) } }
        /// Spawns a new window with the given `WindowCreateOptions`.
        pub fn create_window(&mut self, new_window: WindowCreateOptions)  { unsafe { crate::dll::az_callback_info_create_window(self, new_window) } }
        /// Starts a new `Thread` to the runtime. See the documentation for `Thread` for more information.
        pub fn start_thread(&mut self, id: ThreadId, thread_initialize_data: RefAny, writeback_data: RefAny, callback: ThreadCallbackType)  { unsafe { crate::dll::az_callback_info_start_thread(self, id, thread_initialize_data, writeback_data, callback) } }
        /// Adds a new `Timer` to the runtime. See the documentation for `Timer` for more information.
        pub fn start_timer(&mut self, id: TimerId, timer: Timer)  { unsafe { crate::dll::az_callback_info_start_timer(self, id, timer) } }
    }

    /// Specifies if the screen should be updated after the callback function has returned
    
#[doc(inline)] pub use crate::dll::AzUpdateScreen as UpdateScreen;
    /// `IFrameCallback` struct
    
#[doc(inline)] pub use crate::dll::AzIFrameCallback as IFrameCallback;
    /// `IFrameCallbackType` struct
    
#[doc(inline)] pub use crate::dll::AzIFrameCallbackType as IFrameCallbackType;
    /// `IFrameCallbackInfo` struct
    
#[doc(inline)] pub use crate::dll::AzIFrameCallbackInfo as IFrameCallbackInfo;
    /// `IFrameCallbackReturn` struct
    
#[doc(inline)] pub use crate::dll::AzIFrameCallbackReturn as IFrameCallbackReturn;
    /// `GlCallback` struct
    
#[doc(inline)] pub use crate::dll::AzGlCallback as GlCallback;
    /// `GlCallbackType` struct
    
#[doc(inline)] pub use crate::dll::AzGlCallbackType as GlCallbackType;
    /// `GlCallbackInfo` struct
    
#[doc(inline)] pub use crate::dll::AzGlCallbackInfo as GlCallbackInfo;
    impl GlCallbackInfo {
        /// Returns a copy of the internal `GlContextPtr`
        pub fn get_gl_context(&self)  -> crate::option::OptionGlContextPtr { unsafe { crate::dll::az_gl_callback_info_get_gl_context(self) } }
        /// Returns a copy of the internal `HidpiAdjustedBounds`
        pub fn get_bounds(&self)  -> crate::callbacks::HidpiAdjustedBounds { unsafe { crate::dll::az_gl_callback_info_get_bounds(self) } }
    }

    /// `GlCallbackReturn` struct
    
#[doc(inline)] pub use crate::dll::AzGlCallbackReturn as GlCallbackReturn;
    /// `TimerCallback` struct
    
#[doc(inline)] pub use crate::dll::AzTimerCallback as TimerCallback;
    /// `TimerCallbackType` struct
    
#[doc(inline)] pub use crate::dll::AzTimerCallbackType as TimerCallbackType;
    /// `TimerCallbackInfo` struct
    
#[doc(inline)] pub use crate::dll::AzTimerCallbackInfo as TimerCallbackInfo;
    /// `TimerCallbackReturn` struct
    
#[doc(inline)] pub use crate::dll::AzTimerCallbackReturn as TimerCallbackReturn;
    /// `WriteBackCallbackType` struct
    
#[doc(inline)] pub use crate::dll::AzWriteBackCallbackType as WriteBackCallbackType;
    /// `WriteBackCallback` struct
    
#[doc(inline)] pub use crate::dll::AzWriteBackCallback as WriteBackCallback;
    /// `ThreadCallbackType` struct
    
#[doc(inline)] pub use crate::dll::AzThreadCallbackType as ThreadCallbackType;
    /// `RefAnyDestructorType` struct
    
#[doc(inline)] pub use crate::dll::AzRefAnyDestructorType as RefAnyDestructorType;
    /// `RefCountInner` struct
    
#[doc(inline)] pub use crate::dll::AzRefCountInner as RefCountInner;
    /// `RefCount` struct
    
#[doc(inline)] pub use crate::dll::AzRefCount as RefCount;
    impl RefCount {
        /// Calls the `RefCount::can_be_shared` function.
        pub fn can_be_shared(&self)  -> bool { unsafe { crate::dll::az_ref_count_can_be_shared(self) } }
        /// Calls the `RefCount::can_be_shared_mut` function.
        pub fn can_be_shared_mut(&self)  -> bool { unsafe { crate::dll::az_ref_count_can_be_shared_mut(self) } }
        /// Calls the `RefCount::increase_ref` function.
        pub fn increase_ref(&mut self)  { unsafe { crate::dll::az_ref_count_increase_ref(self) } }
        /// Calls the `RefCount::decrease_ref` function.
        pub fn decrease_ref(&mut self)  { unsafe { crate::dll::az_ref_count_decrease_ref(self) } }
        /// Calls the `RefCount::increase_refmut` function.
        pub fn increase_refmut(&mut self)  { unsafe { crate::dll::az_ref_count_increase_refmut(self) } }
        /// Calls the `RefCount::decrease_refmut` function.
        pub fn decrease_refmut(&mut self)  { unsafe { crate::dll::az_ref_count_decrease_refmut(self) } }
    }

    impl Clone for RefCount { fn clone(&self) -> Self { unsafe { crate::dll::az_ref_count_deep_copy(self) } } }
    impl Drop for RefCount { fn drop(&mut self) { unsafe { crate::dll::az_ref_count_delete(self) } } }
    /// RefAny is a reference-counted, type-erased pointer, which stores a reference to a struct. `RefAny` can be up- and downcasted (this usually done via generics and can't be expressed in the Rust API)
    
#[doc(inline)] pub use crate::dll::AzRefAny as RefAny;
    impl RefAny {
        /// Creates a new `RefAny` instance.
        pub fn new_c(ptr: *const c_void, len: usize, type_id: u64, type_name: String, destructor: RefAnyDestructorType) -> Self { unsafe { crate::dll::az_ref_any_new_c(ptr, len, type_id, type_name, destructor) } }
        /// Calls the `RefAny::is_type` function.
        pub fn is_type(&self, type_id: u64)  -> bool { unsafe { crate::dll::az_ref_any_is_type(self, type_id) } }
        /// Calls the `RefAny::get_type_name` function.
        pub fn get_type_name(&self)  -> crate::str::String { unsafe { crate::dll::az_ref_any_get_type_name(self) } }
        /// Calls the `RefAny::clone` function.
        pub fn clone(&mut self)  -> crate::callbacks::RefAny { unsafe { crate::dll::az_ref_any_clone(self) } }
    }

    impl Drop for RefAny { fn drop(&mut self) { unsafe { crate::dll::az_ref_any_delete(self) } } }
    /// `LayoutInfo` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutInfo as LayoutInfo;
    impl LayoutInfo {
        /// Calls the `LayoutInfo::window_width_larger_than` function.
        pub fn window_width_larger_than(&mut self, width: f32)  -> bool { unsafe { crate::dll::az_layout_info_window_width_larger_than(self, width) } }
        /// Calls the `LayoutInfo::window_width_smaller_than` function.
        pub fn window_width_smaller_than(&mut self, width: f32)  -> bool { unsafe { crate::dll::az_layout_info_window_width_smaller_than(self, width) } }
        /// Calls the `LayoutInfo::window_height_larger_than` function.
        pub fn window_height_larger_than(&mut self, width: f32)  -> bool { unsafe { crate::dll::az_layout_info_window_height_larger_than(self, width) } }
        /// Calls the `LayoutInfo::window_height_smaller_than` function.
        pub fn window_height_smaller_than(&mut self, width: f32)  -> bool { unsafe { crate::dll::az_layout_info_window_height_smaller_than(self, width) } }
    }

}

pub mod css {
    #![allow(dead_code, unused_imports)]
    //! `Css` parsing module
    use crate::dll::*;
    use core::ffi::c_void;
    use crate::vec::{StyleBackgroundPositionVec, StyleBackgroundContentVec, StyleBackgroundSizeVec, StyleBackgroundRepeatVec, StyleTransformVec};

    macro_rules! css_property_from_type {($prop_type:expr, $content_type:ident) => ({
        match $prop_type {
            CssPropertyType::TextColor => CssProperty::TextColor(StyleTextColorValue::$content_type),
            CssPropertyType::FontSize => CssProperty::FontSize(StyleFontSizeValue::$content_type),
            CssPropertyType::FontFamily => CssProperty::FontFamily(StyleFontFamilyValue::$content_type),
            CssPropertyType::TextAlign => CssProperty::TextAlign(StyleTextAlignmentHorzValue::$content_type),
            CssPropertyType::LetterSpacing => CssProperty::LetterSpacing(StyleLetterSpacingValue::$content_type),
            CssPropertyType::LineHeight => CssProperty::LineHeight(StyleLineHeightValue::$content_type),
            CssPropertyType::WordSpacing => CssProperty::WordSpacing(StyleWordSpacingValue::$content_type),
            CssPropertyType::TabWidth => CssProperty::TabWidth(StyleTabWidthValue::$content_type),
            CssPropertyType::Cursor => CssProperty::Cursor(StyleCursorValue::$content_type),
            CssPropertyType::Display => CssProperty::Display(LayoutDisplayValue::$content_type),
            CssPropertyType::Float => CssProperty::Float(LayoutFloatValue::$content_type),
            CssPropertyType::BoxSizing => CssProperty::BoxSizing(LayoutBoxSizingValue::$content_type),
            CssPropertyType::Width => CssProperty::Width(LayoutWidthValue::$content_type),
            CssPropertyType::Height => CssProperty::Height(LayoutHeightValue::$content_type),
            CssPropertyType::MinWidth => CssProperty::MinWidth(LayoutMinWidthValue::$content_type),
            CssPropertyType::MinHeight => CssProperty::MinHeight(LayoutMinHeightValue::$content_type),
            CssPropertyType::MaxWidth => CssProperty::MaxWidth(LayoutMaxWidthValue::$content_type),
            CssPropertyType::MaxHeight => CssProperty::MaxHeight(LayoutMaxHeightValue::$content_type),
            CssPropertyType::Position => CssProperty::Position(LayoutPositionValue::$content_type),
            CssPropertyType::Top => CssProperty::Top(LayoutTopValue::$content_type),
            CssPropertyType::Right => CssProperty::Right(LayoutRightValue::$content_type),
            CssPropertyType::Left => CssProperty::Left(LayoutLeftValue::$content_type),
            CssPropertyType::Bottom => CssProperty::Bottom(LayoutBottomValue::$content_type),
            CssPropertyType::FlexWrap => CssProperty::FlexWrap(LayoutFlexWrapValue::$content_type),
            CssPropertyType::FlexDirection => CssProperty::FlexDirection(LayoutFlexDirectionValue::$content_type),
            CssPropertyType::FlexGrow => CssProperty::FlexGrow(LayoutFlexGrowValue::$content_type),
            CssPropertyType::FlexShrink => CssProperty::FlexShrink(LayoutFlexShrinkValue::$content_type),
            CssPropertyType::JustifyContent => CssProperty::JustifyContent(LayoutJustifyContentValue::$content_type),
            CssPropertyType::AlignItems => CssProperty::AlignItems(LayoutAlignItemsValue::$content_type),
            CssPropertyType::AlignContent => CssProperty::AlignContent(LayoutAlignContentValue::$content_type),
            CssPropertyType::Background => CssProperty::BackgroundContent(StyleBackgroundContentVecValue::$content_type),
            CssPropertyType::BackgroundImage => CssProperty::BackgroundContent(StyleBackgroundContentVecValue::$content_type),
            CssPropertyType::BackgroundColor => CssProperty::BackgroundContent(StyleBackgroundContentVecValue::$content_type),
            CssPropertyType::BackgroundPosition => CssProperty::BackgroundPosition(StyleBackgroundPositionVecValue::$content_type),
            CssPropertyType::BackgroundSize => CssProperty::BackgroundSize(StyleBackgroundSizeVecValue::$content_type),
            CssPropertyType::BackgroundRepeat => CssProperty::BackgroundRepeat(StyleBackgroundRepeatVecValue::$content_type),
            CssPropertyType::OverflowX => CssProperty::OverflowX(LayoutOverflowValue::$content_type),
            CssPropertyType::OverflowY => CssProperty::OverflowY(LayoutOverflowValue::$content_type),
            CssPropertyType::PaddingTop => CssProperty::PaddingTop(LayoutPaddingTopValue::$content_type),
            CssPropertyType::PaddingLeft => CssProperty::PaddingLeft(LayoutPaddingLeftValue::$content_type),
            CssPropertyType::PaddingRight => CssProperty::PaddingRight(LayoutPaddingRightValue::$content_type),
            CssPropertyType::PaddingBottom => CssProperty::PaddingBottom(LayoutPaddingBottomValue::$content_type),
            CssPropertyType::MarginTop => CssProperty::MarginTop(LayoutMarginTopValue::$content_type),
            CssPropertyType::MarginLeft => CssProperty::MarginLeft(LayoutMarginLeftValue::$content_type),
            CssPropertyType::MarginRight => CssProperty::MarginRight(LayoutMarginRightValue::$content_type),
            CssPropertyType::MarginBottom => CssProperty::MarginBottom(LayoutMarginBottomValue::$content_type),
            CssPropertyType::BorderTopLeftRadius => CssProperty::BorderTopLeftRadius(StyleBorderTopLeftRadiusValue::$content_type),
            CssPropertyType::BorderTopRightRadius => CssProperty::BorderTopRightRadius(StyleBorderTopRightRadiusValue::$content_type),
            CssPropertyType::BorderBottomLeftRadius => CssProperty::BorderBottomLeftRadius(StyleBorderBottomLeftRadiusValue::$content_type),
            CssPropertyType::BorderBottomRightRadius => CssProperty::BorderBottomRightRadius(StyleBorderBottomRightRadiusValue::$content_type),
            CssPropertyType::BorderTopColor => CssProperty::BorderTopColor(StyleBorderTopColorValue::$content_type),
            CssPropertyType::BorderRightColor => CssProperty::BorderRightColor(StyleBorderRightColorValue::$content_type),
            CssPropertyType::BorderLeftColor => CssProperty::BorderLeftColor(StyleBorderLeftColorValue::$content_type),
            CssPropertyType::BorderBottomColor => CssProperty::BorderBottomColor(StyleBorderBottomColorValue::$content_type),
            CssPropertyType::BorderTopStyle => CssProperty::BorderTopStyle(StyleBorderTopStyleValue::$content_type),
            CssPropertyType::BorderRightStyle => CssProperty::BorderRightStyle(StyleBorderRightStyleValue::$content_type),
            CssPropertyType::BorderLeftStyle => CssProperty::BorderLeftStyle(StyleBorderLeftStyleValue::$content_type),
            CssPropertyType::BorderBottomStyle => CssProperty::BorderBottomStyle(StyleBorderBottomStyleValue::$content_type),
            CssPropertyType::BorderTopWidth => CssProperty::BorderTopWidth(LayoutBorderTopWidthValue::$content_type),
            CssPropertyType::BorderRightWidth => CssProperty::BorderRightWidth(LayoutBorderRightWidthValue::$content_type),
            CssPropertyType::BorderLeftWidth => CssProperty::BorderLeftWidth(LayoutBorderLeftWidthValue::$content_type),
            CssPropertyType::BorderBottomWidth => CssProperty::BorderBottomWidth(LayoutBorderBottomWidthValue::$content_type),
            CssPropertyType::BoxShadowLeft => CssProperty::BoxShadowLeft(StyleBoxShadowValue::$content_type),
            CssPropertyType::BoxShadowRight => CssProperty::BoxShadowRight(StyleBoxShadowValue::$content_type),
            CssPropertyType::BoxShadowTop => CssProperty::BoxShadowTop(StyleBoxShadowValue::$content_type),
            CssPropertyType::BoxShadowBottom => CssProperty::BoxShadowBottom(StyleBoxShadowValue::$content_type),
            CssPropertyType::ScrollbarStyle => CssProperty::ScrollbarStyle(ScrollbarStyleValue::$content_type),
            CssPropertyType::Opacity => CssProperty::Opacity(StyleOpacityValue::$content_type),
            CssPropertyType::Transform => CssProperty::Transform(StyleTransformVecValue::$content_type),
            CssPropertyType::PerspectiveOrigin => CssProperty::PerspectiveOrigin(StylePerspectiveOriginValue::$content_type),
            CssPropertyType::TransformOrigin => CssProperty::TransformOrigin(StyleTransformOriginValue::$content_type),
            CssPropertyType::BackfaceVisibility => CssProperty::BackfaceVisibility(StyleBackfaceVisibilityValue::$content_type),
        }
    })}

    impl CssProperty {

        /// Return the type (key) of this property as a statically typed enum
        pub const fn get_type(&self) -> CssPropertyType {
            match &self {
                CssProperty::TextColor(_) => CssPropertyType::TextColor,
                CssProperty::FontSize(_) => CssPropertyType::FontSize,
                CssProperty::FontFamily(_) => CssPropertyType::FontFamily,
                CssProperty::TextAlign(_) => CssPropertyType::TextAlign,
                CssProperty::LetterSpacing(_) => CssPropertyType::LetterSpacing,
                CssProperty::LineHeight(_) => CssPropertyType::LineHeight,
                CssProperty::WordSpacing(_) => CssPropertyType::WordSpacing,
                CssProperty::TabWidth(_) => CssPropertyType::TabWidth,
                CssProperty::Cursor(_) => CssPropertyType::Cursor,
                CssProperty::Display(_) => CssPropertyType::Display,
                CssProperty::Float(_) => CssPropertyType::Float,
                CssProperty::BoxSizing(_) => CssPropertyType::BoxSizing,
                CssProperty::Width(_) => CssPropertyType::Width,
                CssProperty::Height(_) => CssPropertyType::Height,
                CssProperty::MinWidth(_) => CssPropertyType::MinWidth,
                CssProperty::MinHeight(_) => CssPropertyType::MinHeight,
                CssProperty::MaxWidth(_) => CssPropertyType::MaxWidth,
                CssProperty::MaxHeight(_) => CssPropertyType::MaxHeight,
                CssProperty::Position(_) => CssPropertyType::Position,
                CssProperty::Top(_) => CssPropertyType::Top,
                CssProperty::Right(_) => CssPropertyType::Right,
                CssProperty::Left(_) => CssPropertyType::Left,
                CssProperty::Bottom(_) => CssPropertyType::Bottom,
                CssProperty::FlexWrap(_) => CssPropertyType::FlexWrap,
                CssProperty::FlexDirection(_) => CssPropertyType::FlexDirection,
                CssProperty::FlexGrow(_) => CssPropertyType::FlexGrow,
                CssProperty::FlexShrink(_) => CssPropertyType::FlexShrink,
                CssProperty::JustifyContent(_) => CssPropertyType::JustifyContent,
                CssProperty::AlignItems(_) => CssPropertyType::AlignItems,
                CssProperty::AlignContent(_) => CssPropertyType::AlignContent,
                CssProperty::BackgroundContent(_) => CssPropertyType::BackgroundImage, // TODO: wrong!
                CssProperty::BackgroundPosition(_) => CssPropertyType::BackgroundPosition,
                CssProperty::BackgroundSize(_) => CssPropertyType::BackgroundSize,
                CssProperty::BackgroundRepeat(_) => CssPropertyType::BackgroundRepeat,
                CssProperty::OverflowX(_) => CssPropertyType::OverflowX,
                CssProperty::OverflowY(_) => CssPropertyType::OverflowY,
                CssProperty::PaddingTop(_) => CssPropertyType::PaddingTop,
                CssProperty::PaddingLeft(_) => CssPropertyType::PaddingLeft,
                CssProperty::PaddingRight(_) => CssPropertyType::PaddingRight,
                CssProperty::PaddingBottom(_) => CssPropertyType::PaddingBottom,
                CssProperty::MarginTop(_) => CssPropertyType::MarginTop,
                CssProperty::MarginLeft(_) => CssPropertyType::MarginLeft,
                CssProperty::MarginRight(_) => CssPropertyType::MarginRight,
                CssProperty::MarginBottom(_) => CssPropertyType::MarginBottom,
                CssProperty::BorderTopLeftRadius(_) => CssPropertyType::BorderTopLeftRadius,
                CssProperty::BorderTopRightRadius(_) => CssPropertyType::BorderTopRightRadius,
                CssProperty::BorderBottomLeftRadius(_) => CssPropertyType::BorderBottomLeftRadius,
                CssProperty::BorderBottomRightRadius(_) => CssPropertyType::BorderBottomRightRadius,
                CssProperty::BorderTopColor(_) => CssPropertyType::BorderTopColor,
                CssProperty::BorderRightColor(_) => CssPropertyType::BorderRightColor,
                CssProperty::BorderLeftColor(_) => CssPropertyType::BorderLeftColor,
                CssProperty::BorderBottomColor(_) => CssPropertyType::BorderBottomColor,
                CssProperty::BorderTopStyle(_) => CssPropertyType::BorderTopStyle,
                CssProperty::BorderRightStyle(_) => CssPropertyType::BorderRightStyle,
                CssProperty::BorderLeftStyle(_) => CssPropertyType::BorderLeftStyle,
                CssProperty::BorderBottomStyle(_) => CssPropertyType::BorderBottomStyle,
                CssProperty::BorderTopWidth(_) => CssPropertyType::BorderTopWidth,
                CssProperty::BorderRightWidth(_) => CssPropertyType::BorderRightWidth,
                CssProperty::BorderLeftWidth(_) => CssPropertyType::BorderLeftWidth,
                CssProperty::BorderBottomWidth(_) => CssPropertyType::BorderBottomWidth,
                CssProperty::BoxShadowLeft(_) => CssPropertyType::BoxShadowLeft,
                CssProperty::BoxShadowRight(_) => CssPropertyType::BoxShadowRight,
                CssProperty::BoxShadowTop(_) => CssPropertyType::BoxShadowTop,
                CssProperty::BoxShadowBottom(_) => CssPropertyType::BoxShadowBottom,
                CssProperty::ScrollbarStyle(_) => CssPropertyType::ScrollbarStyle,
                CssProperty::Opacity(_) => CssPropertyType::Opacity,
                CssProperty::Transform(_) => CssPropertyType::Transform,
                CssProperty::PerspectiveOrigin(_) => CssPropertyType::PerspectiveOrigin,
                CssProperty::TransformOrigin(_) => CssPropertyType::TransformOrigin,
                CssProperty::BackfaceVisibility(_) => CssPropertyType::BackfaceVisibility,
            }
        }

        // const constructors for easier API access

        pub const fn none(prop_type: CssPropertyType) -> Self { css_property_from_type!(prop_type, None) }
        pub const fn auto(prop_type: CssPropertyType) -> Self { css_property_from_type!(prop_type, Auto) }
        pub const fn initial(prop_type: CssPropertyType) -> Self { css_property_from_type!(prop_type, Initial) }
        pub const fn inherit(prop_type: CssPropertyType) -> Self { css_property_from_type!(prop_type, Inherit) }

        pub const fn text_color(input: StyleTextColor) -> Self { CssProperty::TextColor(StyleTextColorValue::Exact(input)) }
        pub const fn font_size(input: StyleFontSize) -> Self { CssProperty::FontSize(StyleFontSizeValue::Exact(input)) }
        pub const fn font_family(input: StyleFontFamily) -> Self { CssProperty::FontFamily(StyleFontFamilyValue::Exact(input)) }
        pub const fn text_align(input: StyleTextAlignmentHorz) -> Self { CssProperty::TextAlign(StyleTextAlignmentHorzValue::Exact(input)) }
        pub const fn letter_spacing(input: StyleLetterSpacing) -> Self { CssProperty::LetterSpacing(StyleLetterSpacingValue::Exact(input)) }
        pub const fn line_height(input: StyleLineHeight) -> Self { CssProperty::LineHeight(StyleLineHeightValue::Exact(input)) }
        pub const fn word_spacing(input: StyleWordSpacing) -> Self { CssProperty::WordSpacing(StyleWordSpacingValue::Exact(input)) }
        pub const fn tab_width(input: StyleTabWidth) -> Self { CssProperty::TabWidth(StyleTabWidthValue::Exact(input)) }
        pub const fn cursor(input: StyleCursor) -> Self { CssProperty::Cursor(StyleCursorValue::Exact(input)) }
        pub const fn display(input: LayoutDisplay) -> Self { CssProperty::Display(LayoutDisplayValue::Exact(input)) }
        pub const fn float(input: LayoutFloat) -> Self { CssProperty::Float(LayoutFloatValue::Exact(input)) }
        pub const fn box_sizing(input: LayoutBoxSizing) -> Self { CssProperty::BoxSizing(LayoutBoxSizingValue::Exact(input)) }
        pub const fn width(input: LayoutWidth) -> Self { CssProperty::Width(LayoutWidthValue::Exact(input)) }
        pub const fn height(input: LayoutHeight) -> Self { CssProperty::Height(LayoutHeightValue::Exact(input)) }
        pub const fn min_width(input: LayoutMinWidth) -> Self { CssProperty::MinWidth(LayoutMinWidthValue::Exact(input)) }
        pub const fn min_height(input: LayoutMinHeight) -> Self { CssProperty::MinHeight(LayoutMinHeightValue::Exact(input)) }
        pub const fn max_width(input: LayoutMaxWidth) -> Self { CssProperty::MaxWidth(LayoutMaxWidthValue::Exact(input)) }
        pub const fn max_height(input: LayoutMaxHeight) -> Self { CssProperty::MaxHeight(LayoutMaxHeightValue::Exact(input)) }
        pub const fn position(input: LayoutPosition) -> Self { CssProperty::Position(LayoutPositionValue::Exact(input)) }
        pub const fn top(input: LayoutTop) -> Self { CssProperty::Top(LayoutTopValue::Exact(input)) }
        pub const fn right(input: LayoutRight) -> Self { CssProperty::Right(LayoutRightValue::Exact(input)) }
        pub const fn left(input: LayoutLeft) -> Self { CssProperty::Left(LayoutLeftValue::Exact(input)) }
        pub const fn bottom(input: LayoutBottom) -> Self { CssProperty::Bottom(LayoutBottomValue::Exact(input)) }
        pub const fn flex_wrap(input: LayoutFlexWrap) -> Self { CssProperty::FlexWrap(LayoutFlexWrapValue::Exact(input)) }
        pub const fn flex_direction(input: LayoutFlexDirection) -> Self { CssProperty::FlexDirection(LayoutFlexDirectionValue::Exact(input)) }
        pub const fn flex_grow(input: LayoutFlexGrow) -> Self { CssProperty::FlexGrow(LayoutFlexGrowValue::Exact(input)) }
        pub const fn flex_shrink(input: LayoutFlexShrink) -> Self { CssProperty::FlexShrink(LayoutFlexShrinkValue::Exact(input)) }
        pub const fn justify_content(input: LayoutJustifyContent) -> Self { CssProperty::JustifyContent(LayoutJustifyContentValue::Exact(input)) }
        pub const fn align_items(input: LayoutAlignItems) -> Self { CssProperty::AlignItems(LayoutAlignItemsValue::Exact(input)) }
        pub const fn align_content(input: LayoutAlignContent) -> Self { CssProperty::AlignContent(LayoutAlignContentValue::Exact(input)) }
        pub const fn background_content(input: StyleBackgroundContentVec) -> Self { CssProperty::BackgroundContent(StyleBackgroundContentVecValue::Exact(input)) }
        pub const fn background_position(input: StyleBackgroundPositionVec) -> Self { CssProperty::BackgroundPosition(StyleBackgroundPositionVecValue::Exact(input)) }
        pub const fn background_size(input: StyleBackgroundSizeVec) -> Self { CssProperty::BackgroundSize(StyleBackgroundSizeVecValue::Exact(input)) }
        pub const fn background_repeat(input: StyleBackgroundRepeatVec) -> Self { CssProperty::BackgroundRepeat(StyleBackgroundRepeatVecValue::Exact(input)) }
        pub const fn overflow_x(input: LayoutOverflow) -> Self { CssProperty::OverflowX(LayoutOverflowValue::Exact(input)) }
        pub const fn overflow_y(input: LayoutOverflow) -> Self { CssProperty::OverflowY(LayoutOverflowValue::Exact(input)) }
        pub const fn padding_top(input: LayoutPaddingTop) -> Self { CssProperty::PaddingTop(LayoutPaddingTopValue::Exact(input)) }
        pub const fn padding_left(input: LayoutPaddingLeft) -> Self { CssProperty::PaddingLeft(LayoutPaddingLeftValue::Exact(input)) }
        pub const fn padding_right(input: LayoutPaddingRight) -> Self { CssProperty::PaddingRight(LayoutPaddingRightValue::Exact(input)) }
        pub const fn padding_bottom(input: LayoutPaddingBottom) -> Self { CssProperty::PaddingBottom(LayoutPaddingBottomValue::Exact(input)) }
        pub const fn margin_top(input: LayoutMarginTop) -> Self { CssProperty::MarginTop(LayoutMarginTopValue::Exact(input)) }
        pub const fn margin_left(input: LayoutMarginLeft) -> Self { CssProperty::MarginLeft(LayoutMarginLeftValue::Exact(input)) }
        pub const fn margin_right(input: LayoutMarginRight) -> Self { CssProperty::MarginRight(LayoutMarginRightValue::Exact(input)) }
        pub const fn margin_bottom(input: LayoutMarginBottom) -> Self { CssProperty::MarginBottom(LayoutMarginBottomValue::Exact(input)) }
        pub const fn border_top_left_radius(input: StyleBorderTopLeftRadius) -> Self { CssProperty::BorderTopLeftRadius(StyleBorderTopLeftRadiusValue::Exact(input)) }
        pub const fn border_top_right_radius(input: StyleBorderTopRightRadius) -> Self { CssProperty::BorderTopRightRadius(StyleBorderTopRightRadiusValue::Exact(input)) }
        pub const fn border_bottom_left_radius(input: StyleBorderBottomLeftRadius) -> Self { CssProperty::BorderBottomLeftRadius(StyleBorderBottomLeftRadiusValue::Exact(input)) }
        pub const fn border_bottom_right_radius(input: StyleBorderBottomRightRadius) -> Self { CssProperty::BorderBottomRightRadius(StyleBorderBottomRightRadiusValue::Exact(input)) }
        pub const fn border_top_color(input: StyleBorderTopColor) -> Self { CssProperty::BorderTopColor(StyleBorderTopColorValue::Exact(input)) }
        pub const fn border_right_color(input: StyleBorderRightColor) -> Self { CssProperty::BorderRightColor(StyleBorderRightColorValue::Exact(input)) }
        pub const fn border_left_color(input: StyleBorderLeftColor) -> Self { CssProperty::BorderLeftColor(StyleBorderLeftColorValue::Exact(input)) }
        pub const fn border_bottom_color(input: StyleBorderBottomColor) -> Self { CssProperty::BorderBottomColor(StyleBorderBottomColorValue::Exact(input)) }
        pub const fn border_top_style(input: StyleBorderTopStyle) -> Self { CssProperty::BorderTopStyle(StyleBorderTopStyleValue::Exact(input)) }
        pub const fn border_right_style(input: StyleBorderRightStyle) -> Self { CssProperty::BorderRightStyle(StyleBorderRightStyleValue::Exact(input)) }
        pub const fn border_left_style(input: StyleBorderLeftStyle) -> Self { CssProperty::BorderLeftStyle(StyleBorderLeftStyleValue::Exact(input)) }
        pub const fn border_bottom_style(input: StyleBorderBottomStyle) -> Self { CssProperty::BorderBottomStyle(StyleBorderBottomStyleValue::Exact(input)) }
        pub const fn border_top_width(input: LayoutBorderTopWidth) -> Self { CssProperty::BorderTopWidth(LayoutBorderTopWidthValue::Exact(input)) }
        pub const fn border_right_width(input: LayoutBorderRightWidth) -> Self { CssProperty::BorderRightWidth(LayoutBorderRightWidthValue::Exact(input)) }
        pub const fn border_left_width(input: LayoutBorderLeftWidth) -> Self { CssProperty::BorderLeftWidth(LayoutBorderLeftWidthValue::Exact(input)) }
        pub const fn border_bottom_width(input: LayoutBorderBottomWidth) -> Self { CssProperty::BorderBottomWidth(LayoutBorderBottomWidthValue::Exact(input)) }
        pub const fn box_shadow_left(input: StyleBoxShadow) -> Self { CssProperty::BoxShadowLeft(StyleBoxShadowValue::Exact(input)) }
        pub const fn box_shadow_right(input: StyleBoxShadow) -> Self { CssProperty::BoxShadowRight(StyleBoxShadowValue::Exact(input)) }
        pub const fn box_shadow_top(input: StyleBoxShadow) -> Self { CssProperty::BoxShadowTop(StyleBoxShadowValue::Exact(input)) }
        pub const fn box_shadow_bottom(input: StyleBoxShadow) -> Self { CssProperty::BoxShadowBottom(StyleBoxShadowValue::Exact(input)) }
        pub const fn opacity(input: StyleOpacity) -> Self { CssProperty::Opacity(StyleOpacityValue::Exact(input)) }
        pub const fn transform(input: StyleTransformVec) -> Self { CssProperty::Transform(StyleTransformVecValue::Exact(input)) }
        pub const fn transform_origin(input: StyleTransformOrigin) -> Self { CssProperty::TransformOrigin(StyleTransformOriginValue::Exact(input)) }
        pub const fn perspective_origin(input: StylePerspectiveOrigin) -> Self { CssProperty::PerspectiveOrigin(StylePerspectiveOriginValue::Exact(input)) }
        pub const fn backface_visiblity(input: StyleBackfaceVisibility) -> Self { CssProperty::BackfaceVisibility(StyleBackfaceVisibilityValue::Exact(input)) }

        pub const fn as_background(&self) -> Option<&StyleBackgroundContentVecValue> { match self { CssProperty::BackgroundContent(f) => Some(f), _ => None, } }
        pub const fn as_background_position(&self) -> Option<&StyleBackgroundPositionVecValue> { match self { CssProperty::BackgroundPosition(f) => Some(f), _ => None, } }
        pub const fn as_background_size(&self) -> Option<&StyleBackgroundSizeVecValue> { match self { CssProperty::BackgroundSize(f) => Some(f), _ => None, } }
        pub const fn as_background_repeat(&self) -> Option<&StyleBackgroundRepeatVecValue> { match self { CssProperty::BackgroundRepeat(f) => Some(f), _ => None, } }
        pub const fn as_font_size(&self) -> Option<&StyleFontSizeValue> { match self { CssProperty::FontSize(f) => Some(f), _ => None, } }
        pub const fn as_font_family(&self) -> Option<&StyleFontFamilyValue> { match self { CssProperty::FontFamily(f) => Some(f), _ => None, } }
        pub const fn as_text_color(&self) -> Option<&StyleTextColorValue> { match self { CssProperty::TextColor(f) => Some(f), _ => None, } }
        pub const fn as_text_align(&self) -> Option<&StyleTextAlignmentHorzValue> { match self { CssProperty::TextAlign(f) => Some(f), _ => None, } }
        pub const fn as_line_height(&self) -> Option<&StyleLineHeightValue> { match self { CssProperty::LineHeight(f) => Some(f), _ => None, } }
        pub const fn as_letter_spacing(&self) -> Option<&StyleLetterSpacingValue> { match self { CssProperty::LetterSpacing(f) => Some(f), _ => None, } }
        pub const fn as_word_spacing(&self) -> Option<&StyleWordSpacingValue> { match self { CssProperty::WordSpacing(f) => Some(f), _ => None, } }
        pub const fn as_tab_width(&self) -> Option<&StyleTabWidthValue> { match self { CssProperty::TabWidth(f) => Some(f), _ => None, } }
        pub const fn as_cursor(&self) -> Option<&StyleCursorValue> { match self { CssProperty::Cursor(f) => Some(f), _ => None, } }
        pub const fn as_box_shadow_left(&self) -> Option<&StyleBoxShadowValue> { match self { CssProperty::BoxShadowLeft(f) => Some(f), _ => None, } }
        pub const fn as_box_shadow_right(&self) -> Option<&StyleBoxShadowValue> { match self { CssProperty::BoxShadowRight(f) => Some(f), _ => None, } }
        pub const fn as_box_shadow_top(&self) -> Option<&StyleBoxShadowValue> { match self { CssProperty::BoxShadowTop(f) => Some(f), _ => None, } }
        pub const fn as_box_shadow_bottom(&self) -> Option<&StyleBoxShadowValue> { match self { CssProperty::BoxShadowBottom(f) => Some(f), _ => None, } }
        pub const fn as_border_top_color(&self) -> Option<&StyleBorderTopColorValue> { match self { CssProperty::BorderTopColor(f) => Some(f), _ => None, } }
        pub const fn as_border_left_color(&self) -> Option<&StyleBorderLeftColorValue> { match self { CssProperty::BorderLeftColor(f) => Some(f), _ => None, } }
        pub const fn as_border_right_color(&self) -> Option<&StyleBorderRightColorValue> { match self { CssProperty::BorderRightColor(f) => Some(f), _ => None, } }
        pub const fn as_border_bottom_color(&self) -> Option<&StyleBorderBottomColorValue> { match self { CssProperty::BorderBottomColor(f) => Some(f), _ => None, } }
        pub const fn as_border_top_style(&self) -> Option<&StyleBorderTopStyleValue> { match self { CssProperty::BorderTopStyle(f) => Some(f), _ => None, } }
        pub const fn as_border_left_style(&self) -> Option<&StyleBorderLeftStyleValue> { match self { CssProperty::BorderLeftStyle(f) => Some(f), _ => None, } }
        pub const fn as_border_right_style(&self) -> Option<&StyleBorderRightStyleValue> { match self { CssProperty::BorderRightStyle(f) => Some(f), _ => None, } }
        pub const fn as_border_bottom_style(&self) -> Option<&StyleBorderBottomStyleValue> { match self { CssProperty::BorderBottomStyle(f) => Some(f), _ => None, } }
        pub const fn as_border_top_left_radius(&self) -> Option<&StyleBorderTopLeftRadiusValue> { match self { CssProperty::BorderTopLeftRadius(f) => Some(f), _ => None, } }
        pub const fn as_border_top_right_radius(&self) -> Option<&StyleBorderTopRightRadiusValue> { match self { CssProperty::BorderTopRightRadius(f) => Some(f), _ => None, } }
        pub const fn as_border_bottom_left_radius(&self) -> Option<&StyleBorderBottomLeftRadiusValue> { match self { CssProperty::BorderBottomLeftRadius(f) => Some(f), _ => None, } }
        pub const fn as_border_bottom_right_radius(&self) -> Option<&StyleBorderBottomRightRadiusValue> { match self { CssProperty::BorderBottomRightRadius(f) => Some(f), _ => None, } }
        pub const fn as_opacity(&self) -> Option<&StyleOpacityValue> { match self { CssProperty::Opacity(f) => Some(f), _ => None, } }
        pub const fn as_transform(&self) -> Option<&StyleTransformVecValue> { match self { CssProperty::Transform(f) => Some(f), _ => None, } }
        pub const fn as_transform_origin(&self) -> Option<&StyleTransformOriginValue> { match self { CssProperty::TransformOrigin(f) => Some(f), _ => None, } }
        pub const fn as_perspective_origin(&self) -> Option<&StylePerspectiveOriginValue> { match self { CssProperty::PerspectiveOrigin(f) => Some(f), _ => None, } }
        pub const fn as_backface_visibility(&self) -> Option<&StyleBackfaceVisibilityValue> { match self { CssProperty::BackfaceVisibility(f) => Some(f), _ => None, } }
        pub const fn as_display(&self) -> Option<&LayoutDisplayValue> { match self { CssProperty::Display(f) => Some(f), _ => None, } }
        pub const fn as_float(&self) -> Option<&LayoutFloatValue> { match self { CssProperty::Float(f) => Some(f), _ => None, } }
        pub const fn as_box_sizing(&self) -> Option<&LayoutBoxSizingValue> { match self { CssProperty::BoxSizing(f) => Some(f), _ => None, } }
        pub const fn as_width(&self) -> Option<&LayoutWidthValue> { match self { CssProperty::Width(f) => Some(f), _ => None, } }
        pub const fn as_height(&self) -> Option<&LayoutHeightValue> { match self { CssProperty::Height(f) => Some(f), _ => None, } }
        pub const fn as_min_width(&self) -> Option<&LayoutMinWidthValue> { match self { CssProperty::MinWidth(f) => Some(f), _ => None, } }
        pub const fn as_min_height(&self) -> Option<&LayoutMinHeightValue> { match self { CssProperty::MinHeight(f) => Some(f), _ => None, } }
        pub const fn as_max_width(&self) -> Option<&LayoutMaxWidthValue> { match self { CssProperty::MaxWidth(f) => Some(f), _ => None, } }
        pub const fn as_max_height(&self) -> Option<&LayoutMaxHeightValue> { match self { CssProperty::MaxHeight(f) => Some(f), _ => None, } }
        pub const fn as_position(&self) -> Option<&LayoutPositionValue> { match self { CssProperty::Position(f) => Some(f), _ => None, } }
        pub const fn as_top(&self) -> Option<&LayoutTopValue> { match self { CssProperty::Top(f) => Some(f), _ => None, } }
        pub const fn as_bottom(&self) -> Option<&LayoutBottomValue> { match self { CssProperty::Bottom(f) => Some(f), _ => None, } }
        pub const fn as_right(&self) -> Option<&LayoutRightValue> { match self { CssProperty::Right(f) => Some(f), _ => None, } }
        pub const fn as_left(&self) -> Option<&LayoutLeftValue> { match self { CssProperty::Left(f) => Some(f), _ => None, } }
        pub const fn as_padding_top(&self) -> Option<&LayoutPaddingTopValue> { match self { CssProperty::PaddingTop(f) => Some(f), _ => None, } }
        pub const fn as_padding_bottom(&self) -> Option<&LayoutPaddingBottomValue> { match self { CssProperty::PaddingBottom(f) => Some(f), _ => None, } }
        pub const fn as_padding_left(&self) -> Option<&LayoutPaddingLeftValue> { match self { CssProperty::PaddingLeft(f) => Some(f), _ => None, } }
        pub const fn as_padding_right(&self) -> Option<&LayoutPaddingRightValue> { match self { CssProperty::PaddingRight(f) => Some(f), _ => None, } }
        pub const fn as_margin_top(&self) -> Option<&LayoutMarginTopValue> { match self { CssProperty::MarginTop(f) => Some(f), _ => None, } }
        pub const fn as_margin_bottom(&self) -> Option<&LayoutMarginBottomValue> { match self { CssProperty::MarginBottom(f) => Some(f), _ => None, } }
        pub const fn as_margin_left(&self) -> Option<&LayoutMarginLeftValue> { match self { CssProperty::MarginLeft(f) => Some(f), _ => None, } }
        pub const fn as_margin_right(&self) -> Option<&LayoutMarginRightValue> { match self { CssProperty::MarginRight(f) => Some(f), _ => None, } }
        pub const fn as_border_top_width(&self) -> Option<&LayoutBorderTopWidthValue> { match self { CssProperty::BorderTopWidth(f) => Some(f), _ => None, } }
        pub const fn as_border_left_width(&self) -> Option<&LayoutBorderLeftWidthValue> { match self { CssProperty::BorderLeftWidth(f) => Some(f), _ => None, } }
        pub const fn as_border_right_width(&self) -> Option<&LayoutBorderRightWidthValue> { match self { CssProperty::BorderRightWidth(f) => Some(f), _ => None, } }
        pub const fn as_border_bottom_width(&self) -> Option<&LayoutBorderBottomWidthValue> { match self { CssProperty::BorderBottomWidth(f) => Some(f), _ => None, } }
        pub const fn as_overflow_x(&self) -> Option<&LayoutOverflowValue> { match self { CssProperty::OverflowX(f) => Some(f), _ => None, } }
        pub const fn as_overflow_y(&self) -> Option<&LayoutOverflowValue> { match self { CssProperty::OverflowY(f) => Some(f), _ => None, } }
        pub const fn as_direction(&self) -> Option<&LayoutFlexDirectionValue> { match self { CssProperty::FlexDirection(f) => Some(f), _ => None, } }
        pub const fn as_flex_wrap(&self) -> Option<&LayoutFlexWrapValue> { match self { CssProperty::FlexWrap(f) => Some(f), _ => None, } }
        pub const fn as_flex_grow(&self) -> Option<&LayoutFlexGrowValue> { match self { CssProperty::FlexGrow(f) => Some(f), _ => None, } }
        pub const fn as_flex_shrink(&self) -> Option<&LayoutFlexShrinkValue> { match self { CssProperty::FlexShrink(f) => Some(f), _ => None, } }
        pub const fn as_justify_content(&self) -> Option<&LayoutJustifyContentValue> { match self { CssProperty::JustifyContent(f) => Some(f), _ => None, } }
        pub const fn as_align_items(&self) -> Option<&LayoutAlignItemsValue> { match self { CssProperty::AlignItems(f) => Some(f), _ => None, } }
        pub const fn as_align_content(&self) -> Option<&LayoutAlignContentValue> { match self { CssProperty::AlignContent(f) => Some(f), _ => None, } }
    }

    const FP_PRECISION_MULTIPLIER: f32 = 1000.0;
    const FP_PRECISION_MULTIPLIER_CONST: isize = FP_PRECISION_MULTIPLIER as isize;

    impl FloatValue {
        /// Same as `FloatValue::new()`, but only accepts whole numbers,
        /// since using `f32` in const fn is not yet stabilized.
        pub const fn const_new(value: isize)  -> Self {
            Self { number: value * FP_PRECISION_MULTIPLIER_CONST }
        }

        pub fn new(value: f32) -> Self {
            Self { number: (value * FP_PRECISION_MULTIPLIER) as isize }
        }

        pub fn get(&self) -> f32 {
            self.number as f32 / FP_PRECISION_MULTIPLIER
        }
    }

    impl From<f32> for FloatValue {
        fn from(val: f32) -> Self {
            Self::new(val)
        }
    }

    impl PixelValue {

        #[inline]
        pub const fn zero() -> Self {
            const ZERO_PX: PixelValue = PixelValue::const_px(0);
            ZERO_PX
        }

        /// Same as `PixelValue::px()`, but only accepts whole numbers,
        /// since using `f32` in const fn is not yet stabilized.
        #[inline]
        pub const fn const_px(value: isize) -> Self {
            Self::const_from_metric(SizeMetric::Px, value)
        }

        /// Same as `PixelValue::em()`, but only accepts whole numbers,
        /// since using `f32` in const fn is not yet stabilized.
        #[inline]
        pub const fn const_em(value: isize) -> Self {
            Self::const_from_metric(SizeMetric::Em, value)
        }

        /// Same as `PixelValue::pt()`, but only accepts whole numbers,
        /// since using `f32` in const fn is not yet stabilized.
        #[inline]
        pub const fn const_pt(value: isize) -> Self {
            Self::const_from_metric(SizeMetric::Pt, value)
        }

        /// Same as `PixelValue::pt()`, but only accepts whole numbers,
        /// since using `f32` in const fn is not yet stabilized.
        #[inline]
        pub const fn const_percent(value: isize) -> Self {
            Self::const_from_metric(SizeMetric::Percent, value)
        }

        #[inline]
        pub const fn const_from_metric(metric: SizeMetric, value: isize) -> Self {
            Self {
                metric: metric,
                number: FloatValue::const_new(value),
            }
        }

        #[inline]
        pub fn px(value: f32) -> Self {
            Self::from_metric(SizeMetric::Px, value)
        }

        #[inline]
        pub fn em(value: f32) -> Self {
            Self::from_metric(SizeMetric::Em, value)
        }

        #[inline]
        pub fn pt(value: f32) -> Self {
            Self::from_metric(SizeMetric::Pt, value)
        }

        #[inline]
        pub fn percent(value: f32) -> Self {
            Self::from_metric(SizeMetric::Percent, value)
        }

        #[inline]
        pub fn from_metric(metric: SizeMetric, value: f32) -> Self {
            Self {
                metric: metric,
                number: FloatValue::new(value),
            }
        }
    }

    impl PixelValueNoPercent {

        #[inline]
        pub const fn zero() -> Self {
            Self { inner: PixelValue::zero() }
        }

        /// Same as `PixelValueNoPercent::px()`, but only accepts whole numbers,
        /// since using `f32` in const fn is not yet stabilized.
        #[inline]
        pub const fn const_px(value: isize) -> Self {
            Self { inner: PixelValue::const_px(value) }
        }

        /// Same as `PixelValueNoPercent::em()`, but only accepts whole numbers,
        /// since using `f32` in const fn is not yet stabilized.
        #[inline]
        pub const fn const_em(value: isize) -> Self {
            Self { inner: PixelValue::const_em(value) }
        }

        /// Same as `PixelValueNoPercent::pt()`, but only accepts whole numbers,
        /// since using `f32` in const fn is not yet stabilized.
        #[inline]
        pub const fn const_pt(value: isize) -> Self {
            Self { inner: PixelValue::const_pt(value) }
        }

        #[inline]
        const fn const_from_metric(metric: SizeMetric, value: isize) -> Self {
            Self { inner: PixelValue::const_from_metric(metric, value) }
        }

        #[inline]
        pub fn px(value: f32) -> Self {
            Self { inner: PixelValue::px(value) }
        }

        #[inline]
        pub fn em(value: f32) -> Self {
            Self { inner: PixelValue::em(value) }
        }

        #[inline]
        pub fn pt(value: f32) -> Self {
            Self { inner: PixelValue::pt(value) }
        }

        #[inline]
        fn from_metric(metric: SizeMetric, value: f32) -> Self {
            Self { inner: PixelValue::from_metric(metric, value) }
        }
    }

    /// Creates `pt`, `px` and `em` constructors for any struct that has a
    /// `PixelValue` as it's self.0 field.
    macro_rules! impl_pixel_value {($struct:ident) => (

        impl $struct {

            #[inline]
            pub const fn zero() -> Self {
                Self { inner: PixelValue::zero() }
            }

            /// Same as `PixelValue::px()`, but only accepts whole numbers,
            /// since using `f32` in const fn is not yet stabilized.
            #[inline]
            pub const fn const_px(value: isize) -> Self {
                Self { inner: PixelValue::const_px(value) }
            }

            /// Same as `PixelValue::em()`, but only accepts whole numbers,
            /// since using `f32` in const fn is not yet stabilized.
            #[inline]
            pub const fn const_em(value: isize) -> Self {
                Self { inner: PixelValue::const_em(value) }
            }

            /// Same as `PixelValue::pt()`, but only accepts whole numbers,
            /// since using `f32` in const fn is not yet stabilized.
            #[inline]
            pub const fn const_pt(value: isize) -> Self {
                Self { inner: PixelValue::const_pt(value) }
            }

            /// Same as `PixelValue::pt()`, but only accepts whole numbers,
            /// since using `f32` in const fn is not yet stabilized.
            #[inline]
            pub const fn const_percent(value: isize) -> Self {
                Self { inner: PixelValue::const_percent(value) }
            }

            #[inline]
            pub const fn const_from_metric(metric: SizeMetric, value: isize) -> Self {
                Self { inner: PixelValue::const_from_metric(metric, value) }
            }

            #[inline]
            pub fn px(value: f32) -> Self {
                Self { inner: PixelValue::px(value) }
            }

            #[inline]
            pub fn em(value: f32) -> Self {
                Self { inner: PixelValue::em(value) }
            }

            #[inline]
            pub fn pt(value: f32) -> Self {
                Self { inner: PixelValue::pt(value) }
            }

            #[inline]
            pub fn percent(value: f32) -> Self {
                Self { inner: PixelValue::percent(value) }
            }

            #[inline]
            pub fn from_metric(metric: SizeMetric, value: f32) -> Self {
                Self { inner: PixelValue::from_metric(metric, value) }
            }
        }
    )}

    impl_pixel_value!(StyleBorderTopLeftRadius);
    impl_pixel_value!(StyleBorderBottomLeftRadius);
    impl_pixel_value!(StyleBorderTopRightRadius);
    impl_pixel_value!(StyleBorderBottomRightRadius);
    impl_pixel_value!(LayoutBorderTopWidth);
    impl_pixel_value!(LayoutBorderLeftWidth);
    impl_pixel_value!(LayoutBorderRightWidth);
    impl_pixel_value!(LayoutBorderBottomWidth);
    impl_pixel_value!(LayoutWidth);
    impl_pixel_value!(LayoutHeight);
    impl_pixel_value!(LayoutMinHeight);
    impl_pixel_value!(LayoutMinWidth);
    impl_pixel_value!(LayoutMaxWidth);
    impl_pixel_value!(LayoutMaxHeight);
    impl_pixel_value!(LayoutTop);
    impl_pixel_value!(LayoutBottom);
    impl_pixel_value!(LayoutRight);
    impl_pixel_value!(LayoutLeft);
    impl_pixel_value!(LayoutPaddingTop);
    impl_pixel_value!(LayoutPaddingBottom);
    impl_pixel_value!(LayoutPaddingRight);
    impl_pixel_value!(LayoutPaddingLeft);
    impl_pixel_value!(LayoutMarginTop);
    impl_pixel_value!(LayoutMarginBottom);
    impl_pixel_value!(LayoutMarginRight);
    impl_pixel_value!(LayoutMarginLeft);
    impl_pixel_value!(StyleLetterSpacing);
    impl_pixel_value!(StyleWordSpacing);
    impl_pixel_value!(StyleFontSize);

    macro_rules! impl_float_value {($struct:ident) => (
        impl $struct {
            /// Same as `FloatValue::new()`, but only accepts whole numbers,
            /// since using `f32` in const fn is not yet stabilized.
            pub const fn const_new(value: isize)  -> Self {
                Self { inner: FloatValue::const_new(value) }
            }

            pub fn new(value: f32) -> Self {
                Self { inner: FloatValue::new(value) }
            }

            pub fn get(&self) -> f32 {
                self.inner.get()
            }
        }

        impl From<f32> for $struct {
            fn from(val: f32) -> Self {
                Self { inner: FloatValue::from(val) }
            }
        }
    )}

    impl_float_value!(LayoutFlexGrow);
    impl_float_value!(LayoutFlexShrink);
    impl_float_value!(StyleOpacity);
    use crate::str::String;
    /// `CssRuleBlock` struct
    
#[doc(inline)] pub use crate::dll::AzCssRuleBlock as CssRuleBlock;
    /// `CssDeclaration` struct
    
#[doc(inline)] pub use crate::dll::AzCssDeclaration as CssDeclaration;
    /// `DynamicCssProperty` struct
    
#[doc(inline)] pub use crate::dll::AzDynamicCssProperty as DynamicCssProperty;
    /// `CssPath` struct
    
#[doc(inline)] pub use crate::dll::AzCssPath as CssPath;
    /// `CssPathSelector` struct
    
#[doc(inline)] pub use crate::dll::AzCssPathSelector as CssPathSelector;
    /// `NodeTypePath` struct
    
#[doc(inline)] pub use crate::dll::AzNodeTypePath as NodeTypePath;
    /// `CssPathPseudoSelector` struct
    
#[doc(inline)] pub use crate::dll::AzCssPathPseudoSelector as CssPathPseudoSelector;
    /// `CssNthChildSelector` struct
    
#[doc(inline)] pub use crate::dll::AzCssNthChildSelector as CssNthChildSelector;
    /// `CssNthChildPattern` struct
    
#[doc(inline)] pub use crate::dll::AzCssNthChildPattern as CssNthChildPattern;
    /// `Stylesheet` struct
    
#[doc(inline)] pub use crate::dll::AzStylesheet as Stylesheet;
    /// `Css` struct
    
#[doc(inline)] pub use crate::dll::AzCss as Css;
    impl Css {
        /// Returns an empty CSS style
        pub fn empty() -> Self { unsafe { crate::dll::az_css_empty() } }
        /// Returns a CSS style parsed from a `String`
        pub fn from_string(s: String) -> Self { unsafe { crate::dll::az_css_from_string(s) } }
    }

    /// `CssPropertyType` struct
    
#[doc(inline)] pub use crate::dll::AzCssPropertyType as CssPropertyType;
    /// `ColorU` struct
    
#[doc(inline)] pub use crate::dll::AzColorU as ColorU;
    impl ColorU {
        /// Creates a new `ColorU` instance.
        pub fn from_str(string: String) -> Self { unsafe { crate::dll::az_color_u_from_str(string) } }
        /// Calls the `ColorU::to_hash` function.
        pub fn to_hash(&self)  -> crate::str::String { unsafe { crate::dll::az_color_u_to_hash(self) } }
    }

    /// `SizeMetric` struct
    
#[doc(inline)] pub use crate::dll::AzSizeMetric as SizeMetric;
    /// `FloatValue` struct
    
#[doc(inline)] pub use crate::dll::AzFloatValue as FloatValue;
    /// `PixelValue` struct
    
#[doc(inline)] pub use crate::dll::AzPixelValue as PixelValue;
    /// `PixelValueNoPercent` struct
    
#[doc(inline)] pub use crate::dll::AzPixelValueNoPercent as PixelValueNoPercent;
    /// `BoxShadowClipMode` struct
    
#[doc(inline)] pub use crate::dll::AzBoxShadowClipMode as BoxShadowClipMode;
    /// `StyleBoxShadow` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBoxShadow as StyleBoxShadow;
    /// `LayoutAlignContent` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutAlignContent as LayoutAlignContent;
    /// `LayoutAlignItems` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutAlignItems as LayoutAlignItems;
    /// `LayoutBottom` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutBottom as LayoutBottom;
    /// `LayoutBoxSizing` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutBoxSizing as LayoutBoxSizing;
    /// `LayoutFlexDirection` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutFlexDirection as LayoutFlexDirection;
    /// `LayoutDisplay` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutDisplay as LayoutDisplay;
    /// `LayoutFlexGrow` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutFlexGrow as LayoutFlexGrow;
    /// `LayoutFlexShrink` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutFlexShrink as LayoutFlexShrink;
    /// `LayoutFloat` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutFloat as LayoutFloat;
    /// `LayoutHeight` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutHeight as LayoutHeight;
    /// `LayoutJustifyContent` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutJustifyContent as LayoutJustifyContent;
    /// `LayoutLeft` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutLeft as LayoutLeft;
    /// `LayoutMarginBottom` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutMarginBottom as LayoutMarginBottom;
    /// `LayoutMarginLeft` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutMarginLeft as LayoutMarginLeft;
    /// `LayoutMarginRight` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutMarginRight as LayoutMarginRight;
    /// `LayoutMarginTop` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutMarginTop as LayoutMarginTop;
    /// `LayoutMaxHeight` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutMaxHeight as LayoutMaxHeight;
    /// `LayoutMaxWidth` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutMaxWidth as LayoutMaxWidth;
    /// `LayoutMinHeight` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutMinHeight as LayoutMinHeight;
    /// `LayoutMinWidth` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutMinWidth as LayoutMinWidth;
    /// `LayoutPaddingBottom` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutPaddingBottom as LayoutPaddingBottom;
    /// `LayoutPaddingLeft` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutPaddingLeft as LayoutPaddingLeft;
    /// `LayoutPaddingRight` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutPaddingRight as LayoutPaddingRight;
    /// `LayoutPaddingTop` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutPaddingTop as LayoutPaddingTop;
    /// `LayoutPosition` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutPosition as LayoutPosition;
    /// `LayoutRight` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutRight as LayoutRight;
    /// `LayoutTop` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutTop as LayoutTop;
    /// `LayoutWidth` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutWidth as LayoutWidth;
    /// `LayoutFlexWrap` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutFlexWrap as LayoutFlexWrap;
    /// `LayoutOverflow` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutOverflow as LayoutOverflow;
    /// `PercentageValue` struct
    
#[doc(inline)] pub use crate::dll::AzPercentageValue as PercentageValue;
    /// `AngleMetric` struct
    
#[doc(inline)] pub use crate::dll::AzAngleMetric as AngleMetric;
    /// `AngleValue` struct
    
#[doc(inline)] pub use crate::dll::AzAngleValue as AngleValue;
    /// `LinearColorStop` struct
    
#[doc(inline)] pub use crate::dll::AzLinearColorStop as LinearColorStop;
    /// `RadialColorStop` struct
    
#[doc(inline)] pub use crate::dll::AzRadialColorStop as RadialColorStop;
    /// `DirectionCorner` struct
    
#[doc(inline)] pub use crate::dll::AzDirectionCorner as DirectionCorner;
    /// `DirectionCorners` struct
    
#[doc(inline)] pub use crate::dll::AzDirectionCorners as DirectionCorners;
    /// `Direction` struct
    
#[doc(inline)] pub use crate::dll::AzDirection as Direction;
    /// `ExtendMode` struct
    
#[doc(inline)] pub use crate::dll::AzExtendMode as ExtendMode;
    /// `LinearGradient` struct
    
#[doc(inline)] pub use crate::dll::AzLinearGradient as LinearGradient;
    /// `Shape` struct
    
#[doc(inline)] pub use crate::dll::AzShape as Shape;
    /// `RadialGradientSize` struct
    
#[doc(inline)] pub use crate::dll::AzRadialGradientSize as RadialGradientSize;
    /// `RadialGradient` struct
    
#[doc(inline)] pub use crate::dll::AzRadialGradient as RadialGradient;
    /// `ConicGradient` struct
    
#[doc(inline)] pub use crate::dll::AzConicGradient as ConicGradient;
    /// `CssImageId` struct
    
#[doc(inline)] pub use crate::dll::AzCssImageId as CssImageId;
    /// `StyleBackgroundContent` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundContent as StyleBackgroundContent;
    /// `BackgroundPositionHorizontal` struct
    
#[doc(inline)] pub use crate::dll::AzBackgroundPositionHorizontal as BackgroundPositionHorizontal;
    /// `BackgroundPositionVertical` struct
    
#[doc(inline)] pub use crate::dll::AzBackgroundPositionVertical as BackgroundPositionVertical;
    /// `StyleBackgroundPosition` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundPosition as StyleBackgroundPosition;
    /// `StyleBackgroundRepeat` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundRepeat as StyleBackgroundRepeat;
    /// `StyleBackgroundSize` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundSize as StyleBackgroundSize;
    /// `StyleBorderBottomColor` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderBottomColor as StyleBorderBottomColor;
    /// `StyleBorderBottomLeftRadius` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderBottomLeftRadius as StyleBorderBottomLeftRadius;
    /// `StyleBorderBottomRightRadius` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderBottomRightRadius as StyleBorderBottomRightRadius;
    /// `BorderStyle` struct
    
#[doc(inline)] pub use crate::dll::AzBorderStyle as BorderStyle;
    /// `StyleBorderBottomStyle` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderBottomStyle as StyleBorderBottomStyle;
    /// `LayoutBorderBottomWidth` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutBorderBottomWidth as LayoutBorderBottomWidth;
    /// `StyleBorderLeftColor` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderLeftColor as StyleBorderLeftColor;
    /// `StyleBorderLeftStyle` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderLeftStyle as StyleBorderLeftStyle;
    /// `LayoutBorderLeftWidth` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutBorderLeftWidth as LayoutBorderLeftWidth;
    /// `StyleBorderRightColor` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderRightColor as StyleBorderRightColor;
    /// `StyleBorderRightStyle` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderRightStyle as StyleBorderRightStyle;
    /// `LayoutBorderRightWidth` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutBorderRightWidth as LayoutBorderRightWidth;
    /// `StyleBorderTopColor` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderTopColor as StyleBorderTopColor;
    /// `StyleBorderTopLeftRadius` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderTopLeftRadius as StyleBorderTopLeftRadius;
    /// `StyleBorderTopRightRadius` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderTopRightRadius as StyleBorderTopRightRadius;
    /// `StyleBorderTopStyle` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderTopStyle as StyleBorderTopStyle;
    /// `LayoutBorderTopWidth` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutBorderTopWidth as LayoutBorderTopWidth;
    /// `ScrollbarInfo` struct
    
#[doc(inline)] pub use crate::dll::AzScrollbarInfo as ScrollbarInfo;
    /// `ScrollbarStyle` struct
    
#[doc(inline)] pub use crate::dll::AzScrollbarStyle as ScrollbarStyle;
    /// `StyleCursor` struct
    
#[doc(inline)] pub use crate::dll::AzStyleCursor as StyleCursor;
    /// `StyleFontFamily` struct
    
#[doc(inline)] pub use crate::dll::AzStyleFontFamily as StyleFontFamily;
    /// `StyleFontSize` struct
    
#[doc(inline)] pub use crate::dll::AzStyleFontSize as StyleFontSize;
    /// `StyleLetterSpacing` struct
    
#[doc(inline)] pub use crate::dll::AzStyleLetterSpacing as StyleLetterSpacing;
    /// `StyleLineHeight` struct
    
#[doc(inline)] pub use crate::dll::AzStyleLineHeight as StyleLineHeight;
    /// `StyleTabWidth` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTabWidth as StyleTabWidth;
    /// `StyleOpacity` struct
    
#[doc(inline)] pub use crate::dll::AzStyleOpacity as StyleOpacity;
    /// `StyleTransformOrigin` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTransformOrigin as StyleTransformOrigin;
    /// `StylePerspectiveOrigin` struct
    
#[doc(inline)] pub use crate::dll::AzStylePerspectiveOrigin as StylePerspectiveOrigin;
    /// `StyleBackfaceVisibility` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBackfaceVisibility as StyleBackfaceVisibility;
    /// `StyleTransform` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTransform as StyleTransform;
    /// `StyleTransformMatrix2D` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTransformMatrix2D as StyleTransformMatrix2D;
    /// `StyleTransformMatrix3D` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTransformMatrix3D as StyleTransformMatrix3D;
    /// `StyleTransformTranslate2D` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTransformTranslate2D as StyleTransformTranslate2D;
    /// `StyleTransformTranslate3D` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTransformTranslate3D as StyleTransformTranslate3D;
    /// `StyleTransformRotate3D` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTransformRotate3D as StyleTransformRotate3D;
    /// `StyleTransformScale2D` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTransformScale2D as StyleTransformScale2D;
    /// `StyleTransformScale3D` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTransformScale3D as StyleTransformScale3D;
    /// `StyleTransformSkew2D` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTransformSkew2D as StyleTransformSkew2D;
    /// `StyleTextAlignmentHorz` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTextAlignmentHorz as StyleTextAlignmentHorz;
    /// `StyleTextColor` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTextColor as StyleTextColor;
    /// `StyleWordSpacing` struct
    
#[doc(inline)] pub use crate::dll::AzStyleWordSpacing as StyleWordSpacing;
    /// `StyleBoxShadowValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBoxShadowValue as StyleBoxShadowValue;
    /// `LayoutAlignContentValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutAlignContentValue as LayoutAlignContentValue;
    /// `LayoutAlignItemsValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutAlignItemsValue as LayoutAlignItemsValue;
    /// `LayoutBottomValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutBottomValue as LayoutBottomValue;
    /// `LayoutBoxSizingValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutBoxSizingValue as LayoutBoxSizingValue;
    /// `LayoutFlexDirectionValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutFlexDirectionValue as LayoutFlexDirectionValue;
    /// `LayoutDisplayValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutDisplayValue as LayoutDisplayValue;
    /// `LayoutFlexGrowValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutFlexGrowValue as LayoutFlexGrowValue;
    /// `LayoutFlexShrinkValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutFlexShrinkValue as LayoutFlexShrinkValue;
    /// `LayoutFloatValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutFloatValue as LayoutFloatValue;
    /// `LayoutHeightValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutHeightValue as LayoutHeightValue;
    /// `LayoutJustifyContentValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutJustifyContentValue as LayoutJustifyContentValue;
    /// `LayoutLeftValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutLeftValue as LayoutLeftValue;
    /// `LayoutMarginBottomValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutMarginBottomValue as LayoutMarginBottomValue;
    /// `LayoutMarginLeftValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutMarginLeftValue as LayoutMarginLeftValue;
    /// `LayoutMarginRightValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutMarginRightValue as LayoutMarginRightValue;
    /// `LayoutMarginTopValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutMarginTopValue as LayoutMarginTopValue;
    /// `LayoutMaxHeightValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutMaxHeightValue as LayoutMaxHeightValue;
    /// `LayoutMaxWidthValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutMaxWidthValue as LayoutMaxWidthValue;
    /// `LayoutMinHeightValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutMinHeightValue as LayoutMinHeightValue;
    /// `LayoutMinWidthValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutMinWidthValue as LayoutMinWidthValue;
    /// `LayoutPaddingBottomValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutPaddingBottomValue as LayoutPaddingBottomValue;
    /// `LayoutPaddingLeftValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutPaddingLeftValue as LayoutPaddingLeftValue;
    /// `LayoutPaddingRightValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutPaddingRightValue as LayoutPaddingRightValue;
    /// `LayoutPaddingTopValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutPaddingTopValue as LayoutPaddingTopValue;
    /// `LayoutPositionValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutPositionValue as LayoutPositionValue;
    /// `LayoutRightValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutRightValue as LayoutRightValue;
    /// `LayoutTopValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutTopValue as LayoutTopValue;
    /// `LayoutWidthValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutWidthValue as LayoutWidthValue;
    /// `LayoutFlexWrapValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutFlexWrapValue as LayoutFlexWrapValue;
    /// `LayoutOverflowValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutOverflowValue as LayoutOverflowValue;
    /// `ScrollbarStyleValue` struct
    
#[doc(inline)] pub use crate::dll::AzScrollbarStyleValue as ScrollbarStyleValue;
    /// `StyleBackgroundContentVecValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundContentVecValue as StyleBackgroundContentVecValue;
    /// `StyleBackgroundPositionVecValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundPositionVecValue as StyleBackgroundPositionVecValue;
    /// `StyleBackgroundRepeatVecValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundRepeatVecValue as StyleBackgroundRepeatVecValue;
    /// `StyleBackgroundSizeVecValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBackgroundSizeVecValue as StyleBackgroundSizeVecValue;
    /// `StyleBorderBottomColorValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderBottomColorValue as StyleBorderBottomColorValue;
    /// `StyleBorderBottomLeftRadiusValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderBottomLeftRadiusValue as StyleBorderBottomLeftRadiusValue;
    /// `StyleBorderBottomRightRadiusValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderBottomRightRadiusValue as StyleBorderBottomRightRadiusValue;
    /// `StyleBorderBottomStyleValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderBottomStyleValue as StyleBorderBottomStyleValue;
    /// `LayoutBorderBottomWidthValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutBorderBottomWidthValue as LayoutBorderBottomWidthValue;
    /// `StyleBorderLeftColorValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderLeftColorValue as StyleBorderLeftColorValue;
    /// `StyleBorderLeftStyleValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderLeftStyleValue as StyleBorderLeftStyleValue;
    /// `LayoutBorderLeftWidthValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutBorderLeftWidthValue as LayoutBorderLeftWidthValue;
    /// `StyleBorderRightColorValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderRightColorValue as StyleBorderRightColorValue;
    /// `StyleBorderRightStyleValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderRightStyleValue as StyleBorderRightStyleValue;
    /// `LayoutBorderRightWidthValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutBorderRightWidthValue as LayoutBorderRightWidthValue;
    /// `StyleBorderTopColorValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderTopColorValue as StyleBorderTopColorValue;
    /// `StyleBorderTopLeftRadiusValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderTopLeftRadiusValue as StyleBorderTopLeftRadiusValue;
    /// `StyleBorderTopRightRadiusValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderTopRightRadiusValue as StyleBorderTopRightRadiusValue;
    /// `StyleBorderTopStyleValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBorderTopStyleValue as StyleBorderTopStyleValue;
    /// `LayoutBorderTopWidthValue` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutBorderTopWidthValue as LayoutBorderTopWidthValue;
    /// `StyleCursorValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleCursorValue as StyleCursorValue;
    /// `StyleFontFamilyValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleFontFamilyValue as StyleFontFamilyValue;
    /// `StyleFontSizeValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleFontSizeValue as StyleFontSizeValue;
    /// `StyleLetterSpacingValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleLetterSpacingValue as StyleLetterSpacingValue;
    /// `StyleLineHeightValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleLineHeightValue as StyleLineHeightValue;
    /// `StyleTabWidthValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTabWidthValue as StyleTabWidthValue;
    /// `StyleTextAlignmentHorzValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTextAlignmentHorzValue as StyleTextAlignmentHorzValue;
    /// `StyleTextColorValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTextColorValue as StyleTextColorValue;
    /// `StyleWordSpacingValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleWordSpacingValue as StyleWordSpacingValue;
    /// `StyleOpacityValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleOpacityValue as StyleOpacityValue;
    /// `StyleTransformVecValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTransformVecValue as StyleTransformVecValue;
    /// `StyleTransformOriginValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleTransformOriginValue as StyleTransformOriginValue;
    /// `StylePerspectiveOriginValue` struct
    
#[doc(inline)] pub use crate::dll::AzStylePerspectiveOriginValue as StylePerspectiveOriginValue;
    /// `StyleBackfaceVisibilityValue` struct
    
#[doc(inline)] pub use crate::dll::AzStyleBackfaceVisibilityValue as StyleBackfaceVisibilityValue;
    /// Parsed CSS key-value pair
    
#[doc(inline)] pub use crate::dll::AzCssProperty as CssProperty;
}

pub mod style {
    #![allow(dead_code, unused_imports)]
    //! DOM to CSS cascading and styling module
    use crate::dll::*;
    use core::ffi::c_void;
    use crate::dom::Dom;
    use crate::css::Css;
    /// `Node` struct
    
#[doc(inline)] pub use crate::dll::AzNode as Node;
    /// `CascadeInfo` struct
    
#[doc(inline)] pub use crate::dll::AzCascadeInfo as CascadeInfo;
    /// `CssPropertySource` struct
    
#[doc(inline)] pub use crate::dll::AzCssPropertySource as CssPropertySource;
    /// `StyledNodeState` struct
    
#[doc(inline)] pub use crate::dll::AzStyledNodeState as StyledNodeState;
    /// `StyledNode` struct
    
#[doc(inline)] pub use crate::dll::AzStyledNode as StyledNode;
    /// `TagId` struct
    
#[doc(inline)] pub use crate::dll::AzTagId as TagId;
    /// `TagIdToNodeIdMapping` struct
    
#[doc(inline)] pub use crate::dll::AzTagIdToNodeIdMapping as TagIdToNodeIdMapping;
    /// `ParentWithNodeDepth` struct
    
#[doc(inline)] pub use crate::dll::AzParentWithNodeDepth as ParentWithNodeDepth;
    /// `CssPropertyCache` struct
    
#[doc(inline)] pub use crate::dll::AzCssPropertyCache as CssPropertyCache;
    impl Clone for CssPropertyCache { fn clone(&self) -> Self { unsafe { crate::dll::az_css_property_cache_deep_copy(self) } } }
    impl Drop for CssPropertyCache { fn drop(&mut self) { unsafe { crate::dll::az_css_property_cache_delete(self) } } }
    /// `StyledDom` struct
    
#[doc(inline)] pub use crate::dll::AzStyledDom as StyledDom;
    impl StyledDom {
        /// Styles a `Dom` with the given `Css`, returning the `StyledDom` - complexity `O(count(dom_nodes) * count(css_blocks))`: make sure that the `Dom` and the `Css` are as small as possible, use inline CSS if the performance isn't good enough
        pub fn new(dom: Dom, css: Css) -> Self { unsafe { crate::dll::az_styled_dom_new(dom, css) } }
        /// Appends an already styled list of DOM nodes to the current `dom.root` - complexity `O(count(dom.dom_nodes))`
        pub fn append(&mut self, dom: StyledDom)  { unsafe { crate::dll::az_styled_dom_append(self, dom) } }
        /// Returns the number of nodes in the styled DOM
        pub fn node_count(&self)  -> usize { unsafe { crate::dll::az_styled_dom_node_count(self) } }
    }

}

pub mod dom {
    #![allow(dead_code, unused_imports)]
    //! `Dom` construction and configuration
    use crate::dll::*;
    use core::ffi::c_void;
    use crate::option::OptionImageMask;
    use crate::option::OptionTabIndex;
    use crate::option::OptionRefAny;
    use crate::callbacks::IFrameCallback;
    use crate::callbacks::IFrameCallbackType;
    use crate::callbacks::GlCallback;
    use crate::callbacks::GlCallbackType;
    use crate::callbacks::RefAny;
    use crate::resources::ImageId;
    use crate::resources::FontId;
    use crate::vec::DomVec;
    use crate::vec::IdOrClassVec;
    use crate::vec::CallbackDataVec;
    use crate::vec::NodeDataInlineCssPropertyVec;
    use crate::css::Css;
    use crate::style::StyledDom;

    impl Dom {

        /// Creates an empty DOM with a give `NodeType`. Note: This is a `const fn` and
        /// doesn't allocate, it only allocates once you add at least one child node.
        #[inline]
        pub const fn new(node_type: NodeType) -> Self {
            const DEFAULT_VEC: DomVec = DomVec::from_const_slice(&[]);
            Self {
                root: NodeData::new(node_type),
                children: DEFAULT_VEC,
                estimated_total_children: 0,
            }
        }

        #[inline(always)]
        pub const fn div() -> Self { Self::new(NodeType::Div) }
        #[inline(always)]
        pub const fn body() -> Self { Self::new(NodeType::Body) }
        #[inline(always)]
        pub const fn br() -> Self { Self::new(NodeType::Br) }
        #[inline(always)]
        pub fn label<S: Into<AzString>>(value: S) -> Self { Self::new(NodeType::Label(value.into())) }
        #[inline(always)]
        pub const fn image(image: ImageId) -> Self { Self::new(NodeType::Image(image)) }
        #[inline(always)]
        #[cfg(feature = "opengl")]
        pub fn gl_texture(data: RefAny, callback: GlCallbackType) -> Self { Self::new(NodeType::GlTexture(GlTextureNode { callback: GlCallback { cb: callback }, data })) }
        #[inline(always)]
        pub fn iframe(data: RefAny, callback: IFrameCallbackType) -> Self { Self::new(NodeType::IFrame(IFrameNode { callback: IFrameCallback { cb: callback }, data })) }
        /// Shorthand for `Dom::default()`.
        #[inline(always)]
        pub const fn const_default() -> Self { Self::div() }

        #[inline(always)]
        pub fn with_dataset(mut self, data: RefAny) -> Self { self.set_dataset(data); self }
        #[inline(always)]
        pub fn with_ids_and_classes(mut self, ids: IdOrClassVec) -> Self { self.set_ids_and_classes(ids); self }
        #[inline(always)]
        pub fn with_inline_css_props(mut self, properties: NodeDataInlineCssPropertyVec) -> Self { self.set_inline_css_props(properties); self }
        #[inline(always)]
        pub fn with_callbacks(mut self, callbacks: CallbackDataVec) -> Self { self.set_callbacks(callbacks); self }
        #[inline(always)]
        pub fn with_children(mut self, children: DomVec) -> Self { self.set_children(children); self }
        #[inline(always)]
        pub fn with_clip_mask(mut self, clip_mask: OptionImageMask) -> Self { self.set_clip_mask(clip_mask); self }
        #[inline(always)]
        pub fn with_tab_index(mut self, tab_index: OptionTabIndex) -> Self { self.set_tab_index(tab_index); self }

        #[inline(always)]
        pub fn set_dataset(&mut self, data: RefAny) { self.root.set_dataset(OptionRefAny::Some(data)); }
        #[inline(always)]
        pub fn set_ids_and_classes(&mut self, ids: IdOrClassVec) { self.root.set_ids_and_classes(ids); }
        #[inline(always)]
        pub fn set_inline_css_props(&mut self, properties: NodeDataInlineCssPropertyVec) { self.root.set_inline_css_props(properties); }
        #[inline(always)]
        pub fn set_callbacks(&mut self, callbacks: CallbackDataVec) { self.root.set_callbacks(callbacks); }
        #[inline(always)]
        pub fn set_children(&mut self, children: DomVec) {
            self.estimated_total_children = 0;
            for c in children.iter() {
                self.estimated_total_children += c.estimated_total_children + 1;
            }
            self.children = children;
        }
        #[inline(always)]
        pub fn set_clip_mask(&mut self, clip_mask: OptionImageMask) { self.root.set_clip_mask(clip_mask); }
        #[inline(always)]
        pub fn set_tab_index(&mut self, tab_index: OptionTabIndex) { self.root.set_tab_index(tab_index); }
        #[inline(always)]
        pub fn style(self, css: Css) -> StyledDom { StyledDom::new(self, css) }
    }

    impl NodeData {

        /// Creates a new `NodeData` instance from a given `NodeType`
        #[inline]
        pub const fn new(node_type: NodeType) -> Self {
            Self {
                node_type,
                dataset: OptionRefAny::None,
                ids_and_classes: IdOrClassVec::from_const_slice(&[]),
                callbacks: CallbackDataVec::from_const_slice(&[]),
                inline_css_props: NodeDataInlineCssPropertyVec::from_const_slice(&[]),
                clip_mask: OptionImageMask::None,
                tab_index: OptionTabIndex::None,
            }
        }

        /// Shorthand for `NodeData::new(NodeType::Body)`.
        #[inline(always)]
        pub const fn body() -> Self {
            Self::new(NodeType::Body)
        }

        /// Shorthand for `NodeData::new(NodeType::Div)`.
        #[inline(always)]
        pub const fn div() -> Self {
            Self::new(NodeType::Div)
        }

        /// Shorthand for `NodeData::new(NodeType::Br)`.
        #[inline(always)]
        pub const fn br() -> Self {
            Self::new(NodeType::Br)
        }

        /// Shorthand for `NodeData::default()`.
        #[inline(always)]
        pub const fn const_default() -> Self {
            Self::div()
        }

        /// Shorthand for `NodeData::new(NodeType::Label(value.into()))`
        #[inline(always)]
        pub fn label<S: Into<AzString>>(value: S) -> Self {
            Self::new(NodeType::Label(value.into()))
        }

        /// Shorthand for `NodeData::new(NodeType::Image(image_id))`
        #[inline(always)]
        pub fn image(image: ImageId) -> Self {
            Self::new(NodeType::Image(image))
        }

        #[inline(always)]
        #[cfg(feature = "opengl")]
        pub fn gl_texture(data: RefAny, callback: GlCallbackType) -> Self {
            Self::new(NodeType::GlTexture(GlTextureNode { callback: GlCallback { cb: callback }, data }))
        }

        #[inline(always)]
        pub fn iframe(data: RefAny, callback: IFrameCallbackType) -> Self {
            Self::new(NodeType::IFrame(IFrameNode { callback: IFrameCallback { cb: callback }, data }))
        }

        // NOTE: Getters are used here in order to allow changing the memory allocator for the NodeData
        // in the future (which is why the fields are all private).

        #[inline(always)]
        pub const fn get_node_type(&self) -> &NodeType { &self.node_type }
        #[inline(always)]
        pub const fn get_dataset(&self) -> &OptionRefAny { &self.dataset }
        #[inline(always)]
        pub const fn get_ids_and_classes(&self) -> &IdOrClassVec { &self.ids_and_classes }
        #[inline(always)]
        pub const fn get_callbacks(&self) -> &CallbackDataVec { &self.callbacks }
        #[inline(always)]
        pub const fn get_inline_css_props(&self) -> &NodeDataInlineCssPropertyVec { &self.inline_css_props }
        #[inline(always)]
        pub const fn get_clip_mask(&self) -> &OptionImageMask { &self.clip_mask }
        #[inline(always)]
        pub const fn get_tab_index(&self) -> OptionTabIndex { self.tab_index }

        #[inline(always)]
        pub fn set_node_type(&mut self, node_type: NodeType) { self.node_type = node_type; }
        #[inline(always)]
        pub fn set_dataset(&mut self, data: OptionRefAny) { self.dataset = data; }
        #[inline(always)]
        pub fn set_ids_and_classes(&mut self, ids_and_classes: IdOrClassVec) { self.ids_and_classes = ids_and_classes; }
        #[inline(always)]
        pub fn set_callbacks(&mut self, callbacks: CallbackDataVec) { self.callbacks = callbacks; }
        #[inline(always)]
        pub fn set_inline_css_props(&mut self, inline_css_props: NodeDataInlineCssPropertyVec) { self.inline_css_props = inline_css_props; }
        #[inline(always)]
        pub fn set_clip_mask(&mut self, clip_mask: OptionImageMask) { self.clip_mask = clip_mask; }
        #[inline(always)]
        pub fn set_tab_index(&mut self, tab_index: OptionTabIndex) { self.tab_index = tab_index; }

        #[inline(always)]
        pub fn with_node_type(self, node_type: NodeType) -> Self { Self { node_type, .. self } }
        #[inline(always)]
        pub fn with_dataset(self, data: OptionRefAny) -> Self { Self { dataset: data, .. self } }
        #[inline(always)]
        pub fn with_ids_and_classes(self, ids_and_classes: IdOrClassVec) -> Self { Self { ids_and_classes, .. self } }
        #[inline(always)]
        pub fn with_callbacks(self, callbacks: CallbackDataVec) -> Self { Self { callbacks, .. self } }
        #[inline(always)]
        pub fn with_inline_css_props(self, inline_css_props: NodeDataInlineCssPropertyVec) -> Self { Self { inline_css_props, .. self } }
        #[inline(always)]
        pub fn with_clip_mask(self, clip_mask: OptionImageMask) -> Self { Self { clip_mask, .. self } }
        #[inline(always)]
        pub fn with_tab_index(self, tab_index: OptionTabIndex) -> Self { Self { tab_index, .. self } }

    }

    impl Default for Dom {
        fn default() -> Self {
            Dom::const_default()
        }
    }

    impl Default for NodeData {
        fn default() -> Self {
            NodeData::const_default()
        }
    }

    impl Default for TabIndex {
        fn default() -> Self {
            TabIndex::Auto
        }
    }

    impl core::iter::FromIterator<Dom> for Dom {
        fn from_iter<I: IntoIterator<Item=Dom>>(iter: I) -> Self {
            use crate::vec::DomVec;
            let mut estimated_total_children = 0;
            let children = iter.into_iter().map(|c| {
                estimated_total_children += c.estimated_total_children + 1;
                c
            }).collect::<DomVec>();

            Dom {
                root: NodeData::div(),
                children,
                estimated_total_children,
            }
        }
    }

    impl core::iter::FromIterator<NodeData> for Dom {
        fn from_iter<I: IntoIterator<Item=NodeData>>(iter: I) -> Self {
            use crate::vec::DomVec;
            let children = iter.into_iter().map(|c| Dom {
                root: c,
                children: DomVec::from_const_slice(&[]),
                estimated_total_children: 0
            }).collect::<DomVec>();
            let estimated_total_children = children.len();

            Dom {
                root: NodeData::div(),
                children: children,
                estimated_total_children,
            }
        }
    }

    impl core::iter::FromIterator<NodeType> for Dom {
        fn from_iter<I: core::iter::IntoIterator<Item=NodeType>>(iter: I) -> Self {
            iter.into_iter().map(|i| {
                let mut nd = NodeData::default();
                nd.node_type = i;
                nd
            }).collect()
        }
    }

    impl From<On> for AzEventFilter {
        fn from(on: On) -> AzEventFilter {
            on.into_event_filter()
        }
    }    /// `IdOrClass` struct
    
#[doc(inline)] pub use crate::dll::AzIdOrClass as IdOrClass;
    /// `NodeDataInlineCssProperty` struct
    
#[doc(inline)] pub use crate::dll::AzNodeDataInlineCssProperty as NodeDataInlineCssProperty;
    /// `Dom` struct
    
#[doc(inline)] pub use crate::dll::AzDom as Dom;
    impl Dom {
        /// Returns the number of nodes in the DOM
        pub fn node_count(&self)  -> usize { unsafe { crate::dll::az_dom_node_count(self) } }
    }

    /// `GlTextureNode` struct
    
#[doc(inline)] pub use crate::dll::AzGlTextureNode as GlTextureNode;
    /// `IFrameNode` struct
    
#[doc(inline)] pub use crate::dll::AzIFrameNode as IFrameNode;
    /// `CallbackData` struct
    
#[doc(inline)] pub use crate::dll::AzCallbackData as CallbackData;
    /// Represents one single DOM node (node type, classes, ids and callbacks are stored here)
    
#[doc(inline)] pub use crate::dll::AzNodeData as NodeData;
    /// List of core DOM node types built-into by `azul`
    
#[doc(inline)] pub use crate::dll::AzNodeType as NodeType;
    /// When to call a callback action - `On::MouseOver`, `On::MouseOut`, etc.
    
#[doc(inline)] pub use crate::dll::AzOn as On;
    impl On {
        /// Converts the `On` shorthand into a `EventFilter`
        pub fn into_event_filter(self)  -> crate::dom::EventFilter { unsafe { crate::dll::az_on_into_event_filter(self) } }
    }

    /// `EventFilter` struct
    
#[doc(inline)] pub use crate::dll::AzEventFilter as EventFilter;
    /// `HoverEventFilter` struct
    
#[doc(inline)] pub use crate::dll::AzHoverEventFilter as HoverEventFilter;
    /// `FocusEventFilter` struct
    
#[doc(inline)] pub use crate::dll::AzFocusEventFilter as FocusEventFilter;
    /// `NotEventFilter` struct
    
#[doc(inline)] pub use crate::dll::AzNotEventFilter as NotEventFilter;
    /// `WindowEventFilter` struct
    
#[doc(inline)] pub use crate::dll::AzWindowEventFilter as WindowEventFilter;
    /// `ComponentEventFilter` struct
    
#[doc(inline)] pub use crate::dll::AzComponentEventFilter as ComponentEventFilter;
    /// `ApplicationEventFilter` struct
    
#[doc(inline)] pub use crate::dll::AzApplicationEventFilter as ApplicationEventFilter;
    /// `TabIndex` struct
    
#[doc(inline)] pub use crate::dll::AzTabIndex as TabIndex;
}

pub mod gl {
    #![allow(dead_code, unused_imports)]
    //! OpenGl helper types (`Texture`, `GlContext`, etc.)
    use crate::dll::*;
    use core::ffi::c_void;
    impl Refstr {
        fn as_str(&self) -> &str { unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(self.ptr, self.len)) } }
    }

    impl From<&str> for Refstr {
        fn from(s: &str) -> Self {
            Self { ptr: s.as_ptr(), len: s.len() }
        }
    }

    impl RefstrVecRef {
        fn as_slice(&self) -> &[Refstr] { unsafe { core::slice::from_raw_parts(self.ptr, self.len) } }
    }

    impl From<&[Refstr]> for RefstrVecRef {
        fn from(s: &[Refstr]) -> Self {
            Self { ptr: s.as_ptr(), len: s.len() }
        }
    }

    impl From<&mut [GLint64]> for GLint64VecRefMut {
        fn from(s: &mut [GLint64]) -> Self {
            Self { ptr: s.as_mut_ptr(), len: s.len() }
        }
    }

    impl GLint64VecRefMut {
        fn as_mut_slice(&mut self) -> &mut [GLint64] { unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) } }
    }

    impl From<&mut [GLfloat]> for GLfloatVecRefMut {
        fn from(s: &mut [GLfloat]) -> Self {
            Self { ptr: s.as_mut_ptr(), len: s.len() }
        }
    }

    impl GLfloatVecRefMut {
        fn as_mut_slice(&mut self) -> &mut [GLfloat] { unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) } }
    }

    impl From<&mut [GLint]> for GLintVecRefMut {
        fn from(s: &mut [GLint]) -> Self {
            Self { ptr: s.as_mut_ptr(), len: s.len() }
        }
    }

    impl GLintVecRefMut {
        fn as_mut_slice(&mut self) -> &mut [GLint] { unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) } }
    }

    impl From<&[GLuint]> for GLuintVecRef {
        fn from(s: &[GLuint]) -> Self {
            Self { ptr: s.as_ptr(), len: s.len() }
        }
    }

    impl GLuintVecRef {
        fn as_slice(&self) -> &[GLuint] { unsafe { core::slice::from_raw_parts(self.ptr, self.len) } }
    }

    impl From<&[GLenum]> for GLenumVecRef {
        fn from(s: &[GLenum]) -> Self {
            Self { ptr: s.as_ptr(), len: s.len() }
        }
    }

    impl GLenumVecRef {
        fn as_slice(&self) -> &[GLenum] { unsafe { core::slice::from_raw_parts(self.ptr, self.len) } }
    }

    impl From<&[u8]> for U8VecRef {
        fn from(s: &[u8]) -> Self {
            Self { ptr: s.as_ptr(), len: s.len() }
        }
    }

    impl U8VecRef {
        fn as_slice(&self) -> &[u8] { unsafe { core::slice::from_raw_parts(self.ptr, self.len) } }
    }

    impl ::core::fmt::Debug for U8VecRef {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            self.as_slice().fmt(f)
        }
    }

    impl Clone for U8VecRef {
        fn clone(&self) -> Self {
            U8VecRef::from(self.as_slice())
        }
    }

    impl PartialOrd for U8VecRef {
        fn partial_cmp(&self, rhs: &Self) -> Option<core::cmp::Ordering> {
            self.as_slice().partial_cmp(rhs.as_slice())
        }
    }

    impl Ord for U8VecRef {
        fn cmp(&self, rhs: &Self) -> core::cmp::Ordering {
            self.as_slice().cmp(rhs.as_slice())
        }
    }

    impl PartialEq for U8VecRef {
        fn eq(&self, rhs: &Self) -> bool {
            self.as_slice().eq(rhs.as_slice())
        }
    }

    impl Eq for U8VecRef { }

    impl core::hash::Hash for U8VecRef {
        fn hash<H>(&self, state: &mut H) where H: core::hash::Hasher {
            self.as_slice().hash(state)
        }
    }

    impl From<&[f32]> for F32VecRef {
        fn from(s: &[f32]) -> Self {
            Self { ptr: s.as_ptr(), len: s.len() }
        }
    }

    impl F32VecRef {
        fn as_slice(&self) -> &[f32] { unsafe { core::slice::from_raw_parts(self.ptr, self.len) } }
    }

    impl From<&[i32]> for I32VecRef {
        fn from(s: &[i32]) -> Self {
            Self { ptr: s.as_ptr(), len: s.len() }
        }
    }

    impl I32VecRef {
        fn as_slice(&self) -> &[i32] { unsafe { core::slice::from_raw_parts(self.ptr, self.len) } }
    }

    impl From<&mut [GLboolean]> for GLbooleanVecRefMut {
        fn from(s: &mut [GLboolean]) -> Self {
            Self { ptr: s.as_mut_ptr(), len: s.len() }
        }
    }

    impl GLbooleanVecRefMut {
        fn as_mut_slice(&mut self) -> &mut [GLboolean] { unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) } }
    }

    impl From<&mut [u8]> for U8VecRefMut {
        fn from(s: &mut [u8]) -> Self {
            Self { ptr: s.as_mut_ptr(), len: s.len() }
        }
    }

    impl U8VecRefMut {
        fn as_mut_slice(&mut self) -> &mut [u8] { unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) } }
    }

    /// Built in primitive types provided by the C language
    #[allow(non_camel_case_types)]
    pub mod ctypes {
        pub enum c_void {}
        pub type c_char = i8;
        pub type c_schar = i8;
        pub type c_uchar = u8;
        pub type c_short = i16;
        pub type c_ushort = u16;
        pub type c_int = i32;
        pub type c_uint = u32;
        pub type c_long = i32;
        pub type c_ulong = u32;
        pub type c_longlong = i64;
        pub type c_ulonglong = u64;
        pub type c_float = f32;
        pub type c_double = f64;
        pub type __int8 = i8;
        pub type __uint8 = u8;
        pub type __int16 = i16;
        pub type __uint16 = u16;
        pub type __int32 = i32;
        pub type __uint32 = u32;
        pub type __int64 = i64;
        pub type __uint64 = u64;
        pub type wchar_t = u16;
    }

    pub use self::ctypes::*;

    pub type GLenum = c_uint;
    pub type GLboolean = c_uchar;
    pub type GLbitfield = c_uint;
    pub type GLvoid = c_void;
    pub type GLbyte = c_char;
    pub type GLshort = c_short;
    pub type GLint = c_int;
    pub type GLclampx = c_int;
    pub type GLubyte = c_uchar;
    pub type GLushort = c_ushort;
    pub type GLuint = c_uint;
    pub type GLsizei = c_int;
    pub type GLfloat = c_float;
    pub type GLclampf = c_float;
    pub type GLdouble = c_double;
    pub type GLclampd = c_double;
    pub type GLeglImageOES = *const c_void;
    pub type GLchar = c_char;
    pub type GLcharARB = c_char;

    #[cfg(target_os = "macos")]
    pub type GLhandleARB = *const c_void;
    #[cfg(not(target_os = "macos"))]
    pub type GLhandleARB = c_uint;

    pub type GLhalfARB = c_ushort;
    pub type GLhalf = c_ushort;

    // Must be 32 bits
    pub type GLfixed = GLint;
    pub type GLintptr = isize;
    pub type GLsizeiptr = isize;
    pub type GLint64 = i64;
    pub type GLuint64 = u64;
    pub type GLintptrARB = isize;
    pub type GLsizeiptrARB = isize;
    pub type GLint64EXT = i64;
    pub type GLuint64EXT = u64;

    pub type GLDEBUGPROC = Option<extern "system" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut c_void)>;
    pub type GLDEBUGPROCARB = Option<extern "system" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut c_void)>;
    pub type GLDEBUGPROCKHR = Option<extern "system" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut c_void)>;

    // Vendor extension types
    pub type GLDEBUGPROCAMD = Option<extern "system" fn(id: GLuint, category: GLenum, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut c_void)>;
    pub type GLhalfNV = c_ushort;
    pub type GLvdpauSurfaceNV = GLintptr;

    pub const ACCUM: GLenum = 0x0100;
    pub const ACCUM_ALPHA_BITS: GLenum = 0x0D5B;
    pub const ACCUM_BLUE_BITS: GLenum = 0x0D5A;
    pub const ACCUM_BUFFER_BIT: GLenum = 0x00000200;
    pub const ACCUM_CLEAR_VALUE: GLenum = 0x0B80;
    pub const ACCUM_GREEN_BITS: GLenum = 0x0D59;
    pub const ACCUM_RED_BITS: GLenum = 0x0D58;
    pub const ACTIVE_ATTRIBUTES: GLenum = 0x8B89;
    pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A;
    pub const ACTIVE_TEXTURE: GLenum = 0x84E0;
    pub const ACTIVE_UNIFORMS: GLenum = 0x8B86;
    pub const ACTIVE_UNIFORM_BLOCKS: GLenum = 0x8A36;
    pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = 0x8A35;
    pub const ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87;
    pub const ADD: GLenum = 0x0104;
    pub const ADD_SIGNED: GLenum = 0x8574;
    pub const ALIASED_LINE_WIDTH_RANGE: GLenum = 0x846E;
    pub const ALIASED_POINT_SIZE_RANGE: GLenum = 0x846D;
    pub const ALL_ATTRIB_BITS: GLenum = 0xFFFFFFFF;
    pub const ALPHA: GLenum = 0x1906;
    pub const ALPHA12: GLenum = 0x803D;
    pub const ALPHA16: GLenum = 0x803E;
    pub const ALPHA16F_EXT: GLenum = 0x881C;
    pub const ALPHA32F_EXT: GLenum = 0x8816;
    pub const ALPHA4: GLenum = 0x803B;
    pub const ALPHA8: GLenum = 0x803C;
    pub const ALPHA8_EXT: GLenum = 0x803C;
    pub const ALPHA_BIAS: GLenum = 0x0D1D;
    pub const ALPHA_BITS: GLenum = 0x0D55;
    pub const ALPHA_INTEGER: GLenum = 0x8D97;
    pub const ALPHA_SCALE: GLenum = 0x0D1C;
    pub const ALPHA_TEST: GLenum = 0x0BC0;
    pub const ALPHA_TEST_FUNC: GLenum = 0x0BC1;
    pub const ALPHA_TEST_REF: GLenum = 0x0BC2;
    pub const ALREADY_SIGNALED: GLenum = 0x911A;
    pub const ALWAYS: GLenum = 0x0207;
    pub const AMBIENT: GLenum = 0x1200;
    pub const AMBIENT_AND_DIFFUSE: GLenum = 0x1602;
    pub const AND: GLenum = 0x1501;
    pub const AND_INVERTED: GLenum = 0x1504;
    pub const AND_REVERSE: GLenum = 0x1502;
    pub const ANY_SAMPLES_PASSED: GLenum = 0x8C2F;
    pub const ANY_SAMPLES_PASSED_CONSERVATIVE: GLenum = 0x8D6A;
    pub const ARRAY_BUFFER: GLenum = 0x8892;
    pub const ARRAY_BUFFER_BINDING: GLenum = 0x8894;
    pub const ATTACHED_SHADERS: GLenum = 0x8B85;
    pub const ATTRIB_STACK_DEPTH: GLenum = 0x0BB0;
    pub const AUTO_NORMAL: GLenum = 0x0D80;
    pub const AUX0: GLenum = 0x0409;
    pub const AUX1: GLenum = 0x040A;
    pub const AUX2: GLenum = 0x040B;
    pub const AUX3: GLenum = 0x040C;
    pub const AUX_BUFFERS: GLenum = 0x0C00;
    pub const BACK: GLenum = 0x0405;
    pub const BACK_LEFT: GLenum = 0x0402;
    pub const BACK_RIGHT: GLenum = 0x0403;
    pub const BGR: GLenum = 0x80E0;
    pub const BGRA: GLenum = 0x80E1;
    pub const BGRA8_EXT: GLenum = 0x93A1;
    pub const BGRA_EXT: GLenum = 0x80E1;
    pub const BGRA_INTEGER: GLenum = 0x8D9B;
    pub const BGR_INTEGER: GLenum = 0x8D9A;
    pub const BITMAP: GLenum = 0x1A00;
    pub const BITMAP_TOKEN: GLenum = 0x0704;
    pub const BLEND: GLenum = 0x0BE2;
    pub const BLEND_ADVANCED_COHERENT_KHR: GLenum = 0x9285;
    pub const BLEND_COLOR: GLenum = 0x8005;
    pub const BLEND_DST: GLenum = 0x0BE0;
    pub const BLEND_DST_ALPHA: GLenum = 0x80CA;
    pub const BLEND_DST_RGB: GLenum = 0x80C8;
    pub const BLEND_EQUATION: GLenum = 0x8009;
    pub const BLEND_EQUATION_ALPHA: GLenum = 0x883D;
    pub const BLEND_EQUATION_RGB: GLenum = 0x8009;
    pub const BLEND_SRC: GLenum = 0x0BE1;
    pub const BLEND_SRC_ALPHA: GLenum = 0x80CB;
    pub const BLEND_SRC_RGB: GLenum = 0x80C9;
    pub const BLUE: GLenum = 0x1905;
    pub const BLUE_BIAS: GLenum = 0x0D1B;
    pub const BLUE_BITS: GLenum = 0x0D54;
    pub const BLUE_INTEGER: GLenum = 0x8D96;
    pub const BLUE_SCALE: GLenum = 0x0D1A;
    pub const BOOL: GLenum = 0x8B56;
    pub const BOOL_VEC2: GLenum = 0x8B57;
    pub const BOOL_VEC3: GLenum = 0x8B58;
    pub const BOOL_VEC4: GLenum = 0x8B59;
    pub const BUFFER: GLenum = 0x82E0;
    pub const BUFFER_ACCESS: GLenum = 0x88BB;
    pub const BUFFER_ACCESS_FLAGS: GLenum = 0x911F;
    pub const BUFFER_KHR: GLenum = 0x82E0;
    pub const BUFFER_MAPPED: GLenum = 0x88BC;
    pub const BUFFER_MAP_LENGTH: GLenum = 0x9120;
    pub const BUFFER_MAP_OFFSET: GLenum = 0x9121;
    pub const BUFFER_MAP_POINTER: GLenum = 0x88BD;
    pub const BUFFER_SIZE: GLenum = 0x8764;
    pub const BUFFER_USAGE: GLenum = 0x8765;
    pub const BYTE: GLenum = 0x1400;
    pub const C3F_V3F: GLenum = 0x2A24;
    pub const C4F_N3F_V3F: GLenum = 0x2A26;
    pub const C4UB_V2F: GLenum = 0x2A22;
    pub const C4UB_V3F: GLenum = 0x2A23;
    pub const CCW: GLenum = 0x0901;
    pub const CLAMP: GLenum = 0x2900;
    pub const CLAMP_FRAGMENT_COLOR: GLenum = 0x891B;
    pub const CLAMP_READ_COLOR: GLenum = 0x891C;
    pub const CLAMP_TO_BORDER: GLenum = 0x812D;
    pub const CLAMP_TO_EDGE: GLenum = 0x812F;
    pub const CLAMP_VERTEX_COLOR: GLenum = 0x891A;
    pub const CLEAR: GLenum = 0x1500;
    pub const CLIENT_ACTIVE_TEXTURE: GLenum = 0x84E1;
    pub const CLIENT_ALL_ATTRIB_BITS: GLenum = 0xFFFFFFFF;
    pub const CLIENT_ATTRIB_STACK_DEPTH: GLenum = 0x0BB1;
    pub const CLIENT_PIXEL_STORE_BIT: GLenum = 0x00000001;
    pub const CLIENT_VERTEX_ARRAY_BIT: GLenum = 0x00000002;
    pub const CLIP_DISTANCE0: GLenum = 0x3000;
    pub const CLIP_DISTANCE1: GLenum = 0x3001;
    pub const CLIP_DISTANCE2: GLenum = 0x3002;
    pub const CLIP_DISTANCE3: GLenum = 0x3003;
    pub const CLIP_DISTANCE4: GLenum = 0x3004;
    pub const CLIP_DISTANCE5: GLenum = 0x3005;
    pub const CLIP_DISTANCE6: GLenum = 0x3006;
    pub const CLIP_DISTANCE7: GLenum = 0x3007;
    pub const CLIP_PLANE0: GLenum = 0x3000;
    pub const CLIP_PLANE1: GLenum = 0x3001;
    pub const CLIP_PLANE2: GLenum = 0x3002;
    pub const CLIP_PLANE3: GLenum = 0x3003;
    pub const CLIP_PLANE4: GLenum = 0x3004;
    pub const CLIP_PLANE5: GLenum = 0x3005;
    pub const COEFF: GLenum = 0x0A00;
    pub const COLOR: GLenum = 0x1800;
    pub const COLORBURN_KHR: GLenum = 0x929A;
    pub const COLORDODGE_KHR: GLenum = 0x9299;
    pub const COLOR_ARRAY: GLenum = 0x8076;
    pub const COLOR_ARRAY_BUFFER_BINDING: GLenum = 0x8898;
    pub const COLOR_ARRAY_POINTER: GLenum = 0x8090;
    pub const COLOR_ARRAY_SIZE: GLenum = 0x8081;
    pub const COLOR_ARRAY_STRIDE: GLenum = 0x8083;
    pub const COLOR_ARRAY_TYPE: GLenum = 0x8082;
    pub const COLOR_ATTACHMENT0: GLenum = 0x8CE0;
    pub const COLOR_ATTACHMENT1: GLenum = 0x8CE1;
    pub const COLOR_ATTACHMENT10: GLenum = 0x8CEA;
    pub const COLOR_ATTACHMENT11: GLenum = 0x8CEB;
    pub const COLOR_ATTACHMENT12: GLenum = 0x8CEC;
    pub const COLOR_ATTACHMENT13: GLenum = 0x8CED;
    pub const COLOR_ATTACHMENT14: GLenum = 0x8CEE;
    pub const COLOR_ATTACHMENT15: GLenum = 0x8CEF;
    pub const COLOR_ATTACHMENT16: GLenum = 0x8CF0;
    pub const COLOR_ATTACHMENT17: GLenum = 0x8CF1;
    pub const COLOR_ATTACHMENT18: GLenum = 0x8CF2;
    pub const COLOR_ATTACHMENT19: GLenum = 0x8CF3;
    pub const COLOR_ATTACHMENT2: GLenum = 0x8CE2;
    pub const COLOR_ATTACHMENT20: GLenum = 0x8CF4;
    pub const COLOR_ATTACHMENT21: GLenum = 0x8CF5;
    pub const COLOR_ATTACHMENT22: GLenum = 0x8CF6;
    pub const COLOR_ATTACHMENT23: GLenum = 0x8CF7;
    pub const COLOR_ATTACHMENT24: GLenum = 0x8CF8;
    pub const COLOR_ATTACHMENT25: GLenum = 0x8CF9;
    pub const COLOR_ATTACHMENT26: GLenum = 0x8CFA;
    pub const COLOR_ATTACHMENT27: GLenum = 0x8CFB;
    pub const COLOR_ATTACHMENT28: GLenum = 0x8CFC;
    pub const COLOR_ATTACHMENT29: GLenum = 0x8CFD;
    pub const COLOR_ATTACHMENT3: GLenum = 0x8CE3;
    pub const COLOR_ATTACHMENT30: GLenum = 0x8CFE;
    pub const COLOR_ATTACHMENT31: GLenum = 0x8CFF;
    pub const COLOR_ATTACHMENT4: GLenum = 0x8CE4;
    pub const COLOR_ATTACHMENT5: GLenum = 0x8CE5;
    pub const COLOR_ATTACHMENT6: GLenum = 0x8CE6;
    pub const COLOR_ATTACHMENT7: GLenum = 0x8CE7;
    pub const COLOR_ATTACHMENT8: GLenum = 0x8CE8;
    pub const COLOR_ATTACHMENT9: GLenum = 0x8CE9;
    pub const COLOR_BUFFER_BIT: GLenum = 0x00004000;
    pub const COLOR_CLEAR_VALUE: GLenum = 0x0C22;
    pub const COLOR_INDEX: GLenum = 0x1900;
    pub const COLOR_INDEXES: GLenum = 0x1603;
    pub const COLOR_LOGIC_OP: GLenum = 0x0BF2;
    pub const COLOR_MATERIAL: GLenum = 0x0B57;
    pub const COLOR_MATERIAL_FACE: GLenum = 0x0B55;
    pub const COLOR_MATERIAL_PARAMETER: GLenum = 0x0B56;
    pub const COLOR_SUM: GLenum = 0x8458;
    pub const COLOR_WRITEMASK: GLenum = 0x0C23;
    pub const COMBINE: GLenum = 0x8570;
    pub const COMBINE_ALPHA: GLenum = 0x8572;
    pub const COMBINE_RGB: GLenum = 0x8571;
    pub const COMPARE_REF_TO_TEXTURE: GLenum = 0x884E;
    pub const COMPARE_R_TO_TEXTURE: GLenum = 0x884E;
    pub const COMPILE: GLenum = 0x1300;
    pub const COMPILE_AND_EXECUTE: GLenum = 0x1301;
    pub const COMPILE_STATUS: GLenum = 0x8B81;
    pub const COMPRESSED_ALPHA: GLenum = 0x84E9;
    pub const COMPRESSED_INTENSITY: GLenum = 0x84EC;
    pub const COMPRESSED_LUMINANCE: GLenum = 0x84EA;
    pub const COMPRESSED_LUMINANCE_ALPHA: GLenum = 0x84EB;
    pub const COMPRESSED_R11_EAC: GLenum = 0x9270;
    pub const COMPRESSED_RED: GLenum = 0x8225;
    pub const COMPRESSED_RED_RGTC1: GLenum = 0x8DBB;
    pub const COMPRESSED_RG: GLenum = 0x8226;
    pub const COMPRESSED_RG11_EAC: GLenum = 0x9272;
    pub const COMPRESSED_RGB: GLenum = 0x84ED;
    pub const COMPRESSED_RGB8_ETC2: GLenum = 0x9274;
    pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9276;
    pub const COMPRESSED_RGBA: GLenum = 0x84EE;
    pub const COMPRESSED_RGBA8_ETC2_EAC: GLenum = 0x9278;
    pub const COMPRESSED_RG_RGTC2: GLenum = 0x8DBD;
    pub const COMPRESSED_SIGNED_R11_EAC: GLenum = 0x9271;
    pub const COMPRESSED_SIGNED_RED_RGTC1: GLenum = 0x8DBC;
    pub const COMPRESSED_SIGNED_RG11_EAC: GLenum = 0x9273;
    pub const COMPRESSED_SIGNED_RG_RGTC2: GLenum = 0x8DBE;
    pub const COMPRESSED_SLUMINANCE: GLenum = 0x8C4A;
    pub const COMPRESSED_SLUMINANCE_ALPHA: GLenum = 0x8C4B;
    pub const COMPRESSED_SRGB: GLenum = 0x8C48;
    pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: GLenum = 0x9279;
    pub const COMPRESSED_SRGB8_ETC2: GLenum = 0x9275;
    pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: GLenum = 0x9277;
    pub const COMPRESSED_SRGB_ALPHA: GLenum = 0x8C49;
    pub const COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3;
    pub const CONDITION_SATISFIED: GLenum = 0x911C;
    pub const CONSTANT: GLenum = 0x8576;
    pub const CONSTANT_ALPHA: GLenum = 0x8003;
    pub const CONSTANT_ATTENUATION: GLenum = 0x1207;
    pub const CONSTANT_COLOR: GLenum = 0x8001;
    pub const CONTEXT_COMPATIBILITY_PROFILE_BIT: GLenum = 0x00000002;
    pub const CONTEXT_CORE_PROFILE_BIT: GLenum = 0x00000001;
    pub const CONTEXT_FLAGS: GLenum = 0x821E;
    pub const CONTEXT_FLAG_DEBUG_BIT: GLenum = 0x00000002;
    pub const CONTEXT_FLAG_DEBUG_BIT_KHR: GLenum = 0x00000002;
    pub const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: GLenum = 0x00000001;
    pub const CONTEXT_PROFILE_MASK: GLenum = 0x9126;
    pub const COORD_REPLACE: GLenum = 0x8862;
    pub const COPY: GLenum = 0x1503;
    pub const COPY_INVERTED: GLenum = 0x150C;
    pub const COPY_PIXEL_TOKEN: GLenum = 0x0706;
    pub const COPY_READ_BUFFER: GLenum = 0x8F36;
    pub const COPY_READ_BUFFER_BINDING: GLenum = 0x8F36;
    pub const COPY_WRITE_BUFFER: GLenum = 0x8F37;
    pub const COPY_WRITE_BUFFER_BINDING: GLenum = 0x8F37;
    pub const CULL_FACE: GLenum = 0x0B44;
    pub const CULL_FACE_MODE: GLenum = 0x0B45;
    pub const CURRENT_BIT: GLenum = 0x00000001;
    pub const CURRENT_COLOR: GLenum = 0x0B00;
    pub const CURRENT_FOG_COORD: GLenum = 0x8453;
    pub const CURRENT_FOG_COORDINATE: GLenum = 0x8453;
    pub const CURRENT_INDEX: GLenum = 0x0B01;
    pub const CURRENT_NORMAL: GLenum = 0x0B02;
    pub const CURRENT_PROGRAM: GLenum = 0x8B8D;
    pub const CURRENT_QUERY: GLenum = 0x8865;
    pub const CURRENT_QUERY_EXT: GLenum = 0x8865;
    pub const CURRENT_RASTER_COLOR: GLenum = 0x0B04;
    pub const CURRENT_RASTER_DISTANCE: GLenum = 0x0B09;
    pub const CURRENT_RASTER_INDEX: GLenum = 0x0B05;
    pub const CURRENT_RASTER_POSITION: GLenum = 0x0B07;
    pub const CURRENT_RASTER_POSITION_VALID: GLenum = 0x0B08;
    pub const CURRENT_RASTER_SECONDARY_COLOR: GLenum = 0x845F;
    pub const CURRENT_RASTER_TEXTURE_COORDS: GLenum = 0x0B06;
    pub const CURRENT_SECONDARY_COLOR: GLenum = 0x8459;
    pub const CURRENT_TEXTURE_COORDS: GLenum = 0x0B03;
    pub const CURRENT_VERTEX_ATTRIB: GLenum = 0x8626;
    pub const CW: GLenum = 0x0900;
    pub const DARKEN_KHR: GLenum = 0x9297;
    pub const DEBUG_CALLBACK_FUNCTION: GLenum = 0x8244;
    pub const DEBUG_CALLBACK_FUNCTION_KHR: GLenum = 0x8244;
    pub const DEBUG_CALLBACK_USER_PARAM: GLenum = 0x8245;
    pub const DEBUG_CALLBACK_USER_PARAM_KHR: GLenum = 0x8245;
    pub const DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826D;
    pub const DEBUG_GROUP_STACK_DEPTH_KHR: GLenum = 0x826D;
    pub const DEBUG_LOGGED_MESSAGES: GLenum = 0x9145;
    pub const DEBUG_LOGGED_MESSAGES_KHR: GLenum = 0x9145;
    pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLenum = 0x8243;
    pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_KHR: GLenum = 0x8243;
    pub const DEBUG_OUTPUT: GLenum = 0x92E0;
    pub const DEBUG_OUTPUT_KHR: GLenum = 0x92E0;
    pub const DEBUG_OUTPUT_SYNCHRONOUS: GLenum = 0x8242;
    pub const DEBUG_OUTPUT_SYNCHRONOUS_KHR: GLenum = 0x8242;
    pub const DEBUG_SEVERITY_HIGH: GLenum = 0x9146;
    pub const DEBUG_SEVERITY_HIGH_KHR: GLenum = 0x9146;
    pub const DEBUG_SEVERITY_LOW: GLenum = 0x9148;
    pub const DEBUG_SEVERITY_LOW_KHR: GLenum = 0x9148;
    pub const DEBUG_SEVERITY_MEDIUM: GLenum = 0x9147;
    pub const DEBUG_SEVERITY_MEDIUM_KHR: GLenum = 0x9147;
    pub const DEBUG_SEVERITY_NOTIFICATION: GLenum = 0x826B;
    pub const DEBUG_SEVERITY_NOTIFICATION_KHR: GLenum = 0x826B;
    pub const DEBUG_SOURCE_API: GLenum = 0x8246;
    pub const DEBUG_SOURCE_API_KHR: GLenum = 0x8246;
    pub const DEBUG_SOURCE_APPLICATION: GLenum = 0x824A;
    pub const DEBUG_SOURCE_APPLICATION_KHR: GLenum = 0x824A;
    pub const DEBUG_SOURCE_OTHER: GLenum = 0x824B;
    pub const DEBUG_SOURCE_OTHER_KHR: GLenum = 0x824B;
    pub const DEBUG_SOURCE_SHADER_COMPILER: GLenum = 0x8248;
    pub const DEBUG_SOURCE_SHADER_COMPILER_KHR: GLenum = 0x8248;
    pub const DEBUG_SOURCE_THIRD_PARTY: GLenum = 0x8249;
    pub const DEBUG_SOURCE_THIRD_PARTY_KHR: GLenum = 0x8249;
    pub const DEBUG_SOURCE_WINDOW_SYSTEM: GLenum = 0x8247;
    pub const DEBUG_SOURCE_WINDOW_SYSTEM_KHR: GLenum = 0x8247;
    pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLenum = 0x824D;
    pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR_KHR: GLenum = 0x824D;
    pub const DEBUG_TYPE_ERROR: GLenum = 0x824C;
    pub const DEBUG_TYPE_ERROR_KHR: GLenum = 0x824C;
    pub const DEBUG_TYPE_MARKER: GLenum = 0x8268;
    pub const DEBUG_TYPE_MARKER_KHR: GLenum = 0x8268;
    pub const DEBUG_TYPE_OTHER: GLenum = 0x8251;
    pub const DEBUG_TYPE_OTHER_KHR: GLenum = 0x8251;
    pub const DEBUG_TYPE_PERFORMANCE: GLenum = 0x8250;
    pub const DEBUG_TYPE_PERFORMANCE_KHR: GLenum = 0x8250;
    pub const DEBUG_TYPE_POP_GROUP: GLenum = 0x826A;
    pub const DEBUG_TYPE_POP_GROUP_KHR: GLenum = 0x826A;
    pub const DEBUG_TYPE_PORTABILITY: GLenum = 0x824F;
    pub const DEBUG_TYPE_PORTABILITY_KHR: GLenum = 0x824F;
    pub const DEBUG_TYPE_PUSH_GROUP: GLenum = 0x8269;
    pub const DEBUG_TYPE_PUSH_GROUP_KHR: GLenum = 0x8269;
    pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLenum = 0x824E;
    pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR_KHR: GLenum = 0x824E;
    pub const DECAL: GLenum = 0x2101;
    pub const DECR: GLenum = 0x1E03;
    pub const DECR_WRAP: GLenum = 0x8508;
    pub const DELETE_STATUS: GLenum = 0x8B80;
    pub const DEPTH: GLenum = 0x1801;
    pub const DEPTH24_STENCIL8: GLenum = 0x88F0;
    pub const DEPTH32F_STENCIL8: GLenum = 0x8CAD;
    pub const DEPTH_ATTACHMENT: GLenum = 0x8D00;
    pub const DEPTH_BIAS: GLenum = 0x0D1F;
    pub const DEPTH_BITS: GLenum = 0x0D56;
    pub const DEPTH_BUFFER_BIT: GLenum = 0x00000100;
    pub const DEPTH_CLAMP: GLenum = 0x864F;
    pub const DEPTH_CLEAR_VALUE: GLenum = 0x0B73;
    pub const DEPTH_COMPONENT: GLenum = 0x1902;
    pub const DEPTH_COMPONENT16: GLenum = 0x81A5;
    pub const DEPTH_COMPONENT24: GLenum = 0x81A6;
    pub const DEPTH_COMPONENT32: GLenum = 0x81A7;
    pub const DEPTH_COMPONENT32F: GLenum = 0x8CAC;
    pub const DEPTH_FUNC: GLenum = 0x0B74;
    pub const DEPTH_RANGE: GLenum = 0x0B70;
    pub const DEPTH_SCALE: GLenum = 0x0D1E;
    pub const DEPTH_STENCIL: GLenum = 0x84F9;
    pub const DEPTH_STENCIL_ATTACHMENT: GLenum = 0x821A;
    pub const DEPTH_TEST: GLenum = 0x0B71;
    pub const DEPTH_TEXTURE_MODE: GLenum = 0x884B;
    pub const DEPTH_WRITEMASK: GLenum = 0x0B72;
    pub const DIFFERENCE_KHR: GLenum = 0x929E;
    pub const DIFFUSE: GLenum = 0x1201;
    pub const DISPLAY_LIST: GLenum = 0x82E7;
    pub const DITHER: GLenum = 0x0BD0;
    pub const DOMAIN: GLenum = 0x0A02;
    pub const DONT_CARE: GLenum = 0x1100;
    pub const DOT3_RGB: GLenum = 0x86AE;
    pub const DOT3_RGBA: GLenum = 0x86AF;
    pub const DOUBLE: GLenum = 0x140A;
    pub const DOUBLEBUFFER: GLenum = 0x0C32;
    pub const DRAW_BUFFER: GLenum = 0x0C01;
    pub const DRAW_BUFFER0: GLenum = 0x8825;
    pub const DRAW_BUFFER1: GLenum = 0x8826;
    pub const DRAW_BUFFER10: GLenum = 0x882F;
    pub const DRAW_BUFFER11: GLenum = 0x8830;
    pub const DRAW_BUFFER12: GLenum = 0x8831;
    pub const DRAW_BUFFER13: GLenum = 0x8832;
    pub const DRAW_BUFFER14: GLenum = 0x8833;
    pub const DRAW_BUFFER15: GLenum = 0x8834;
    pub const DRAW_BUFFER2: GLenum = 0x8827;
    pub const DRAW_BUFFER3: GLenum = 0x8828;
    pub const DRAW_BUFFER4: GLenum = 0x8829;
    pub const DRAW_BUFFER5: GLenum = 0x882A;
    pub const DRAW_BUFFER6: GLenum = 0x882B;
    pub const DRAW_BUFFER7: GLenum = 0x882C;
    pub const DRAW_BUFFER8: GLenum = 0x882D;
    pub const DRAW_BUFFER9: GLenum = 0x882E;
    pub const DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
    pub const DRAW_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
    pub const DRAW_PIXELS_APPLE: GLenum = 0x8A0A;
    pub const DRAW_PIXEL_TOKEN: GLenum = 0x0705;
    pub const DST_ALPHA: GLenum = 0x0304;
    pub const DST_COLOR: GLenum = 0x0306;
    pub const DYNAMIC_COPY: GLenum = 0x88EA;
    pub const DYNAMIC_DRAW: GLenum = 0x88E8;
    pub const DYNAMIC_READ: GLenum = 0x88E9;
    pub const EDGE_FLAG: GLenum = 0x0B43;
    pub const EDGE_FLAG_ARRAY: GLenum = 0x8079;
    pub const EDGE_FLAG_ARRAY_BUFFER_BINDING: GLenum = 0x889B;
    pub const EDGE_FLAG_ARRAY_POINTER: GLenum = 0x8093;
    pub const EDGE_FLAG_ARRAY_STRIDE: GLenum = 0x808C;
    pub const ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
    pub const ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895;
    pub const EMISSION: GLenum = 0x1600;
    pub const ENABLE_BIT: GLenum = 0x00002000;
    pub const EQUAL: GLenum = 0x0202;
    pub const EQUIV: GLenum = 0x1509;
    pub const EVAL_BIT: GLenum = 0x00010000;
    pub const EXCLUSION_KHR: GLenum = 0x92A0;
    pub const EXP: GLenum = 0x0800;
    pub const EXP2: GLenum = 0x0801;
    pub const EXTENSIONS: GLenum = 0x1F03;
    pub const EYE_LINEAR: GLenum = 0x2400;
    pub const EYE_PLANE: GLenum = 0x2502;
    pub const FALSE: GLboolean = 0;
    pub const FASTEST: GLenum = 0x1101;
    pub const FEEDBACK: GLenum = 0x1C01;
    pub const FEEDBACK_BUFFER_POINTER: GLenum = 0x0DF0;
    pub const FEEDBACK_BUFFER_SIZE: GLenum = 0x0DF1;
    pub const FEEDBACK_BUFFER_TYPE: GLenum = 0x0DF2;
    pub const FENCE_APPLE: GLenum = 0x8A0B;
    pub const FILL: GLenum = 0x1B02;
    pub const FIRST_VERTEX_CONVENTION: GLenum = 0x8E4D;
    pub const FIXED: GLenum = 0x140C;
    pub const FIXED_ONLY: GLenum = 0x891D;
    pub const FLAT: GLenum = 0x1D00;
    pub const FLOAT: GLenum = 0x1406;
    pub const FLOAT_32_UNSIGNED_INT_24_8_REV: GLenum = 0x8DAD;
    pub const FLOAT_MAT2: GLenum = 0x8B5A;
    #[allow(non_upper_case_globals)]
    pub const FLOAT_MAT2x3: GLenum = 0x8B65;
    #[allow(non_upper_case_globals)]
    pub const FLOAT_MAT2x4: GLenum = 0x8B66;
    pub const FLOAT_MAT3: GLenum = 0x8B5B;
    #[allow(non_upper_case_globals)]
    pub const FLOAT_MAT3x2: GLenum = 0x8B67;
    #[allow(non_upper_case_globals)]
    pub const FLOAT_MAT3x4: GLenum = 0x8B68;
    pub const FLOAT_MAT4: GLenum = 0x8B5C;
    #[allow(non_upper_case_globals)]
    pub const FLOAT_MAT4x2: GLenum = 0x8B69;
    #[allow(non_upper_case_globals)]
    pub const FLOAT_MAT4x3: GLenum = 0x8B6A;
    pub const FLOAT_VEC2: GLenum = 0x8B50;
    pub const FLOAT_VEC3: GLenum = 0x8B51;
    pub const FLOAT_VEC4: GLenum = 0x8B52;
    pub const FOG: GLenum = 0x0B60;
    pub const FOG_BIT: GLenum = 0x00000080;
    pub const FOG_COLOR: GLenum = 0x0B66;
    pub const FOG_COORD: GLenum = 0x8451;
    pub const FOG_COORDINATE: GLenum = 0x8451;
    pub const FOG_COORDINATE_ARRAY: GLenum = 0x8457;
    pub const FOG_COORDINATE_ARRAY_BUFFER_BINDING: GLenum = 0x889D;
    pub const FOG_COORDINATE_ARRAY_POINTER: GLenum = 0x8456;
    pub const FOG_COORDINATE_ARRAY_STRIDE: GLenum = 0x8455;
    pub const FOG_COORDINATE_ARRAY_TYPE: GLenum = 0x8454;
    pub const FOG_COORDINATE_SOURCE: GLenum = 0x8450;
    pub const FOG_COORD_ARRAY: GLenum = 0x8457;
    pub const FOG_COORD_ARRAY_BUFFER_BINDING: GLenum = 0x889D;
    pub const FOG_COORD_ARRAY_POINTER: GLenum = 0x8456;
    pub const FOG_COORD_ARRAY_STRIDE: GLenum = 0x8455;
    pub const FOG_COORD_ARRAY_TYPE: GLenum = 0x8454;
    pub const FOG_COORD_SRC: GLenum = 0x8450;
    pub const FOG_DENSITY: GLenum = 0x0B62;
    pub const FOG_END: GLenum = 0x0B64;
    pub const FOG_HINT: GLenum = 0x0C54;
    pub const FOG_INDEX: GLenum = 0x0B61;
    pub const FOG_MODE: GLenum = 0x0B65;
    pub const FOG_START: GLenum = 0x0B63;
    pub const FRAGMENT_DEPTH: GLenum = 0x8452;
    pub const FRAGMENT_SHADER: GLenum = 0x8B30;
    pub const FRAGMENT_SHADER_DERIVATIVE_HINT: GLenum = 0x8B8B;
    pub const FRAMEBUFFER: GLenum = 0x8D40;
    pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 0x8215;
    pub const FRAMEBUFFER_ATTACHMENT_ANGLE: GLenum = 0x93A3;
    pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 0x8214;
    pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 0x8210;
    pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 0x8211;
    pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 0x8216;
    pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 0x8213;
    pub const FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = 0x8DA7;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 0x8CD1;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 0x8CD0;
    pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 0x8212;
    pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 0x8217;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 0x8CD3;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 0x8CD4;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 0x8CD2;
    pub const FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
    pub const FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;
    pub const FRAMEBUFFER_DEFAULT: GLenum = 0x8218;
    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 0x8CD6;
    pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: GLenum = 0x8CD9;
    pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: GLenum = 0x8CDB;
    pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = 0x8DA8;
    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 0x8CD7;
    pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 0x8D56;
    pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: GLenum = 0x8CDC;
    pub const FRAMEBUFFER_SRGB: GLenum = 0x8DB9;
    pub const FRAMEBUFFER_UNDEFINED: GLenum = 0x8219;
    pub const FRAMEBUFFER_UNSUPPORTED: GLenum = 0x8CDD;
    pub const FRONT: GLenum = 0x0404;
    pub const FRONT_AND_BACK: GLenum = 0x0408;
    pub const FRONT_FACE: GLenum = 0x0B46;
    pub const FRONT_LEFT: GLenum = 0x0400;
    pub const FRONT_RIGHT: GLenum = 0x0401;
    pub const FUNC_ADD: GLenum = 0x8006;
    pub const FUNC_REVERSE_SUBTRACT: GLenum = 0x800B;
    pub const FUNC_SUBTRACT: GLenum = 0x800A;
    pub const GENERATE_MIPMAP: GLenum = 0x8191;
    pub const GENERATE_MIPMAP_HINT: GLenum = 0x8192;
    pub const GEOMETRY_INPUT_TYPE: GLenum = 0x8917;
    pub const GEOMETRY_OUTPUT_TYPE: GLenum = 0x8918;
    pub const GEOMETRY_SHADER: GLenum = 0x8DD9;
    pub const GEOMETRY_VERTICES_OUT: GLenum = 0x8916;
    pub const GEQUAL: GLenum = 0x0206;
    pub const GPU_DISJOINT_EXT: GLenum = 0x8FBB;
    pub const GREATER: GLenum = 0x0204;
    pub const GREEN: GLenum = 0x1904;
    pub const GREEN_BIAS: GLenum = 0x0D19;
    pub const GREEN_BITS: GLenum = 0x0D53;
    pub const GREEN_INTEGER: GLenum = 0x8D95;
    pub const GREEN_SCALE: GLenum = 0x0D18;
    pub const HALF_FLOAT: GLenum = 0x140B;
    pub const HALF_FLOAT_OES: GLenum = 0x8D61;
    pub const HARDLIGHT_KHR: GLenum = 0x929B;
    pub const HIGH_FLOAT: GLenum = 0x8DF2;
    pub const HIGH_INT: GLenum = 0x8DF5;
    pub const HINT_BIT: GLenum = 0x00008000;
    pub const HSL_COLOR_KHR: GLenum = 0x92AF;
    pub const HSL_HUE_KHR: GLenum = 0x92AD;
    pub const HSL_LUMINOSITY_KHR: GLenum = 0x92B0;
    pub const HSL_SATURATION_KHR: GLenum = 0x92AE;
    pub const IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 0x8B9B;
    pub const IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 0x8B9A;
    pub const INCR: GLenum = 0x1E02;
    pub const INCR_WRAP: GLenum = 0x8507;
    pub const INDEX: GLenum = 0x8222;
    pub const INDEX_ARRAY: GLenum = 0x8077;
    pub const INDEX_ARRAY_BUFFER_BINDING: GLenum = 0x8899;
    pub const INDEX_ARRAY_POINTER: GLenum = 0x8091;
    pub const INDEX_ARRAY_STRIDE: GLenum = 0x8086;
    pub const INDEX_ARRAY_TYPE: GLenum = 0x8085;
    pub const INDEX_BITS: GLenum = 0x0D51;
    pub const INDEX_CLEAR_VALUE: GLenum = 0x0C20;
    pub const INDEX_LOGIC_OP: GLenum = 0x0BF1;
    pub const INDEX_MODE: GLenum = 0x0C30;
    pub const INDEX_OFFSET: GLenum = 0x0D13;
    pub const INDEX_SHIFT: GLenum = 0x0D12;
    pub const INDEX_WRITEMASK: GLenum = 0x0C21;
    pub const INFO_LOG_LENGTH: GLenum = 0x8B84;
    pub const INT: GLenum = 0x1404;
    pub const INTENSITY: GLenum = 0x8049;
    pub const INTENSITY12: GLenum = 0x804C;
    pub const INTENSITY16: GLenum = 0x804D;
    pub const INTENSITY4: GLenum = 0x804A;
    pub const INTENSITY8: GLenum = 0x804B;
    pub const INTERLEAVED_ATTRIBS: GLenum = 0x8C8C;
    pub const INTERPOLATE: GLenum = 0x8575;
    pub const INT_2_10_10_10_REV: GLenum = 0x8D9F;
    pub const INT_SAMPLER_1D: GLenum = 0x8DC9;
    pub const INT_SAMPLER_1D_ARRAY: GLenum = 0x8DCE;
    pub const INT_SAMPLER_2D: GLenum = 0x8DCA;
    pub const INT_SAMPLER_2D_ARRAY: GLenum = 0x8DCF;
    pub const INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9109;
    pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910C;
    pub const INT_SAMPLER_2D_RECT: GLenum = 0x8DCD;
    pub const INT_SAMPLER_3D: GLenum = 0x8DCB;
    pub const INT_SAMPLER_BUFFER: GLenum = 0x8DD0;
    pub const INT_SAMPLER_CUBE: GLenum = 0x8DCC;
    pub const INT_VEC2: GLenum = 0x8B53;
    pub const INT_VEC3: GLenum = 0x8B54;
    pub const INT_VEC4: GLenum = 0x8B55;
    pub const INVALID_ENUM: GLenum = 0x0500;
    pub const INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
    pub const INVALID_INDEX: GLuint = 0xFFFFFFFF;
    pub const INVALID_OPERATION: GLenum = 0x0502;
    pub const INVALID_VALUE: GLenum = 0x0501;
    pub const INVERT: GLenum = 0x150A;
    pub const KEEP: GLenum = 0x1E00;
    pub const LAST_VERTEX_CONVENTION: GLenum = 0x8E4E;
    pub const LEFT: GLenum = 0x0406;
    pub const LEQUAL: GLenum = 0x0203;
    pub const LESS: GLenum = 0x0201;
    pub const LIGHT0: GLenum = 0x4000;
    pub const LIGHT1: GLenum = 0x4001;
    pub const LIGHT2: GLenum = 0x4002;
    pub const LIGHT3: GLenum = 0x4003;
    pub const LIGHT4: GLenum = 0x4004;
    pub const LIGHT5: GLenum = 0x4005;
    pub const LIGHT6: GLenum = 0x4006;
    pub const LIGHT7: GLenum = 0x4007;
    pub const LIGHTEN_KHR: GLenum = 0x9298;
    pub const LIGHTING: GLenum = 0x0B50;
    pub const LIGHTING_BIT: GLenum = 0x00000040;
    pub const LIGHT_MODEL_AMBIENT: GLenum = 0x0B53;
    pub const LIGHT_MODEL_COLOR_CONTROL: GLenum = 0x81F8;
    pub const LIGHT_MODEL_LOCAL_VIEWER: GLenum = 0x0B51;
    pub const LIGHT_MODEL_TWO_SIDE: GLenum = 0x0B52;
    pub const LINE: GLenum = 0x1B01;
    pub const LINEAR: GLenum = 0x2601;
    pub const LINEAR_ATTENUATION: GLenum = 0x1208;
    pub const LINEAR_MIPMAP_LINEAR: GLenum = 0x2703;
    pub const LINEAR_MIPMAP_NEAREST: GLenum = 0x2701;
    pub const LINES: GLenum = 0x0001;
    pub const LINES_ADJACENCY: GLenum = 0x000A;
    pub const LINE_BIT: GLenum = 0x00000004;
    pub const LINE_LOOP: GLenum = 0x0002;
    pub const LINE_RESET_TOKEN: GLenum = 0x0707;
    pub const LINE_SMOOTH: GLenum = 0x0B20;
    pub const LINE_SMOOTH_HINT: GLenum = 0x0C52;
    pub const LINE_STIPPLE: GLenum = 0x0B24;
    pub const LINE_STIPPLE_PATTERN: GLenum = 0x0B25;
    pub const LINE_STIPPLE_REPEAT: GLenum = 0x0B26;
    pub const LINE_STRIP: GLenum = 0x0003;
    pub const LINE_STRIP_ADJACENCY: GLenum = 0x000B;
    pub const LINE_TOKEN: GLenum = 0x0702;
    pub const LINE_WIDTH: GLenum = 0x0B21;
    pub const LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
    pub const LINE_WIDTH_RANGE: GLenum = 0x0B22;
    pub const LINK_STATUS: GLenum = 0x8B82;
    pub const LIST_BASE: GLenum = 0x0B32;
    pub const LIST_BIT: GLenum = 0x00020000;
    pub const LIST_INDEX: GLenum = 0x0B33;
    pub const LIST_MODE: GLenum = 0x0B30;
    pub const LOAD: GLenum = 0x0101;
    pub const LOGIC_OP: GLenum = 0x0BF1;
    pub const LOGIC_OP_MODE: GLenum = 0x0BF0;
    pub const LOWER_LEFT: GLenum = 0x8CA1;
    pub const LOW_FLOAT: GLenum = 0x8DF0;
    pub const LOW_INT: GLenum = 0x8DF3;
    pub const LUMINANCE: GLenum = 0x1909;
    pub const LUMINANCE12: GLenum = 0x8041;
    pub const LUMINANCE12_ALPHA12: GLenum = 0x8047;
    pub const LUMINANCE12_ALPHA4: GLenum = 0x8046;
    pub const LUMINANCE16: GLenum = 0x8042;
    pub const LUMINANCE16F_EXT: GLenum = 0x881E;
    pub const LUMINANCE16_ALPHA16: GLenum = 0x8048;
    pub const LUMINANCE32F_EXT: GLenum = 0x8818;
    pub const LUMINANCE4: GLenum = 0x803F;
    pub const LUMINANCE4_ALPHA4: GLenum = 0x8043;
    pub const LUMINANCE6_ALPHA2: GLenum = 0x8044;
    pub const LUMINANCE8: GLenum = 0x8040;
    pub const LUMINANCE8_ALPHA8: GLenum = 0x8045;
    pub const LUMINANCE8_ALPHA8_EXT: GLenum = 0x8045;
    pub const LUMINANCE8_EXT: GLenum = 0x8040;
    pub const LUMINANCE_ALPHA: GLenum = 0x190A;
    pub const LUMINANCE_ALPHA16F_EXT: GLenum = 0x881F;
    pub const LUMINANCE_ALPHA32F_EXT: GLenum = 0x8819;
    pub const MAJOR_VERSION: GLenum = 0x821B;
    pub const MAP1_COLOR_4: GLenum = 0x0D90;
    pub const MAP1_GRID_DOMAIN: GLenum = 0x0DD0;
    pub const MAP1_GRID_SEGMENTS: GLenum = 0x0DD1;
    pub const MAP1_INDEX: GLenum = 0x0D91;
    pub const MAP1_NORMAL: GLenum = 0x0D92;
    pub const MAP1_TEXTURE_COORD_1: GLenum = 0x0D93;
    pub const MAP1_TEXTURE_COORD_2: GLenum = 0x0D94;
    pub const MAP1_TEXTURE_COORD_3: GLenum = 0x0D95;
    pub const MAP1_TEXTURE_COORD_4: GLenum = 0x0D96;
    pub const MAP1_VERTEX_3: GLenum = 0x0D97;
    pub const MAP1_VERTEX_4: GLenum = 0x0D98;
    pub const MAP2_COLOR_4: GLenum = 0x0DB0;
    pub const MAP2_GRID_DOMAIN: GLenum = 0x0DD2;
    pub const MAP2_GRID_SEGMENTS: GLenum = 0x0DD3;
    pub const MAP2_INDEX: GLenum = 0x0DB1;
    pub const MAP2_NORMAL: GLenum = 0x0DB2;
    pub const MAP2_TEXTURE_COORD_1: GLenum = 0x0DB3;
    pub const MAP2_TEXTURE_COORD_2: GLenum = 0x0DB4;
    pub const MAP2_TEXTURE_COORD_3: GLenum = 0x0DB5;
    pub const MAP2_TEXTURE_COORD_4: GLenum = 0x0DB6;
    pub const MAP2_VERTEX_3: GLenum = 0x0DB7;
    pub const MAP2_VERTEX_4: GLenum = 0x0DB8;
    pub const MAP_COLOR: GLenum = 0x0D10;
    pub const MAP_FLUSH_EXPLICIT_BIT: GLenum = 0x0010;
    pub const MAP_INVALIDATE_BUFFER_BIT: GLenum = 0x0008;
    pub const MAP_INVALIDATE_RANGE_BIT: GLenum = 0x0004;
    pub const MAP_READ_BIT: GLenum = 0x0001;
    pub const MAP_STENCIL: GLenum = 0x0D11;
    pub const MAP_UNSYNCHRONIZED_BIT: GLenum = 0x0020;
    pub const MAP_WRITE_BIT: GLenum = 0x0002;
    pub const MATRIX_MODE: GLenum = 0x0BA0;
    pub const MAX: GLenum = 0x8008;
    pub const MAX_3D_TEXTURE_SIZE: GLenum = 0x8073;
    pub const MAX_ARRAY_TEXTURE_LAYERS: GLenum = 0x88FF;
    pub const MAX_ATTRIB_STACK_DEPTH: GLenum = 0x0D35;
    pub const MAX_CLIENT_ATTRIB_STACK_DEPTH: GLenum = 0x0D3B;
    pub const MAX_CLIP_DISTANCES: GLenum = 0x0D32;
    pub const MAX_CLIP_PLANES: GLenum = 0x0D32;
    pub const MAX_COLOR_ATTACHMENTS: GLenum = 0x8CDF;
    pub const MAX_COLOR_TEXTURE_SAMPLES: GLenum = 0x910E;
    pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8A33;
    pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8A32;
    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4D;
    pub const MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 0x8A2E;
    pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8A31;
    pub const MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 0x851C;
    pub const MAX_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826C;
    pub const MAX_DEBUG_GROUP_STACK_DEPTH_KHR: GLenum = 0x826C;
    pub const MAX_DEBUG_LOGGED_MESSAGES: GLenum = 0x9144;
    pub const MAX_DEBUG_LOGGED_MESSAGES_KHR: GLenum = 0x9144;
    pub const MAX_DEBUG_MESSAGE_LENGTH: GLenum = 0x9143;
    pub const MAX_DEBUG_MESSAGE_LENGTH_KHR: GLenum = 0x9143;
    pub const MAX_DEPTH_TEXTURE_SAMPLES: GLenum = 0x910F;
    pub const MAX_DRAW_BUFFERS: GLenum = 0x8824;
    pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: GLenum = 0x88FC;
    pub const MAX_ELEMENTS_INDICES: GLenum = 0x80E9;
    pub const MAX_ELEMENTS_VERTICES: GLenum = 0x80E8;
    pub const MAX_ELEMENT_INDEX: GLenum = 0x8D6B;
    pub const MAX_EVAL_ORDER: GLenum = 0x0D30;
    pub const MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 0x9125;
    pub const MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 0x8A2D;
    pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8B49;
    pub const MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 0x8DFD;
    pub const MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = 0x9123;
    pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = 0x9124;
    pub const MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = 0x8DE0;
    pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = 0x8C29;
    pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8DE1;
    pub const MAX_GEOMETRY_UNIFORM_BLOCKS: GLenum = 0x8A2C;
    pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8DDF;
    pub const MAX_INTEGER_SAMPLES: GLenum = 0x9110;
    pub const MAX_LABEL_LENGTH: GLenum = 0x82E8;
    pub const MAX_LABEL_LENGTH_KHR: GLenum = 0x82E8;
    pub const MAX_LIGHTS: GLenum = 0x0D31;
    pub const MAX_LIST_NESTING: GLenum = 0x0B31;
    pub const MAX_MODELVIEW_STACK_DEPTH: GLenum = 0x0D36;
    pub const MAX_NAME_STACK_DEPTH: GLenum = 0x0D37;
    pub const MAX_PIXEL_MAP_TABLE: GLenum = 0x0D34;
    pub const MAX_PROGRAM_TEXEL_OFFSET: GLenum = 0x8905;
    pub const MAX_PROJECTION_STACK_DEPTH: GLenum = 0x0D38;
    pub const MAX_RECTANGLE_TEXTURE_SIZE: GLenum = 0x84F8;
    pub const MAX_RECTANGLE_TEXTURE_SIZE_ARB: GLenum = 0x84F8;
    pub const MAX_RENDERBUFFER_SIZE: GLenum = 0x84E8;
    pub const MAX_SAMPLES: GLenum = 0x8D57;
    pub const MAX_SAMPLE_MASK_WORDS: GLenum = 0x8E59;
    pub const MAX_SERVER_WAIT_TIMEOUT: GLenum = 0x9111;
    pub const MAX_SHADER_PIXEL_LOCAL_STORAGE_FAST_SIZE_EXT: GLenum = 0x8F63;
    pub const MAX_SHADER_PIXEL_LOCAL_STORAGE_SIZE_EXT: GLenum = 0x8F67;
    pub const MAX_TEXTURE_BUFFER_SIZE: GLenum = 0x8C2B;
    pub const MAX_TEXTURE_COORDS: GLenum = 0x8871;
    pub const MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872;
    pub const MAX_TEXTURE_LOD_BIAS: GLenum = 0x84FD;
    pub const MAX_TEXTURE_MAX_ANISOTROPY_EXT: GLenum = 0x84FF;
    pub const MAX_TEXTURE_SIZE: GLenum = 0x0D33;
    pub const MAX_TEXTURE_STACK_DEPTH: GLenum = 0x0D39;
    pub const MAX_TEXTURE_UNITS: GLenum = 0x84E2;
    pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 0x8C8A;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 0x8C8B;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 0x8C80;
    pub const MAX_UNIFORM_BLOCK_SIZE: GLenum = 0x8A30;
    pub const MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 0x8A2F;
    pub const MAX_VARYING_COMPONENTS: GLenum = 0x8B4B;
    pub const MAX_VARYING_FLOATS: GLenum = 0x8B4B;
    pub const MAX_VARYING_VECTORS: GLenum = 0x8DFC;
    pub const MAX_VERTEX_ATTRIBS: GLenum = 0x8869;
    pub const MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 0x9122;
    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4C;
    pub const MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 0x8A2B;
    pub const MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8B4A;
    pub const MAX_VERTEX_UNIFORM_VECTORS: GLenum = 0x8DFB;
    pub const MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
    pub const MEDIUM_FLOAT: GLenum = 0x8DF1;
    pub const MEDIUM_INT: GLenum = 0x8DF4;
    pub const MIN: GLenum = 0x8007;
    pub const MINOR_VERSION: GLenum = 0x821C;
    pub const MIN_PROGRAM_TEXEL_OFFSET: GLenum = 0x8904;
    pub const MIRRORED_REPEAT: GLenum = 0x8370;
    pub const MODELVIEW: GLenum = 0x1700;
    pub const MODELVIEW_MATRIX: GLenum = 0x0BA6;
    pub const MODELVIEW_STACK_DEPTH: GLenum = 0x0BA3;
    pub const MODULATE: GLenum = 0x2100;
    pub const MULT: GLenum = 0x0103;
    pub const MULTIPLY_KHR: GLenum = 0x9294;
    pub const MULTISAMPLE: GLenum = 0x809D;
    pub const MULTISAMPLE_BIT: GLenum = 0x20000000;
    pub const N3F_V3F: GLenum = 0x2A25;
    pub const NAME_STACK_DEPTH: GLenum = 0x0D70;
    pub const NAND: GLenum = 0x150E;
    pub const NEAREST: GLenum = 0x2600;
    pub const NEAREST_MIPMAP_LINEAR: GLenum = 0x2702;
    pub const NEAREST_MIPMAP_NEAREST: GLenum = 0x2700;
    pub const NEVER: GLenum = 0x0200;
    pub const NICEST: GLenum = 0x1102;
    pub const NONE: GLenum = 0;
    pub const NOOP: GLenum = 0x1505;
    pub const NOR: GLenum = 0x1508;
    pub const NORMALIZE: GLenum = 0x0BA1;
    pub const NORMAL_ARRAY: GLenum = 0x8075;
    pub const NORMAL_ARRAY_BUFFER_BINDING: GLenum = 0x8897;
    pub const NORMAL_ARRAY_POINTER: GLenum = 0x808F;
    pub const NORMAL_ARRAY_STRIDE: GLenum = 0x807F;
    pub const NORMAL_ARRAY_TYPE: GLenum = 0x807E;
    pub const NORMAL_MAP: GLenum = 0x8511;
    pub const NOTEQUAL: GLenum = 0x0205;
    pub const NO_ERROR: GLenum = 0;
    pub const NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2;
    pub const NUM_EXTENSIONS: GLenum = 0x821D;
    pub const NUM_PROGRAM_BINARY_FORMATS: GLenum = 0x87FE;
    pub const NUM_SAMPLE_COUNTS: GLenum = 0x9380;
    pub const NUM_SHADER_BINARY_FORMATS: GLenum = 0x8DF9;
    pub const OBJECT_LINEAR: GLenum = 0x2401;
    pub const OBJECT_PLANE: GLenum = 0x2501;
    pub const OBJECT_TYPE: GLenum = 0x9112;
    pub const ONE: GLenum = 1;
    pub const ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004;
    pub const ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
    pub const ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
    pub const ONE_MINUS_DST_COLOR: GLenum = 0x0307;
    pub const ONE_MINUS_SRC1_ALPHA: GLenum = 0x88FB;
    pub const ONE_MINUS_SRC1_COLOR: GLenum = 0x88FA;
    pub const ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
    pub const ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
    pub const OPERAND0_ALPHA: GLenum = 0x8598;
    pub const OPERAND0_RGB: GLenum = 0x8590;
    pub const OPERAND1_ALPHA: GLenum = 0x8599;
    pub const OPERAND1_RGB: GLenum = 0x8591;
    pub const OPERAND2_ALPHA: GLenum = 0x859A;
    pub const OPERAND2_RGB: GLenum = 0x8592;
    pub const OR: GLenum = 0x1507;
    pub const ORDER: GLenum = 0x0A01;
    pub const OR_INVERTED: GLenum = 0x150D;
    pub const OR_REVERSE: GLenum = 0x150B;
    pub const OUT_OF_MEMORY: GLenum = 0x0505;
    pub const OVERLAY_KHR: GLenum = 0x9296;
    pub const PACK_ALIGNMENT: GLenum = 0x0D05;
    pub const PACK_IMAGE_HEIGHT: GLenum = 0x806C;
    pub const PACK_LSB_FIRST: GLenum = 0x0D01;
    pub const PACK_ROW_LENGTH: GLenum = 0x0D02;
    pub const PACK_SKIP_IMAGES: GLenum = 0x806B;
    pub const PACK_SKIP_PIXELS: GLenum = 0x0D04;
    pub const PACK_SKIP_ROWS: GLenum = 0x0D03;
    pub const PACK_SWAP_BYTES: GLenum = 0x0D00;
    pub const PASS_THROUGH_TOKEN: GLenum = 0x0700;
    pub const PERSPECTIVE_CORRECTION_HINT: GLenum = 0x0C50;
    pub const PIXEL_MAP_A_TO_A: GLenum = 0x0C79;
    pub const PIXEL_MAP_A_TO_A_SIZE: GLenum = 0x0CB9;
    pub const PIXEL_MAP_B_TO_B: GLenum = 0x0C78;
    pub const PIXEL_MAP_B_TO_B_SIZE: GLenum = 0x0CB8;
    pub const PIXEL_MAP_G_TO_G: GLenum = 0x0C77;
    pub const PIXEL_MAP_G_TO_G_SIZE: GLenum = 0x0CB7;
    pub const PIXEL_MAP_I_TO_A: GLenum = 0x0C75;
    pub const PIXEL_MAP_I_TO_A_SIZE: GLenum = 0x0CB5;
    pub const PIXEL_MAP_I_TO_B: GLenum = 0x0C74;
    pub const PIXEL_MAP_I_TO_B_SIZE: GLenum = 0x0CB4;
    pub const PIXEL_MAP_I_TO_G: GLenum = 0x0C73;
    pub const PIXEL_MAP_I_TO_G_SIZE: GLenum = 0x0CB3;
    pub const PIXEL_MAP_I_TO_I: GLenum = 0x0C70;
    pub const PIXEL_MAP_I_TO_I_SIZE: GLenum = 0x0CB0;
    pub const PIXEL_MAP_I_TO_R: GLenum = 0x0C72;
    pub const PIXEL_MAP_I_TO_R_SIZE: GLenum = 0x0CB2;
    pub const PIXEL_MAP_R_TO_R: GLenum = 0x0C76;
    pub const PIXEL_MAP_R_TO_R_SIZE: GLenum = 0x0CB6;
    pub const PIXEL_MAP_S_TO_S: GLenum = 0x0C71;
    pub const PIXEL_MAP_S_TO_S_SIZE: GLenum = 0x0CB1;
    pub const PIXEL_MODE_BIT: GLenum = 0x00000020;
    pub const PIXEL_PACK_BUFFER: GLenum = 0x88EB;
    pub const PIXEL_PACK_BUFFER_BINDING: GLenum = 0x88ED;
    pub const PIXEL_UNPACK_BUFFER: GLenum = 0x88EC;
    pub const PIXEL_UNPACK_BUFFER_BINDING: GLenum = 0x88EF;
    pub const POINT: GLenum = 0x1B00;
    pub const POINTS: GLenum = 0x0000;
    pub const POINT_BIT: GLenum = 0x00000002;
    pub const POINT_DISTANCE_ATTENUATION: GLenum = 0x8129;
    pub const POINT_FADE_THRESHOLD_SIZE: GLenum = 0x8128;
    pub const POINT_SIZE: GLenum = 0x0B11;
    pub const POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
    pub const POINT_SIZE_MAX: GLenum = 0x8127;
    pub const POINT_SIZE_MIN: GLenum = 0x8126;
    pub const POINT_SIZE_RANGE: GLenum = 0x0B12;
    pub const POINT_SMOOTH: GLenum = 0x0B10;
    pub const POINT_SMOOTH_HINT: GLenum = 0x0C51;
    pub const POINT_SPRITE: GLenum = 0x8861;
    pub const POINT_SPRITE_COORD_ORIGIN: GLenum = 0x8CA0;
    pub const POINT_TOKEN: GLenum = 0x0701;
    pub const POLYGON: GLenum = 0x0009;
    pub const POLYGON_BIT: GLenum = 0x00000008;
    pub const POLYGON_MODE: GLenum = 0x0B40;
    pub const POLYGON_OFFSET_FACTOR: GLenum = 0x8038;
    pub const POLYGON_OFFSET_FILL: GLenum = 0x8037;
    pub const POLYGON_OFFSET_LINE: GLenum = 0x2A02;
    pub const POLYGON_OFFSET_POINT: GLenum = 0x2A01;
    pub const POLYGON_OFFSET_UNITS: GLenum = 0x2A00;
    pub const POLYGON_SMOOTH: GLenum = 0x0B41;
    pub const POLYGON_SMOOTH_HINT: GLenum = 0x0C53;
    pub const POLYGON_STIPPLE: GLenum = 0x0B42;
    pub const POLYGON_STIPPLE_BIT: GLenum = 0x00000010;
    pub const POLYGON_TOKEN: GLenum = 0x0703;
    pub const POSITION: GLenum = 0x1203;
    pub const PREVIOUS: GLenum = 0x8578;
    pub const PRIMARY_COLOR: GLenum = 0x8577;
    pub const PRIMITIVES_GENERATED: GLenum = 0x8C87;
    pub const PRIMITIVE_RESTART: GLenum = 0x8F9D;
    pub const PRIMITIVE_RESTART_FIXED_INDEX: GLenum = 0x8D69;
    pub const PRIMITIVE_RESTART_INDEX: GLenum = 0x8F9E;
    pub const PROGRAM: GLenum = 0x82E2;
    pub const PROGRAM_BINARY_FORMATS: GLenum = 0x87FF;
    pub const PROGRAM_BINARY_LENGTH: GLenum = 0x8741;
    pub const PROGRAM_BINARY_RETRIEVABLE_HINT: GLenum = 0x8257;
    pub const PROGRAM_KHR: GLenum = 0x82E2;
    pub const PROGRAM_PIPELINE: GLenum = 0x82E4;
    pub const PROGRAM_PIPELINE_KHR: GLenum = 0x82E4;
    pub const PROGRAM_POINT_SIZE: GLenum = 0x8642;
    pub const PROJECTION: GLenum = 0x1701;
    pub const PROJECTION_MATRIX: GLenum = 0x0BA7;
    pub const PROJECTION_STACK_DEPTH: GLenum = 0x0BA4;
    pub const PROVOKING_VERTEX: GLenum = 0x8E4F;
    pub const PROXY_TEXTURE_1D: GLenum = 0x8063;
    pub const PROXY_TEXTURE_1D_ARRAY: GLenum = 0x8C19;
    pub const PROXY_TEXTURE_2D: GLenum = 0x8064;
    pub const PROXY_TEXTURE_2D_ARRAY: GLenum = 0x8C1B;
    pub const PROXY_TEXTURE_2D_MULTISAMPLE: GLenum = 0x9101;
    pub const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9103;
    pub const PROXY_TEXTURE_3D: GLenum = 0x8070;
    pub const PROXY_TEXTURE_CUBE_MAP: GLenum = 0x851B;
    pub const PROXY_TEXTURE_RECTANGLE: GLenum = 0x84F7;
    pub const PROXY_TEXTURE_RECTANGLE_ARB: GLenum = 0x84F7;
    pub const Q: GLenum = 0x2003;
    pub const QUADRATIC_ATTENUATION: GLenum = 0x1209;
    pub const QUADS: GLenum = 0x0007;
    pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: GLenum = 0x8E4C;
    pub const QUAD_STRIP: GLenum = 0x0008;
    pub const QUERY: GLenum = 0x82E3;
    pub const QUERY_BY_REGION_NO_WAIT: GLenum = 0x8E16;
    pub const QUERY_BY_REGION_WAIT: GLenum = 0x8E15;
    pub const QUERY_COUNTER_BITS: GLenum = 0x8864;
    pub const QUERY_COUNTER_BITS_EXT: GLenum = 0x8864;
    pub const QUERY_KHR: GLenum = 0x82E3;
    pub const QUERY_NO_WAIT: GLenum = 0x8E14;
    pub const QUERY_RESULT: GLenum = 0x8866;
    pub const QUERY_RESULT_AVAILABLE: GLenum = 0x8867;
    pub const QUERY_RESULT_AVAILABLE_EXT: GLenum = 0x8867;
    pub const QUERY_RESULT_EXT: GLenum = 0x8866;
    pub const QUERY_WAIT: GLenum = 0x8E13;
    pub const R: GLenum = 0x2002;
    pub const R11F_G11F_B10F: GLenum = 0x8C3A;
    pub const R16: GLenum = 0x822A;
    pub const R16F: GLenum = 0x822D;
    pub const R16F_EXT: GLenum = 0x822D;
    pub const R16I: GLenum = 0x8233;
    pub const R16UI: GLenum = 0x8234;
    pub const R16_SNORM: GLenum = 0x8F98;
    pub const R32F: GLenum = 0x822E;
    pub const R32F_EXT: GLenum = 0x822E;
    pub const R32I: GLenum = 0x8235;
    pub const R32UI: GLenum = 0x8236;
    pub const R3_G3_B2: GLenum = 0x2A10;
    pub const R8: GLenum = 0x8229;
    pub const R8I: GLenum = 0x8231;
    pub const R8UI: GLenum = 0x8232;
    pub const R8_EXT: GLenum = 0x8229;
    pub const R8_SNORM: GLenum = 0x8F94;
    pub const RASTERIZER_DISCARD: GLenum = 0x8C89;
    pub const READ_BUFFER: GLenum = 0x0C02;
    pub const READ_FRAMEBUFFER: GLenum = 0x8CA8;
    pub const READ_FRAMEBUFFER_BINDING: GLenum = 0x8CAA;
    pub const READ_ONLY: GLenum = 0x88B8;
    pub const READ_WRITE: GLenum = 0x88BA;
    pub const RED: GLenum = 0x1903;
    pub const RED_BIAS: GLenum = 0x0D15;
    pub const RED_BITS: GLenum = 0x0D52;
    pub const RED_INTEGER: GLenum = 0x8D94;
    pub const RED_SCALE: GLenum = 0x0D14;
    pub const REFLECTION_MAP: GLenum = 0x8512;
    pub const RENDER: GLenum = 0x1C00;
    pub const RENDERBUFFER: GLenum = 0x8D41;
    pub const RENDERBUFFER_ALPHA_SIZE: GLenum = 0x8D53;
    pub const RENDERBUFFER_BINDING: GLenum = 0x8CA7;
    pub const RENDERBUFFER_BLUE_SIZE: GLenum = 0x8D52;
    pub const RENDERBUFFER_DEPTH_SIZE: GLenum = 0x8D54;
    pub const RENDERBUFFER_GREEN_SIZE: GLenum = 0x8D51;
    pub const RENDERBUFFER_HEIGHT: GLenum = 0x8D43;
    pub const RENDERBUFFER_INTERNAL_FORMAT: GLenum = 0x8D44;
    pub const RENDERBUFFER_RED_SIZE: GLenum = 0x8D50;
    pub const RENDERBUFFER_SAMPLES: GLenum = 0x8CAB;
    pub const RENDERBUFFER_STENCIL_SIZE: GLenum = 0x8D55;
    pub const RENDERBUFFER_WIDTH: GLenum = 0x8D42;
    pub const RENDERER: GLenum = 0x1F01;
    pub const RENDER_MODE: GLenum = 0x0C40;
    pub const REPEAT: GLenum = 0x2901;
    pub const REPLACE: GLenum = 0x1E01;
    pub const REQUIRED_TEXTURE_IMAGE_UNITS_OES: GLenum = 0x8D68;
    pub const RESCALE_NORMAL: GLenum = 0x803A;
    pub const RETURN: GLenum = 0x0102;
    pub const RG: GLenum = 0x8227;
    pub const RG16: GLenum = 0x822C;
    pub const RG16F: GLenum = 0x822F;
    pub const RG16F_EXT: GLenum = 0x822F;
    pub const RG16I: GLenum = 0x8239;
    pub const RG16UI: GLenum = 0x823A;
    pub const RG16_SNORM: GLenum = 0x8F99;
    pub const RG32F: GLenum = 0x8230;
    pub const RG32F_EXT: GLenum = 0x8230;
    pub const RG32I: GLenum = 0x823B;
    pub const RG32UI: GLenum = 0x823C;
    pub const RG8: GLenum = 0x822B;
    pub const RG8I: GLenum = 0x8237;
    pub const RG8UI: GLenum = 0x8238;
    pub const RG8_EXT: GLenum = 0x822B;
    pub const RG8_SNORM: GLenum = 0x8F95;
    pub const RGB: GLenum = 0x1907;
    pub const RGB10: GLenum = 0x8052;
    pub const RGB10_A2: GLenum = 0x8059;
    pub const RGB10_A2UI: GLenum = 0x906F;
    pub const RGB10_A2_EXT: GLenum = 0x8059;
    pub const RGB10_EXT: GLenum = 0x8052;
    pub const RGB12: GLenum = 0x8053;
    pub const RGB16: GLenum = 0x8054;
    pub const RGB16F: GLenum = 0x881B;
    pub const RGB16F_EXT: GLenum = 0x881B;
    pub const RGB16I: GLenum = 0x8D89;
    pub const RGB16UI: GLenum = 0x8D77;
    pub const RGB16_SNORM: GLenum = 0x8F9A;
    pub const RGB32F: GLenum = 0x8815;
    pub const RGB32F_EXT: GLenum = 0x8815;
    pub const RGB32I: GLenum = 0x8D83;
    pub const RGB32UI: GLenum = 0x8D71;
    pub const RGB4: GLenum = 0x804F;
    pub const RGB5: GLenum = 0x8050;
    pub const RGB565: GLenum = 0x8D62;
    pub const RGB5_A1: GLenum = 0x8057;
    pub const RGB8: GLenum = 0x8051;
    pub const RGB8I: GLenum = 0x8D8F;
    pub const RGB8UI: GLenum = 0x8D7D;
    pub const RGB8_SNORM: GLenum = 0x8F96;
    pub const RGB9_E5: GLenum = 0x8C3D;
    pub const RGBA: GLenum = 0x1908;
    pub const RGBA12: GLenum = 0x805A;
    pub const RGBA16: GLenum = 0x805B;
    pub const RGBA16F: GLenum = 0x881A;
    pub const RGBA16F_EXT: GLenum = 0x881A;
    pub const RGBA16I: GLenum = 0x8D88;
    pub const RGBA16UI: GLenum = 0x8D76;
    pub const RGBA16_SNORM: GLenum = 0x8F9B;
    pub const RGBA2: GLenum = 0x8055;
    pub const RGBA32F: GLenum = 0x8814;
    pub const RGBA32F_EXT: GLenum = 0x8814;
    pub const RGBA32I: GLenum = 0x8D82;
    pub const RGBA32UI: GLenum = 0x8D70;
    pub const RGBA4: GLenum = 0x8056;
    pub const RGBA8: GLenum = 0x8058;
    pub const RGBA8I: GLenum = 0x8D8E;
    pub const RGBA8UI: GLenum = 0x8D7C;
    pub const RGBA8_SNORM: GLenum = 0x8F97;
    pub const RGBA_INTEGER: GLenum = 0x8D99;
    pub const RGBA_MODE: GLenum = 0x0C31;
    pub const RGB_INTEGER: GLenum = 0x8D98;
    pub const RGB_SCALE: GLenum = 0x8573;
    pub const RG_INTEGER: GLenum = 0x8228;
    pub const RIGHT: GLenum = 0x0407;
    pub const S: GLenum = 0x2000;
    pub const SAMPLER: GLenum = 0x82E6;
    pub const SAMPLER_1D: GLenum = 0x8B5D;
    pub const SAMPLER_1D_ARRAY: GLenum = 0x8DC0;
    pub const SAMPLER_1D_ARRAY_SHADOW: GLenum = 0x8DC3;
    pub const SAMPLER_1D_SHADOW: GLenum = 0x8B61;
    pub const SAMPLER_2D: GLenum = 0x8B5E;
    pub const SAMPLER_2D_ARRAY: GLenum = 0x8DC1;
    pub const SAMPLER_2D_ARRAY_SHADOW: GLenum = 0x8DC4;
    pub const SAMPLER_2D_MULTISAMPLE: GLenum = 0x9108;
    pub const SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910B;
    pub const SAMPLER_2D_RECT: GLenum = 0x8B63;
    pub const SAMPLER_2D_RECT_SHADOW: GLenum = 0x8B64;
    pub const SAMPLER_2D_SHADOW: GLenum = 0x8B62;
    pub const SAMPLER_3D: GLenum = 0x8B5F;
    pub const SAMPLER_BINDING: GLenum = 0x8919;
    pub const SAMPLER_BUFFER: GLenum = 0x8DC2;
    pub const SAMPLER_CUBE: GLenum = 0x8B60;
    pub const SAMPLER_CUBE_SHADOW: GLenum = 0x8DC5;
    pub const SAMPLER_EXTERNAL_OES: GLenum = 0x8D66;
    pub const SAMPLER_KHR: GLenum = 0x82E6;
    pub const SAMPLES: GLenum = 0x80A9;
    pub const SAMPLES_PASSED: GLenum = 0x8914;
    pub const SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
    pub const SAMPLE_ALPHA_TO_ONE: GLenum = 0x809F;
    pub const SAMPLE_BUFFERS: GLenum = 0x80A8;
    pub const SAMPLE_COVERAGE: GLenum = 0x80A0;
    pub const SAMPLE_COVERAGE_INVERT: GLenum = 0x80AB;
    pub const SAMPLE_COVERAGE_VALUE: GLenum = 0x80AA;
    pub const SAMPLE_MASK: GLenum = 0x8E51;
    pub const SAMPLE_MASK_VALUE: GLenum = 0x8E52;
    pub const SAMPLE_POSITION: GLenum = 0x8E50;
    pub const SCISSOR_BIT: GLenum = 0x00080000;
    pub const SCISSOR_BOX: GLenum = 0x0C10;
    pub const SCISSOR_TEST: GLenum = 0x0C11;
    pub const SCREEN_KHR: GLenum = 0x9295;
    pub const SECONDARY_COLOR_ARRAY: GLenum = 0x845E;
    pub const SECONDARY_COLOR_ARRAY_BUFFER_BINDING: GLenum = 0x889C;
    pub const SECONDARY_COLOR_ARRAY_POINTER: GLenum = 0x845D;
    pub const SECONDARY_COLOR_ARRAY_SIZE: GLenum = 0x845A;
    pub const SECONDARY_COLOR_ARRAY_STRIDE: GLenum = 0x845C;
    pub const SECONDARY_COLOR_ARRAY_TYPE: GLenum = 0x845B;
    pub const SELECT: GLenum = 0x1C02;
    pub const SELECTION_BUFFER_POINTER: GLenum = 0x0DF3;
    pub const SELECTION_BUFFER_SIZE: GLenum = 0x0DF4;
    pub const SEPARATE_ATTRIBS: GLenum = 0x8C8D;
    pub const SEPARATE_SPECULAR_COLOR: GLenum = 0x81FA;
    pub const SET: GLenum = 0x150F;
    pub const SHADER: GLenum = 0x82E1;
    pub const SHADER_BINARY_FORMATS: GLenum = 0x8DF8;
    pub const SHADER_COMPILER: GLenum = 0x8DFA;
    pub const SHADER_KHR: GLenum = 0x82E1;
    pub const SHADER_PIXEL_LOCAL_STORAGE_EXT: GLenum = 0x8F64;
    pub const SHADER_SOURCE_LENGTH: GLenum = 0x8B88;
    pub const SHADER_TYPE: GLenum = 0x8B4F;
    pub const SHADE_MODEL: GLenum = 0x0B54;
    pub const SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;
    pub const SHININESS: GLenum = 0x1601;
    pub const SHORT: GLenum = 0x1402;
    pub const SIGNALED: GLenum = 0x9119;
    pub const SIGNED_NORMALIZED: GLenum = 0x8F9C;
    pub const SINGLE_COLOR: GLenum = 0x81F9;
    pub const SLUMINANCE: GLenum = 0x8C46;
    pub const SLUMINANCE8: GLenum = 0x8C47;
    pub const SLUMINANCE8_ALPHA8: GLenum = 0x8C45;
    pub const SLUMINANCE_ALPHA: GLenum = 0x8C44;
    pub const SMOOTH: GLenum = 0x1D01;
    pub const SMOOTH_LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
    pub const SMOOTH_LINE_WIDTH_RANGE: GLenum = 0x0B22;
    pub const SMOOTH_POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
    pub const SMOOTH_POINT_SIZE_RANGE: GLenum = 0x0B12;
    pub const SOFTLIGHT_KHR: GLenum = 0x929C;
    pub const SOURCE0_ALPHA: GLenum = 0x8588;
    pub const SOURCE0_RGB: GLenum = 0x8580;
    pub const SOURCE1_ALPHA: GLenum = 0x8589;
    pub const SOURCE1_RGB: GLenum = 0x8581;
    pub const SOURCE2_ALPHA: GLenum = 0x858A;
    pub const SOURCE2_RGB: GLenum = 0x8582;
    pub const SPECULAR: GLenum = 0x1202;
    pub const SPHERE_MAP: GLenum = 0x2402;
    pub const SPOT_CUTOFF: GLenum = 0x1206;
    pub const SPOT_DIRECTION: GLenum = 0x1204;
    pub const SPOT_EXPONENT: GLenum = 0x1205;
    pub const SRC0_ALPHA: GLenum = 0x8588;
    pub const SRC0_RGB: GLenum = 0x8580;
    pub const SRC1_ALPHA: GLenum = 0x8589;
    pub const SRC1_COLOR: GLenum = 0x88F9;
    pub const SRC1_RGB: GLenum = 0x8581;
    pub const SRC2_ALPHA: GLenum = 0x858A;
    pub const SRC2_RGB: GLenum = 0x8582;
    pub const SRC_ALPHA: GLenum = 0x0302;
    pub const SRC_ALPHA_SATURATE: GLenum = 0x0308;
    pub const SRC_COLOR: GLenum = 0x0300;
    pub const SRGB: GLenum = 0x8C40;
    pub const SRGB8: GLenum = 0x8C41;
    pub const SRGB8_ALPHA8: GLenum = 0x8C43;
    pub const SRGB_ALPHA: GLenum = 0x8C42;
    pub const STACK_OVERFLOW: GLenum = 0x0503;
    pub const STACK_OVERFLOW_KHR: GLenum = 0x0503;
    pub const STACK_UNDERFLOW: GLenum = 0x0504;
    pub const STACK_UNDERFLOW_KHR: GLenum = 0x0504;
    pub const STATIC_COPY: GLenum = 0x88E6;
    pub const STATIC_DRAW: GLenum = 0x88E4;
    pub const STATIC_READ: GLenum = 0x88E5;
    pub const STENCIL: GLenum = 0x1802;
    pub const STENCIL_ATTACHMENT: GLenum = 0x8D20;
    pub const STENCIL_BACK_FAIL: GLenum = 0x8801;
    pub const STENCIL_BACK_FUNC: GLenum = 0x8800;
    pub const STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 0x8802;
    pub const STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 0x8803;
    pub const STENCIL_BACK_REF: GLenum = 0x8CA3;
    pub const STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4;
    pub const STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5;
    pub const STENCIL_BITS: GLenum = 0x0D57;
    pub const STENCIL_BUFFER_BIT: GLenum = 0x00000400;
    pub const STENCIL_CLEAR_VALUE: GLenum = 0x0B91;
    pub const STENCIL_FAIL: GLenum = 0x0B94;
    pub const STENCIL_FUNC: GLenum = 0x0B92;
    pub const STENCIL_INDEX: GLenum = 0x1901;
    pub const STENCIL_INDEX1: GLenum = 0x8D46;
    pub const STENCIL_INDEX16: GLenum = 0x8D49;
    pub const STENCIL_INDEX4: GLenum = 0x8D47;
    pub const STENCIL_INDEX8: GLenum = 0x8D48;
    pub const STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0B95;
    pub const STENCIL_PASS_DEPTH_PASS: GLenum = 0x0B96;
    pub const STENCIL_REF: GLenum = 0x0B97;
    pub const STENCIL_TEST: GLenum = 0x0B90;
    pub const STENCIL_VALUE_MASK: GLenum = 0x0B93;
    pub const STENCIL_WRITEMASK: GLenum = 0x0B98;
    pub const STEREO: GLenum = 0x0C33;
    pub const STORAGE_CACHED_APPLE: GLenum = 0x85BE;
    pub const STORAGE_PRIVATE_APPLE: GLenum = 0x85BD;
    pub const STORAGE_SHARED_APPLE: GLenum = 0x85BF;
    pub const STREAM_COPY: GLenum = 0x88E2;
    pub const STREAM_DRAW: GLenum = 0x88E0;
    pub const STREAM_READ: GLenum = 0x88E1;
    pub const SUBPIXEL_BITS: GLenum = 0x0D50;
    pub const SUBTRACT: GLenum = 0x84E7;
    pub const SYNC_CONDITION: GLenum = 0x9113;
    pub const SYNC_FENCE: GLenum = 0x9116;
    pub const SYNC_FLAGS: GLenum = 0x9115;
    pub const SYNC_FLUSH_COMMANDS_BIT: GLenum = 0x00000001;
    pub const SYNC_GPU_COMMANDS_COMPLETE: GLenum = 0x9117;
    pub const SYNC_STATUS: GLenum = 0x9114;
    pub const T: GLenum = 0x2001;
    pub const T2F_C3F_V3F: GLenum = 0x2A2A;
    pub const T2F_C4F_N3F_V3F: GLenum = 0x2A2C;
    pub const T2F_C4UB_V3F: GLenum = 0x2A29;
    pub const T2F_N3F_V3F: GLenum = 0x2A2B;
    pub const T2F_V3F: GLenum = 0x2A27;
    pub const T4F_C4F_N3F_V4F: GLenum = 0x2A2D;
    pub const T4F_V4F: GLenum = 0x2A28;
    pub const TEXTURE: GLenum = 0x1702;
    pub const TEXTURE0: GLenum = 0x84C0;
    pub const TEXTURE1: GLenum = 0x84C1;
    pub const TEXTURE10: GLenum = 0x84CA;
    pub const TEXTURE11: GLenum = 0x84CB;
    pub const TEXTURE12: GLenum = 0x84CC;
    pub const TEXTURE13: GLenum = 0x84CD;
    pub const TEXTURE14: GLenum = 0x84CE;
    pub const TEXTURE15: GLenum = 0x84CF;
    pub const TEXTURE16: GLenum = 0x84D0;
    pub const TEXTURE17: GLenum = 0x84D1;
    pub const TEXTURE18: GLenum = 0x84D2;
    pub const TEXTURE19: GLenum = 0x84D3;
    pub const TEXTURE2: GLenum = 0x84C2;
    pub const TEXTURE20: GLenum = 0x84D4;
    pub const TEXTURE21: GLenum = 0x84D5;
    pub const TEXTURE22: GLenum = 0x84D6;
    pub const TEXTURE23: GLenum = 0x84D7;
    pub const TEXTURE24: GLenum = 0x84D8;
    pub const TEXTURE25: GLenum = 0x84D9;
    pub const TEXTURE26: GLenum = 0x84DA;
    pub const TEXTURE27: GLenum = 0x84DB;
    pub const TEXTURE28: GLenum = 0x84DC;
    pub const TEXTURE29: GLenum = 0x84DD;
    pub const TEXTURE3: GLenum = 0x84C3;
    pub const TEXTURE30: GLenum = 0x84DE;
    pub const TEXTURE31: GLenum = 0x84DF;
    pub const TEXTURE4: GLenum = 0x84C4;
    pub const TEXTURE5: GLenum = 0x84C5;
    pub const TEXTURE6: GLenum = 0x84C6;
    pub const TEXTURE7: GLenum = 0x84C7;
    pub const TEXTURE8: GLenum = 0x84C8;
    pub const TEXTURE9: GLenum = 0x84C9;
    pub const TEXTURE_1D: GLenum = 0x0DE0;
    pub const TEXTURE_1D_ARRAY: GLenum = 0x8C18;
    pub const TEXTURE_2D: GLenum = 0x0DE1;
    pub const TEXTURE_2D_ARRAY: GLenum = 0x8C1A;
    pub const TEXTURE_2D_MULTISAMPLE: GLenum = 0x9100;
    pub const TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9102;
    pub const TEXTURE_3D: GLenum = 0x806F;
    pub const TEXTURE_ALPHA_SIZE: GLenum = 0x805F;
    pub const TEXTURE_ALPHA_TYPE: GLenum = 0x8C13;
    pub const TEXTURE_BASE_LEVEL: GLenum = 0x813C;
    pub const TEXTURE_BINDING_1D: GLenum = 0x8068;
    pub const TEXTURE_BINDING_1D_ARRAY: GLenum = 0x8C1C;
    pub const TEXTURE_BINDING_2D: GLenum = 0x8069;
    pub const TEXTURE_BINDING_2D_ARRAY: GLenum = 0x8C1D;
    pub const TEXTURE_BINDING_2D_MULTISAMPLE: GLenum = 0x9104;
    pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLenum = 0x9105;
    pub const TEXTURE_BINDING_3D: GLenum = 0x806A;
    pub const TEXTURE_BINDING_BUFFER: GLenum = 0x8C2C;
    pub const TEXTURE_BINDING_CUBE_MAP: GLenum = 0x8514;
    pub const TEXTURE_BINDING_EXTERNAL_OES: GLenum = 0x8D67;
    pub const TEXTURE_BINDING_RECTANGLE: GLenum = 0x84F6;
    pub const TEXTURE_BINDING_RECTANGLE_ARB: GLenum = 0x84F6;
    pub const TEXTURE_BIT: GLenum = 0x00040000;
    pub const TEXTURE_BLUE_SIZE: GLenum = 0x805E;
    pub const TEXTURE_BLUE_TYPE: GLenum = 0x8C12;
    pub const TEXTURE_BORDER: GLenum = 0x1005;
    pub const TEXTURE_BORDER_COLOR: GLenum = 0x1004;
    pub const TEXTURE_BUFFER: GLenum = 0x8C2A;
    pub const TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = 0x8C2D;
    pub const TEXTURE_COMPARE_FUNC: GLenum = 0x884D;
    pub const TEXTURE_COMPARE_MODE: GLenum = 0x884C;
    pub const TEXTURE_COMPONENTS: GLenum = 0x1003;
    pub const TEXTURE_COMPRESSED: GLenum = 0x86A1;
    pub const TEXTURE_COMPRESSED_IMAGE_SIZE: GLenum = 0x86A0;
    pub const TEXTURE_COMPRESSION_HINT: GLenum = 0x84EF;
    pub const TEXTURE_COORD_ARRAY: GLenum = 0x8078;
    pub const TEXTURE_COORD_ARRAY_BUFFER_BINDING: GLenum = 0x889A;
    pub const TEXTURE_COORD_ARRAY_POINTER: GLenum = 0x8092;
    pub const TEXTURE_COORD_ARRAY_SIZE: GLenum = 0x8088;
    pub const TEXTURE_COORD_ARRAY_STRIDE: GLenum = 0x808A;
    pub const TEXTURE_COORD_ARRAY_TYPE: GLenum = 0x8089;
    pub const TEXTURE_CUBE_MAP: GLenum = 0x8513;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 0x8516;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 0x8518;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 0x851A;
    pub const TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 0x8515;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 0x8517;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 0x8519;
    pub const TEXTURE_CUBE_MAP_SEAMLESS: GLenum = 0x884F;
    pub const TEXTURE_DEPTH: GLenum = 0x8071;
    pub const TEXTURE_DEPTH_SIZE: GLenum = 0x884A;
    pub const TEXTURE_DEPTH_TYPE: GLenum = 0x8C16;
    pub const TEXTURE_ENV: GLenum = 0x2300;
    pub const TEXTURE_ENV_COLOR: GLenum = 0x2201;
    pub const TEXTURE_ENV_MODE: GLenum = 0x2200;
    pub const TEXTURE_EXTERNAL_OES: GLenum = 0x8D65;
    pub const TEXTURE_FILTER_CONTROL: GLenum = 0x8500;
    pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9107;
    pub const TEXTURE_GEN_MODE: GLenum = 0x2500;
    pub const TEXTURE_GEN_Q: GLenum = 0x0C63;
    pub const TEXTURE_GEN_R: GLenum = 0x0C62;
    pub const TEXTURE_GEN_S: GLenum = 0x0C60;
    pub const TEXTURE_GEN_T: GLenum = 0x0C61;
    pub const TEXTURE_GREEN_SIZE: GLenum = 0x805D;
    pub const TEXTURE_GREEN_TYPE: GLenum = 0x8C11;
    pub const TEXTURE_HEIGHT: GLenum = 0x1001;
    pub const TEXTURE_IMMUTABLE_FORMAT: GLenum = 0x912F;
    pub const TEXTURE_IMMUTABLE_FORMAT_EXT: GLenum = 0x912F;
    pub const TEXTURE_IMMUTABLE_LEVELS: GLenum = 0x82DF;
    pub const TEXTURE_INTENSITY_SIZE: GLenum = 0x8061;
    pub const TEXTURE_INTENSITY_TYPE: GLenum = 0x8C15;
    pub const TEXTURE_INTERNAL_FORMAT: GLenum = 0x1003;
    pub const TEXTURE_LOD_BIAS: GLenum = 0x8501;
    pub const TEXTURE_LUMINANCE_SIZE: GLenum = 0x8060;
    pub const TEXTURE_LUMINANCE_TYPE: GLenum = 0x8C14;
    pub const TEXTURE_MAG_FILTER: GLenum = 0x2800;
    pub const TEXTURE_MATRIX: GLenum = 0x0BA8;
    pub const TEXTURE_MAX_ANISOTROPY_EXT: GLenum = 0x84FE;
    pub const TEXTURE_MAX_LEVEL: GLenum = 0x813D;
    pub const TEXTURE_MAX_LOD: GLenum = 0x813B;
    pub const TEXTURE_MIN_FILTER: GLenum = 0x2801;
    pub const TEXTURE_MIN_LOD: GLenum = 0x813A;
    pub const TEXTURE_PRIORITY: GLenum = 0x8066;
    pub const TEXTURE_RANGE_LENGTH_APPLE: GLenum = 0x85B7;
    pub const TEXTURE_RANGE_POINTER_APPLE: GLenum = 0x85B8;
    pub const TEXTURE_RECTANGLE: GLenum = 0x84F5;
    pub const TEXTURE_RECTANGLE_ARB: GLenum = 0x84F5;
    pub const TEXTURE_RED_SIZE: GLenum = 0x805C;
    pub const TEXTURE_RED_TYPE: GLenum = 0x8C10;
    pub const TEXTURE_RESIDENT: GLenum = 0x8067;
    pub const TEXTURE_SAMPLES: GLenum = 0x9106;
    pub const TEXTURE_SHARED_SIZE: GLenum = 0x8C3F;
    pub const TEXTURE_STACK_DEPTH: GLenum = 0x0BA5;
    pub const TEXTURE_STENCIL_SIZE: GLenum = 0x88F1;
    pub const TEXTURE_STORAGE_HINT_APPLE: GLenum = 0x85BC;
    pub const TEXTURE_SWIZZLE_A: GLenum = 0x8E45;
    pub const TEXTURE_SWIZZLE_B: GLenum = 0x8E44;
    pub const TEXTURE_SWIZZLE_G: GLenum = 0x8E43;
    pub const TEXTURE_SWIZZLE_R: GLenum = 0x8E42;
    pub const TEXTURE_SWIZZLE_RGBA: GLenum = 0x8E46;
    pub const TEXTURE_USAGE_ANGLE: GLenum = 0x93A2;
    pub const TEXTURE_WIDTH: GLenum = 0x1000;
    pub const TEXTURE_WRAP_R: GLenum = 0x8072;
    pub const TEXTURE_WRAP_S: GLenum = 0x2802;
    pub const TEXTURE_WRAP_T: GLenum = 0x2803;
    pub const TIMEOUT_EXPIRED: GLenum = 0x911B;
    pub const TIMEOUT_IGNORED: GLuint64 = 0xFFFFFFFFFFFFFFFF;
    pub const TIMESTAMP: GLenum = 0x8E28;
    pub const TIMESTAMP_EXT: GLenum = 0x8E28;
    pub const TIME_ELAPSED: GLenum = 0x88BF;
    pub const TIME_ELAPSED_EXT: GLenum = 0x88BF;
    pub const TRANSFORM_BIT: GLenum = 0x00001000;
    pub const TRANSFORM_FEEDBACK: GLenum = 0x8E22;
    pub const TRANSFORM_FEEDBACK_ACTIVE: GLenum = 0x8E24;
    pub const TRANSFORM_FEEDBACK_BINDING: GLenum = 0x8E25;
    pub const TRANSFORM_FEEDBACK_BUFFER: GLenum = 0x8C8E;
    pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 0x8C8F;
    pub const TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 0x8C7F;
    pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 0x8C85;
    pub const TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 0x8C84;
    pub const TRANSFORM_FEEDBACK_PAUSED: GLenum = 0x8E23;
    pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 0x8C88;
    pub const TRANSFORM_FEEDBACK_VARYINGS: GLenum = 0x8C83;
    pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = 0x8C76;
    pub const TRANSPOSE_COLOR_MATRIX: GLenum = 0x84E6;
    pub const TRANSPOSE_MODELVIEW_MATRIX: GLenum = 0x84E3;
    pub const TRANSPOSE_PROJECTION_MATRIX: GLenum = 0x84E4;
    pub const TRANSPOSE_TEXTURE_MATRIX: GLenum = 0x84E5;
    pub const TRIANGLES: GLenum = 0x0004;
    pub const TRIANGLES_ADJACENCY: GLenum = 0x000C;
    pub const TRIANGLE_FAN: GLenum = 0x0006;
    pub const TRIANGLE_STRIP: GLenum = 0x0005;
    pub const TRIANGLE_STRIP_ADJACENCY: GLenum = 0x000D;
    pub const TRUE: GLboolean = 1;
    pub const UNIFORM_ARRAY_STRIDE: GLenum = 0x8A3C;
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 0x8A42;
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 0x8A43;
    pub const UNIFORM_BLOCK_BINDING: GLenum = 0x8A3F;
    pub const UNIFORM_BLOCK_DATA_SIZE: GLenum = 0x8A40;
    pub const UNIFORM_BLOCK_INDEX: GLenum = 0x8A3A;
    pub const UNIFORM_BLOCK_NAME_LENGTH: GLenum = 0x8A41;
    pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x8A46;
    pub const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x8A45;
    pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x8A44;
    pub const UNIFORM_BUFFER: GLenum = 0x8A11;
    pub const UNIFORM_BUFFER_BINDING: GLenum = 0x8A28;
    pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x8A34;
    pub const UNIFORM_BUFFER_SIZE: GLenum = 0x8A2A;
    pub const UNIFORM_BUFFER_START: GLenum = 0x8A29;
    pub const UNIFORM_IS_ROW_MAJOR: GLenum = 0x8A3E;
    pub const UNIFORM_MATRIX_STRIDE: GLenum = 0x8A3D;
    pub const UNIFORM_NAME_LENGTH: GLenum = 0x8A39;
    pub const UNIFORM_OFFSET: GLenum = 0x8A3B;
    pub const UNIFORM_SIZE: GLenum = 0x8A38;
    pub const UNIFORM_TYPE: GLenum = 0x8A37;
    pub const UNPACK_ALIGNMENT: GLenum = 0x0CF5;
    pub const UNPACK_CLIENT_STORAGE_APPLE: GLenum = 0x85B2;
    pub const UNPACK_IMAGE_HEIGHT: GLenum = 0x806E;
    pub const UNPACK_LSB_FIRST: GLenum = 0x0CF1;
    pub const UNPACK_ROW_LENGTH: GLenum = 0x0CF2;
    pub const UNPACK_SKIP_IMAGES: GLenum = 0x806D;
    pub const UNPACK_SKIP_PIXELS: GLenum = 0x0CF4;
    pub const UNPACK_SKIP_ROWS: GLenum = 0x0CF3;
    pub const UNPACK_SWAP_BYTES: GLenum = 0x0CF0;
    pub const UNSIGNALED: GLenum = 0x9118;
    pub const UNSIGNED_BYTE: GLenum = 0x1401;
    pub const UNSIGNED_BYTE_2_3_3_REV: GLenum = 0x8362;
    pub const UNSIGNED_BYTE_3_3_2: GLenum = 0x8032;
    pub const UNSIGNED_INT: GLenum = 0x1405;
    pub const UNSIGNED_INT_10F_11F_11F_REV: GLenum = 0x8C3B;
    pub const UNSIGNED_INT_10_10_10_2: GLenum = 0x8036;
    pub const UNSIGNED_INT_24_8: GLenum = 0x84FA;
    pub const UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368;
    pub const UNSIGNED_INT_5_9_9_9_REV: GLenum = 0x8C3E;
    pub const UNSIGNED_INT_8_8_8_8: GLenum = 0x8035;
    pub const UNSIGNED_INT_8_8_8_8_REV: GLenum = 0x8367;
    pub const UNSIGNED_INT_SAMPLER_1D: GLenum = 0x8DD1;
    pub const UNSIGNED_INT_SAMPLER_1D_ARRAY: GLenum = 0x8DD6;
    pub const UNSIGNED_INT_SAMPLER_2D: GLenum = 0x8DD2;
    pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: GLenum = 0x8DD7;
    pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x910A;
    pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910D;
    pub const UNSIGNED_INT_SAMPLER_2D_RECT: GLenum = 0x8DD5;
    pub const UNSIGNED_INT_SAMPLER_3D: GLenum = 0x8DD3;
    pub const UNSIGNED_INT_SAMPLER_BUFFER: GLenum = 0x8DD8;
    pub const UNSIGNED_INT_SAMPLER_CUBE: GLenum = 0x8DD4;
    pub const UNSIGNED_INT_VEC2: GLenum = 0x8DC6;
    pub const UNSIGNED_INT_VEC3: GLenum = 0x8DC7;
    pub const UNSIGNED_INT_VEC4: GLenum = 0x8DC8;
    pub const UNSIGNED_NORMALIZED: GLenum = 0x8C17;
    pub const UNSIGNED_SHORT: GLenum = 0x1403;
    pub const UNSIGNED_SHORT_1_5_5_5_REV: GLenum = 0x8366;
    pub const UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033;
    pub const UNSIGNED_SHORT_4_4_4_4_REV: GLenum = 0x8365;
    pub const UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034;
    pub const UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
    pub const UNSIGNED_SHORT_5_6_5_REV: GLenum = 0x8364;
    pub const UPPER_LEFT: GLenum = 0x8CA2;
    pub const V2F: GLenum = 0x2A20;
    pub const V3F: GLenum = 0x2A21;
    pub const VALIDATE_STATUS: GLenum = 0x8B83;
    pub const VENDOR: GLenum = 0x1F00;
    pub const VERSION: GLenum = 0x1F02;
    pub const VERTEX_ARRAY: GLenum = 0x8074;
    pub const VERTEX_ARRAY_BINDING: GLenum = 0x85B5;
    pub const VERTEX_ARRAY_BINDING_APPLE: GLenum = 0x85B5;
    pub const VERTEX_ARRAY_BUFFER_BINDING: GLenum = 0x8896;
    pub const VERTEX_ARRAY_KHR: GLenum = 0x8074;
    pub const VERTEX_ARRAY_POINTER: GLenum = 0x808E;
    pub const VERTEX_ARRAY_SIZE: GLenum = 0x807A;
    pub const VERTEX_ARRAY_STRIDE: GLenum = 0x807C;
    pub const VERTEX_ARRAY_TYPE: GLenum = 0x807B;
    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 0x889F;
    pub const VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 0x88FE;
    pub const VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622;
    pub const VERTEX_ATTRIB_ARRAY_INTEGER: GLenum = 0x88FD;
    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A;
    pub const VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645;
    pub const VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623;
    pub const VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624;
    pub const VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625;
    pub const VERTEX_PROGRAM_POINT_SIZE: GLenum = 0x8642;
    pub const VERTEX_PROGRAM_TWO_SIDE: GLenum = 0x8643;
    pub const VERTEX_SHADER: GLenum = 0x8B31;
    pub const VIEWPORT: GLenum = 0x0BA2;
    pub const VIEWPORT_BIT: GLenum = 0x00000800;
    pub const WAIT_FAILED: GLenum = 0x911D;
    pub const WEIGHT_ARRAY_BUFFER_BINDING: GLenum = 0x889E;
    pub const WRITE_ONLY: GLenum = 0x88B9;
    pub const XOR: GLenum = 0x1506;
    pub const ZERO: GLenum = 0;
    pub const ZOOM_X: GLenum = 0x0D16;
    pub const ZOOM_Y: GLenum = 0x0D17;

    use crate::vec::{GLuintVec, StringVec};
    use crate::option::OptionU8VecRef;
    /// `GlShaderPrecisionFormatReturn` struct
    
#[doc(inline)] pub use crate::dll::AzGlShaderPrecisionFormatReturn as GlShaderPrecisionFormatReturn;
    /// `VertexAttributeType` struct
    
#[doc(inline)] pub use crate::dll::AzVertexAttributeType as VertexAttributeType;
    /// `VertexAttribute` struct
    
#[doc(inline)] pub use crate::dll::AzVertexAttribute as VertexAttribute;
    /// `VertexLayout` struct
    
#[doc(inline)] pub use crate::dll::AzVertexLayout as VertexLayout;
    /// `VertexArrayObject` struct
    
#[doc(inline)] pub use crate::dll::AzVertexArrayObject as VertexArrayObject;
    /// `IndexBufferFormat` struct
    
#[doc(inline)] pub use crate::dll::AzIndexBufferFormat as IndexBufferFormat;
    /// `VertexBuffer` struct
    
#[doc(inline)] pub use crate::dll::AzVertexBuffer as VertexBuffer;
    /// `GlType` struct
    
#[doc(inline)] pub use crate::dll::AzGlType as GlType;
    /// `DebugMessage` struct
    
#[doc(inline)] pub use crate::dll::AzDebugMessage as DebugMessage;
    /// C-ABI stable reexport of `&[u8]`
    
#[doc(inline)] pub use crate::dll::AzU8VecRef as U8VecRef;
    /// C-ABI stable reexport of `&mut [u8]`
    
#[doc(inline)] pub use crate::dll::AzU8VecRefMut as U8VecRefMut;
    /// C-ABI stable reexport of `&[f32]`
    
#[doc(inline)] pub use crate::dll::AzF32VecRef as F32VecRef;
    /// C-ABI stable reexport of `&[i32]`
    
#[doc(inline)] pub use crate::dll::AzI32VecRef as I32VecRef;
    /// C-ABI stable reexport of `&[GLuint]` aka `&[u32]`
    
#[doc(inline)] pub use crate::dll::AzGLuintVecRef as GLuintVecRef;
    /// C-ABI stable reexport of `&[GLenum]` aka `&[u32]`
    
#[doc(inline)] pub use crate::dll::AzGLenumVecRef as GLenumVecRef;
    /// C-ABI stable reexport of `&mut [GLint]` aka `&mut [i32]`
    
#[doc(inline)] pub use crate::dll::AzGLintVecRefMut as GLintVecRefMut;
    /// C-ABI stable reexport of `&mut [GLint64]` aka `&mut [i64]`
    
#[doc(inline)] pub use crate::dll::AzGLint64VecRefMut as GLint64VecRefMut;
    /// C-ABI stable reexport of `&mut [GLboolean]` aka `&mut [u8]`
    
#[doc(inline)] pub use crate::dll::AzGLbooleanVecRefMut as GLbooleanVecRefMut;
    /// C-ABI stable reexport of `&mut [GLfloat]` aka `&mut [f32]`
    
#[doc(inline)] pub use crate::dll::AzGLfloatVecRefMut as GLfloatVecRefMut;
    /// C-ABI stable reexport of `&[Refstr]` aka `&mut [&str]`
    
#[doc(inline)] pub use crate::dll::AzRefstrVecRef as RefstrVecRef;
    /// C-ABI stable reexport of `&str`
    
#[doc(inline)] pub use crate::dll::AzRefstr as Refstr;
    /// C-ABI stable reexport of `(U8Vec, u32)`
    
#[doc(inline)] pub use crate::dll::AzGetProgramBinaryReturn as GetProgramBinaryReturn;
    /// C-ABI stable reexport of `(i32, u32, AzString)`
    
#[doc(inline)] pub use crate::dll::AzGetActiveAttribReturn as GetActiveAttribReturn;
    /// C-ABI stable reexport of `*const gleam::gl::GLsync`
    
#[doc(inline)] pub use crate::dll::AzGLsyncPtr as GLsyncPtr;
    impl Drop for GLsyncPtr { fn drop(&mut self) { unsafe { crate::dll::az_g_lsync_ptr_delete(self) } } }
    /// C-ABI stable reexport of `(i32, u32, AzString)`
    
#[doc(inline)] pub use crate::dll::AzGetActiveUniformReturn as GetActiveUniformReturn;
    /// `GlContextPtr` struct
    
#[doc(inline)] pub use crate::dll::AzGlContextPtr as GlContextPtr;
    impl GlContextPtr {
        /// Calls the `GlContextPtr::get_type` function.
        pub fn get_type(&self)  -> crate::gl::GlType { unsafe { crate::dll::az_gl_context_ptr_get_type(self) } }
        /// Calls the `GlContextPtr::buffer_data_untyped` function.
        pub fn buffer_data_untyped(&self, target: u32, size: isize, data: *const c_void, usage: u32)  { unsafe { crate::dll::az_gl_context_ptr_buffer_data_untyped(self, target, size, data, usage) } }
        /// Calls the `GlContextPtr::buffer_sub_data_untyped` function.
        pub fn buffer_sub_data_untyped(&self, target: u32, offset: isize, size: isize, data: *const c_void)  { unsafe { crate::dll::az_gl_context_ptr_buffer_sub_data_untyped(self, target, offset, size, data) } }
        /// Calls the `GlContextPtr::map_buffer` function.
        pub fn map_buffer(&self, target: u32, access: u32)  -> *mut c_void { unsafe { crate::dll::az_gl_context_ptr_map_buffer(self, target, access) } }
        /// Calls the `GlContextPtr::map_buffer_range` function.
        pub fn map_buffer_range(&self, target: u32, offset: isize, length: isize, access: u32)  -> *mut c_void { unsafe { crate::dll::az_gl_context_ptr_map_buffer_range(self, target, offset, length, access) } }
        /// Calls the `GlContextPtr::unmap_buffer` function.
        pub fn unmap_buffer(&self, target: u32)  -> u8 { unsafe { crate::dll::az_gl_context_ptr_unmap_buffer(self, target) } }
        /// Calls the `GlContextPtr::tex_buffer` function.
        pub fn tex_buffer(&self, target: u32, internal_format: u32, buffer: u32)  { unsafe { crate::dll::az_gl_context_ptr_tex_buffer(self, target, internal_format, buffer) } }
        /// Calls the `GlContextPtr::shader_source` function.
        pub fn shader_source(&self, shader: u32, strings: StringVec)  { unsafe { crate::dll::az_gl_context_ptr_shader_source(self, shader, strings) } }
        /// Calls the `GlContextPtr::read_buffer` function.
        pub fn read_buffer(&self, mode: u32)  { unsafe { crate::dll::az_gl_context_ptr_read_buffer(self, mode) } }
        /// Calls the `GlContextPtr::read_pixels_into_buffer` function.
        pub fn read_pixels_into_buffer(&self, x: i32, y: i32, width: i32, height: i32, format: u32, pixel_type: u32, dst_buffer: U8VecRefMut)  { unsafe { crate::dll::az_gl_context_ptr_read_pixels_into_buffer(self, x, y, width, height, format, pixel_type, dst_buffer) } }
        /// Calls the `GlContextPtr::read_pixels` function.
        pub fn read_pixels(&self, x: i32, y: i32, width: i32, height: i32, format: u32, pixel_type: u32)  -> crate::vec::U8Vec { unsafe { crate::dll::az_gl_context_ptr_read_pixels(self, x, y, width, height, format, pixel_type) } }
        /// Calls the `GlContextPtr::read_pixels_into_pbo` function.
        pub fn read_pixels_into_pbo(&self, x: i32, y: i32, width: i32, height: i32, format: u32, pixel_type: u32)  { unsafe { crate::dll::az_gl_context_ptr_read_pixels_into_pbo(self, x, y, width, height, format, pixel_type) } }
        /// Calls the `GlContextPtr::sample_coverage` function.
        pub fn sample_coverage(&self, value: f32, invert: bool)  { unsafe { crate::dll::az_gl_context_ptr_sample_coverage(self, value, invert) } }
        /// Calls the `GlContextPtr::polygon_offset` function.
        pub fn polygon_offset(&self, factor: f32, units: f32)  { unsafe { crate::dll::az_gl_context_ptr_polygon_offset(self, factor, units) } }
        /// Calls the `GlContextPtr::pixel_store_i` function.
        pub fn pixel_store_i(&self, name: u32, param: i32)  { unsafe { crate::dll::az_gl_context_ptr_pixel_store_i(self, name, param) } }
        /// Calls the `GlContextPtr::gen_buffers` function.
        pub fn gen_buffers(&self, n: i32)  -> crate::vec::GLuintVec { unsafe { crate::dll::az_gl_context_ptr_gen_buffers(self, n) } }
        /// Calls the `GlContextPtr::gen_renderbuffers` function.
        pub fn gen_renderbuffers(&self, n: i32)  -> crate::vec::GLuintVec { unsafe { crate::dll::az_gl_context_ptr_gen_renderbuffers(self, n) } }
        /// Calls the `GlContextPtr::gen_framebuffers` function.
        pub fn gen_framebuffers(&self, n: i32)  -> crate::vec::GLuintVec { unsafe { crate::dll::az_gl_context_ptr_gen_framebuffers(self, n) } }
        /// Calls the `GlContextPtr::gen_textures` function.
        pub fn gen_textures(&self, n: i32)  -> crate::vec::GLuintVec { unsafe { crate::dll::az_gl_context_ptr_gen_textures(self, n) } }
        /// Calls the `GlContextPtr::gen_vertex_arrays` function.
        pub fn gen_vertex_arrays(&self, n: i32)  -> crate::vec::GLuintVec { unsafe { crate::dll::az_gl_context_ptr_gen_vertex_arrays(self, n) } }
        /// Calls the `GlContextPtr::gen_queries` function.
        pub fn gen_queries(&self, n: i32)  -> crate::vec::GLuintVec { unsafe { crate::dll::az_gl_context_ptr_gen_queries(self, n) } }
        /// Calls the `GlContextPtr::begin_query` function.
        pub fn begin_query(&self, target: u32, id: u32)  { unsafe { crate::dll::az_gl_context_ptr_begin_query(self, target, id) } }
        /// Calls the `GlContextPtr::end_query` function.
        pub fn end_query(&self, target: u32)  { unsafe { crate::dll::az_gl_context_ptr_end_query(self, target) } }
        /// Calls the `GlContextPtr::query_counter` function.
        pub fn query_counter(&self, id: u32, target: u32)  { unsafe { crate::dll::az_gl_context_ptr_query_counter(self, id, target) } }
        /// Calls the `GlContextPtr::get_query_object_iv` function.
        pub fn get_query_object_iv(&self, id: u32, pname: u32)  -> i32 { unsafe { crate::dll::az_gl_context_ptr_get_query_object_iv(self, id, pname) } }
        /// Calls the `GlContextPtr::get_query_object_uiv` function.
        pub fn get_query_object_uiv(&self, id: u32, pname: u32)  -> u32 { unsafe { crate::dll::az_gl_context_ptr_get_query_object_uiv(self, id, pname) } }
        /// Calls the `GlContextPtr::get_query_object_i64v` function.
        pub fn get_query_object_i64v(&self, id: u32, pname: u32)  -> i64 { unsafe { crate::dll::az_gl_context_ptr_get_query_object_i64v(self, id, pname) } }
        /// Calls the `GlContextPtr::get_query_object_ui64v` function.
        pub fn get_query_object_ui64v(&self, id: u32, pname: u32)  -> u64 { unsafe { crate::dll::az_gl_context_ptr_get_query_object_ui64v(self, id, pname) } }
        /// Calls the `GlContextPtr::delete_queries` function.
        pub fn delete_queries(&self, queries: GLuintVecRef)  { unsafe { crate::dll::az_gl_context_ptr_delete_queries(self, queries) } }
        /// Calls the `GlContextPtr::delete_vertex_arrays` function.
        pub fn delete_vertex_arrays(&self, vertex_arrays: GLuintVecRef)  { unsafe { crate::dll::az_gl_context_ptr_delete_vertex_arrays(self, vertex_arrays) } }
        /// Calls the `GlContextPtr::delete_buffers` function.
        pub fn delete_buffers(&self, buffers: GLuintVecRef)  { unsafe { crate::dll::az_gl_context_ptr_delete_buffers(self, buffers) } }
        /// Calls the `GlContextPtr::delete_renderbuffers` function.
        pub fn delete_renderbuffers(&self, renderbuffers: GLuintVecRef)  { unsafe { crate::dll::az_gl_context_ptr_delete_renderbuffers(self, renderbuffers) } }
        /// Calls the `GlContextPtr::delete_framebuffers` function.
        pub fn delete_framebuffers(&self, framebuffers: GLuintVecRef)  { unsafe { crate::dll::az_gl_context_ptr_delete_framebuffers(self, framebuffers) } }
        /// Calls the `GlContextPtr::delete_textures` function.
        pub fn delete_textures(&self, textures: GLuintVecRef)  { unsafe { crate::dll::az_gl_context_ptr_delete_textures(self, textures) } }
        /// Calls the `GlContextPtr::framebuffer_renderbuffer` function.
        pub fn framebuffer_renderbuffer(&self, target: u32, attachment: u32, renderbuffertarget: u32, renderbuffer: u32)  { unsafe { crate::dll::az_gl_context_ptr_framebuffer_renderbuffer(self, target, attachment, renderbuffertarget, renderbuffer) } }
        /// Calls the `GlContextPtr::renderbuffer_storage` function.
        pub fn renderbuffer_storage(&self, target: u32, internalformat: u32, width: i32, height: i32)  { unsafe { crate::dll::az_gl_context_ptr_renderbuffer_storage(self, target, internalformat, width, height) } }
        /// Calls the `GlContextPtr::depth_func` function.
        pub fn depth_func(&self, func: u32)  { unsafe { crate::dll::az_gl_context_ptr_depth_func(self, func) } }
        /// Calls the `GlContextPtr::active_texture` function.
        pub fn active_texture(&self, texture: u32)  { unsafe { crate::dll::az_gl_context_ptr_active_texture(self, texture) } }
        /// Calls the `GlContextPtr::attach_shader` function.
        pub fn attach_shader(&self, program: u32, shader: u32)  { unsafe { crate::dll::az_gl_context_ptr_attach_shader(self, program, shader) } }
        /// Calls the `GlContextPtr::bind_attrib_location` function.
        pub fn bind_attrib_location(&self, program: u32, index: u32, name: Refstr)  { unsafe { crate::dll::az_gl_context_ptr_bind_attrib_location(self, program, index, name) } }
        /// Calls the `GlContextPtr::get_uniform_iv` function.
        pub fn get_uniform_iv(&self, program: u32, location: i32, result: GLintVecRefMut)  { unsafe { crate::dll::az_gl_context_ptr_get_uniform_iv(self, program, location, result) } }
        /// Calls the `GlContextPtr::get_uniform_fv` function.
        pub fn get_uniform_fv(&self, program: u32, location: i32, result: GLfloatVecRefMut)  { unsafe { crate::dll::az_gl_context_ptr_get_uniform_fv(self, program, location, result) } }
        /// Calls the `GlContextPtr::get_uniform_block_index` function.
        pub fn get_uniform_block_index(&self, program: u32, name: Refstr)  -> u32 { unsafe { crate::dll::az_gl_context_ptr_get_uniform_block_index(self, program, name) } }
        /// Calls the `GlContextPtr::get_uniform_indices` function.
        pub fn get_uniform_indices(&self, program: u32, names: RefstrVecRef)  -> crate::vec::GLuintVec { unsafe { crate::dll::az_gl_context_ptr_get_uniform_indices(self, program, names) } }
        /// Calls the `GlContextPtr::bind_buffer_base` function.
        pub fn bind_buffer_base(&self, target: u32, index: u32, buffer: u32)  { unsafe { crate::dll::az_gl_context_ptr_bind_buffer_base(self, target, index, buffer) } }
        /// Calls the `GlContextPtr::bind_buffer_range` function.
        pub fn bind_buffer_range(&self, target: u32, index: u32, buffer: u32, offset: isize, size: isize)  { unsafe { crate::dll::az_gl_context_ptr_bind_buffer_range(self, target, index, buffer, offset, size) } }
        /// Calls the `GlContextPtr::uniform_block_binding` function.
        pub fn uniform_block_binding(&self, program: u32, uniform_block_index: u32, uniform_block_binding: u32)  { unsafe { crate::dll::az_gl_context_ptr_uniform_block_binding(self, program, uniform_block_index, uniform_block_binding) } }
        /// Calls the `GlContextPtr::bind_buffer` function.
        pub fn bind_buffer(&self, target: u32, buffer: u32)  { unsafe { crate::dll::az_gl_context_ptr_bind_buffer(self, target, buffer) } }
        /// Calls the `GlContextPtr::bind_vertex_array` function.
        pub fn bind_vertex_array(&self, vao: u32)  { unsafe { crate::dll::az_gl_context_ptr_bind_vertex_array(self, vao) } }
        /// Calls the `GlContextPtr::bind_renderbuffer` function.
        pub fn bind_renderbuffer(&self, target: u32, renderbuffer: u32)  { unsafe { crate::dll::az_gl_context_ptr_bind_renderbuffer(self, target, renderbuffer) } }
        /// Calls the `GlContextPtr::bind_framebuffer` function.
        pub fn bind_framebuffer(&self, target: u32, framebuffer: u32)  { unsafe { crate::dll::az_gl_context_ptr_bind_framebuffer(self, target, framebuffer) } }
        /// Calls the `GlContextPtr::bind_texture` function.
        pub fn bind_texture(&self, target: u32, texture: u32)  { unsafe { crate::dll::az_gl_context_ptr_bind_texture(self, target, texture) } }
        /// Calls the `GlContextPtr::draw_buffers` function.
        pub fn draw_buffers(&self, bufs: GLenumVecRef)  { unsafe { crate::dll::az_gl_context_ptr_draw_buffers(self, bufs) } }
        /// Calls the `GlContextPtr::tex_image_2d` function.
        pub fn tex_image_2d(&self, target: u32, level: i32, internal_format: i32, width: i32, height: i32, border: i32, format: u32, ty: u32, opt_data: OptionU8VecRef)  { unsafe { crate::dll::az_gl_context_ptr_tex_image_2d(self, target, level, internal_format, width, height, border, format, ty, opt_data) } }
        /// Calls the `GlContextPtr::compressed_tex_image_2d` function.
        pub fn compressed_tex_image_2d(&self, target: u32, level: i32, internal_format: u32, width: i32, height: i32, border: i32, data: U8VecRef)  { unsafe { crate::dll::az_gl_context_ptr_compressed_tex_image_2d(self, target, level, internal_format, width, height, border, data) } }
        /// Calls the `GlContextPtr::compressed_tex_sub_image_2d` function.
        pub fn compressed_tex_sub_image_2d(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: u32, data: U8VecRef)  { unsafe { crate::dll::az_gl_context_ptr_compressed_tex_sub_image_2d(self, target, level, xoffset, yoffset, width, height, format, data) } }
        /// Calls the `GlContextPtr::tex_image_3d` function.
        pub fn tex_image_3d(&self, target: u32, level: i32, internal_format: i32, width: i32, height: i32, depth: i32, border: i32, format: u32, ty: u32, opt_data: OptionU8VecRef)  { unsafe { crate::dll::az_gl_context_ptr_tex_image_3d(self, target, level, internal_format, width, height, depth, border, format, ty, opt_data) } }
        /// Calls the `GlContextPtr::copy_tex_image_2d` function.
        pub fn copy_tex_image_2d(&self, target: u32, level: i32, internal_format: u32, x: i32, y: i32, width: i32, height: i32, border: i32)  { unsafe { crate::dll::az_gl_context_ptr_copy_tex_image_2d(self, target, level, internal_format, x, y, width, height, border) } }
        /// Calls the `GlContextPtr::copy_tex_sub_image_2d` function.
        pub fn copy_tex_sub_image_2d(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, x: i32, y: i32, width: i32, height: i32)  { unsafe { crate::dll::az_gl_context_ptr_copy_tex_sub_image_2d(self, target, level, xoffset, yoffset, x, y, width, height) } }
        /// Calls the `GlContextPtr::copy_tex_sub_image_3d` function.
        pub fn copy_tex_sub_image_3d(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, zoffset: i32, x: i32, y: i32, width: i32, height: i32)  { unsafe { crate::dll::az_gl_context_ptr_copy_tex_sub_image_3d(self, target, level, xoffset, yoffset, zoffset, x, y, width, height) } }
        /// Calls the `GlContextPtr::tex_sub_image_2d` function.
        pub fn tex_sub_image_2d(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: u32, ty: u32, data: U8VecRef)  { unsafe { crate::dll::az_gl_context_ptr_tex_sub_image_2d(self, target, level, xoffset, yoffset, width, height, format, ty, data) } }
        /// Calls the `GlContextPtr::tex_sub_image_2d_pbo` function.
        pub fn tex_sub_image_2d_pbo(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: u32, ty: u32, offset: usize)  { unsafe { crate::dll::az_gl_context_ptr_tex_sub_image_2d_pbo(self, target, level, xoffset, yoffset, width, height, format, ty, offset) } }
        /// Calls the `GlContextPtr::tex_sub_image_3d` function.
        pub fn tex_sub_image_3d(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, zoffset: i32, width: i32, height: i32, depth: i32, format: u32, ty: u32, data: U8VecRef)  { unsafe { crate::dll::az_gl_context_ptr_tex_sub_image_3d(self, target, level, xoffset, yoffset, zoffset, width, height, depth, format, ty, data) } }
        /// Calls the `GlContextPtr::tex_sub_image_3d_pbo` function.
        pub fn tex_sub_image_3d_pbo(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, zoffset: i32, width: i32, height: i32, depth: i32, format: u32, ty: u32, offset: usize)  { unsafe { crate::dll::az_gl_context_ptr_tex_sub_image_3d_pbo(self, target, level, xoffset, yoffset, zoffset, width, height, depth, format, ty, offset) } }
        /// Calls the `GlContextPtr::tex_storage_2d` function.
        pub fn tex_storage_2d(&self, target: u32, levels: i32, internal_format: u32, width: i32, height: i32)  { unsafe { crate::dll::az_gl_context_ptr_tex_storage_2d(self, target, levels, internal_format, width, height) } }
        /// Calls the `GlContextPtr::tex_storage_3d` function.
        pub fn tex_storage_3d(&self, target: u32, levels: i32, internal_format: u32, width: i32, height: i32, depth: i32)  { unsafe { crate::dll::az_gl_context_ptr_tex_storage_3d(self, target, levels, internal_format, width, height, depth) } }
        /// Calls the `GlContextPtr::get_tex_image_into_buffer` function.
        pub fn get_tex_image_into_buffer(&self, target: u32, level: i32, format: u32, ty: u32, output: U8VecRefMut)  { unsafe { crate::dll::az_gl_context_ptr_get_tex_image_into_buffer(self, target, level, format, ty, output) } }
        /// Calls the `GlContextPtr::copy_image_sub_data` function.
        pub fn copy_image_sub_data(&self, src_name: u32, src_target: u32, src_level: i32, src_x: i32, src_y: i32, src_z: i32, dst_name: u32, dst_target: u32, dst_level: i32, dst_x: i32, dst_y: i32, dst_z: i32, src_width: i32, src_height: i32, src_depth: i32)  { unsafe { crate::dll::az_gl_context_ptr_copy_image_sub_data(self, src_name, src_target, src_level, src_x, src_y, src_z, dst_name, dst_target, dst_level, dst_x, dst_y, dst_z, src_width, src_height, src_depth) } }
        /// Calls the `GlContextPtr::invalidate_framebuffer` function.
        pub fn invalidate_framebuffer(&self, target: u32, attachments: GLenumVecRef)  { unsafe { crate::dll::az_gl_context_ptr_invalidate_framebuffer(self, target, attachments) } }
        /// Calls the `GlContextPtr::invalidate_sub_framebuffer` function.
        pub fn invalidate_sub_framebuffer(&self, target: u32, attachments: GLenumVecRef, xoffset: i32, yoffset: i32, width: i32, height: i32)  { unsafe { crate::dll::az_gl_context_ptr_invalidate_sub_framebuffer(self, target, attachments, xoffset, yoffset, width, height) } }
        /// Calls the `GlContextPtr::get_integer_v` function.
        pub fn get_integer_v(&self, name: u32, result: GLintVecRefMut)  { unsafe { crate::dll::az_gl_context_ptr_get_integer_v(self, name, result) } }
        /// Calls the `GlContextPtr::get_integer_64v` function.
        pub fn get_integer_64v(&self, name: u32, result: GLint64VecRefMut)  { unsafe { crate::dll::az_gl_context_ptr_get_integer_64v(self, name, result) } }
        /// Calls the `GlContextPtr::get_integer_iv` function.
        pub fn get_integer_iv(&self, name: u32, index: u32, result: GLintVecRefMut)  { unsafe { crate::dll::az_gl_context_ptr_get_integer_iv(self, name, index, result) } }
        /// Calls the `GlContextPtr::get_integer_64iv` function.
        pub fn get_integer_64iv(&self, name: u32, index: u32, result: GLint64VecRefMut)  { unsafe { crate::dll::az_gl_context_ptr_get_integer_64iv(self, name, index, result) } }
        /// Calls the `GlContextPtr::get_boolean_v` function.
        pub fn get_boolean_v(&self, name: u32, result: GLbooleanVecRefMut)  { unsafe { crate::dll::az_gl_context_ptr_get_boolean_v(self, name, result) } }
        /// Calls the `GlContextPtr::get_float_v` function.
        pub fn get_float_v(&self, name: u32, result: GLfloatVecRefMut)  { unsafe { crate::dll::az_gl_context_ptr_get_float_v(self, name, result) } }
        /// Calls the `GlContextPtr::get_framebuffer_attachment_parameter_iv` function.
        pub fn get_framebuffer_attachment_parameter_iv(&self, target: u32, attachment: u32, pname: u32)  -> i32 { unsafe { crate::dll::az_gl_context_ptr_get_framebuffer_attachment_parameter_iv(self, target, attachment, pname) } }
        /// Calls the `GlContextPtr::get_renderbuffer_parameter_iv` function.
        pub fn get_renderbuffer_parameter_iv(&self, target: u32, pname: u32)  -> i32 { unsafe { crate::dll::az_gl_context_ptr_get_renderbuffer_parameter_iv(self, target, pname) } }
        /// Calls the `GlContextPtr::get_tex_parameter_iv` function.
        pub fn get_tex_parameter_iv(&self, target: u32, name: u32)  -> i32 { unsafe { crate::dll::az_gl_context_ptr_get_tex_parameter_iv(self, target, name) } }
        /// Calls the `GlContextPtr::get_tex_parameter_fv` function.
        pub fn get_tex_parameter_fv(&self, target: u32, name: u32)  -> f32 { unsafe { crate::dll::az_gl_context_ptr_get_tex_parameter_fv(self, target, name) } }
        /// Calls the `GlContextPtr::tex_parameter_i` function.
        pub fn tex_parameter_i(&self, target: u32, pname: u32, param: i32)  { unsafe { crate::dll::az_gl_context_ptr_tex_parameter_i(self, target, pname, param) } }
        /// Calls the `GlContextPtr::tex_parameter_f` function.
        pub fn tex_parameter_f(&self, target: u32, pname: u32, param: f32)  { unsafe { crate::dll::az_gl_context_ptr_tex_parameter_f(self, target, pname, param) } }
        /// Calls the `GlContextPtr::framebuffer_texture_2d` function.
        pub fn framebuffer_texture_2d(&self, target: u32, attachment: u32, textarget: u32, texture: u32, level: i32)  { unsafe { crate::dll::az_gl_context_ptr_framebuffer_texture_2d(self, target, attachment, textarget, texture, level) } }
        /// Calls the `GlContextPtr::framebuffer_texture_layer` function.
        pub fn framebuffer_texture_layer(&self, target: u32, attachment: u32, texture: u32, level: i32, layer: i32)  { unsafe { crate::dll::az_gl_context_ptr_framebuffer_texture_layer(self, target, attachment, texture, level, layer) } }
        /// Calls the `GlContextPtr::blit_framebuffer` function.
        pub fn blit_framebuffer(&self, src_x0: i32, src_y0: i32, src_x1: i32, src_y1: i32, dst_x0: i32, dst_y0: i32, dst_x1: i32, dst_y1: i32, mask: u32, filter: u32)  { unsafe { crate::dll::az_gl_context_ptr_blit_framebuffer(self, src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter) } }
        /// Calls the `GlContextPtr::vertex_attrib_4f` function.
        pub fn vertex_attrib_4f(&self, index: u32, x: f32, y: f32, z: f32, w: f32)  { unsafe { crate::dll::az_gl_context_ptr_vertex_attrib_4f(self, index, x, y, z, w) } }
        /// Calls the `GlContextPtr::vertex_attrib_pointer_f32` function.
        pub fn vertex_attrib_pointer_f32(&self, index: u32, size: i32, normalized: bool, stride: i32, offset: u32)  { unsafe { crate::dll::az_gl_context_ptr_vertex_attrib_pointer_f32(self, index, size, normalized, stride, offset) } }
        /// Calls the `GlContextPtr::vertex_attrib_pointer` function.
        pub fn vertex_attrib_pointer(&self, index: u32, size: i32, type_: u32, normalized: bool, stride: i32, offset: u32)  { unsafe { crate::dll::az_gl_context_ptr_vertex_attrib_pointer(self, index, size, type_, normalized, stride, offset) } }
        /// Calls the `GlContextPtr::vertex_attrib_i_pointer` function.
        pub fn vertex_attrib_i_pointer(&self, index: u32, size: i32, type_: u32, stride: i32, offset: u32)  { unsafe { crate::dll::az_gl_context_ptr_vertex_attrib_i_pointer(self, index, size, type_, stride, offset) } }
        /// Calls the `GlContextPtr::vertex_attrib_divisor` function.
        pub fn vertex_attrib_divisor(&self, index: u32, divisor: u32)  { unsafe { crate::dll::az_gl_context_ptr_vertex_attrib_divisor(self, index, divisor) } }
        /// Calls the `GlContextPtr::viewport` function.
        pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32)  { unsafe { crate::dll::az_gl_context_ptr_viewport(self, x, y, width, height) } }
        /// Calls the `GlContextPtr::scissor` function.
        pub fn scissor(&self, x: i32, y: i32, width: i32, height: i32)  { unsafe { crate::dll::az_gl_context_ptr_scissor(self, x, y, width, height) } }
        /// Calls the `GlContextPtr::line_width` function.
        pub fn line_width(&self, width: f32)  { unsafe { crate::dll::az_gl_context_ptr_line_width(self, width) } }
        /// Calls the `GlContextPtr::use_program` function.
        pub fn use_program(&self, program: u32)  { unsafe { crate::dll::az_gl_context_ptr_use_program(self, program) } }
        /// Calls the `GlContextPtr::validate_program` function.
        pub fn validate_program(&self, program: u32)  { unsafe { crate::dll::az_gl_context_ptr_validate_program(self, program) } }
        /// Calls the `GlContextPtr::draw_arrays` function.
        pub fn draw_arrays(&self, mode: u32, first: i32, count: i32)  { unsafe { crate::dll::az_gl_context_ptr_draw_arrays(self, mode, first, count) } }
        /// Calls the `GlContextPtr::draw_arrays_instanced` function.
        pub fn draw_arrays_instanced(&self, mode: u32, first: i32, count: i32, primcount: i32)  { unsafe { crate::dll::az_gl_context_ptr_draw_arrays_instanced(self, mode, first, count, primcount) } }
        /// Calls the `GlContextPtr::draw_elements` function.
        pub fn draw_elements(&self, mode: u32, count: i32, element_type: u32, indices_offset: u32)  { unsafe { crate::dll::az_gl_context_ptr_draw_elements(self, mode, count, element_type, indices_offset) } }
        /// Calls the `GlContextPtr::draw_elements_instanced` function.
        pub fn draw_elements_instanced(&self, mode: u32, count: i32, element_type: u32, indices_offset: u32, primcount: i32)  { unsafe { crate::dll::az_gl_context_ptr_draw_elements_instanced(self, mode, count, element_type, indices_offset, primcount) } }
        /// Calls the `GlContextPtr::blend_color` function.
        pub fn blend_color(&self, r: f32, g: f32, b: f32, a: f32)  { unsafe { crate::dll::az_gl_context_ptr_blend_color(self, r, g, b, a) } }
        /// Calls the `GlContextPtr::blend_func` function.
        pub fn blend_func(&self, sfactor: u32, dfactor: u32)  { unsafe { crate::dll::az_gl_context_ptr_blend_func(self, sfactor, dfactor) } }
        /// Calls the `GlContextPtr::blend_func_separate` function.
        pub fn blend_func_separate(&self, src_rgb: u32, dest_rgb: u32, src_alpha: u32, dest_alpha: u32)  { unsafe { crate::dll::az_gl_context_ptr_blend_func_separate(self, src_rgb, dest_rgb, src_alpha, dest_alpha) } }
        /// Calls the `GlContextPtr::blend_equation` function.
        pub fn blend_equation(&self, mode: u32)  { unsafe { crate::dll::az_gl_context_ptr_blend_equation(self, mode) } }
        /// Calls the `GlContextPtr::blend_equation_separate` function.
        pub fn blend_equation_separate(&self, mode_rgb: u32, mode_alpha: u32)  { unsafe { crate::dll::az_gl_context_ptr_blend_equation_separate(self, mode_rgb, mode_alpha) } }
        /// Calls the `GlContextPtr::color_mask` function.
        pub fn color_mask(&self, r: bool, g: bool, b: bool, a: bool)  { unsafe { crate::dll::az_gl_context_ptr_color_mask(self, r, g, b, a) } }
        /// Calls the `GlContextPtr::cull_face` function.
        pub fn cull_face(&self, mode: u32)  { unsafe { crate::dll::az_gl_context_ptr_cull_face(self, mode) } }
        /// Calls the `GlContextPtr::front_face` function.
        pub fn front_face(&self, mode: u32)  { unsafe { crate::dll::az_gl_context_ptr_front_face(self, mode) } }
        /// Calls the `GlContextPtr::enable` function.
        pub fn enable(&self, cap: u32)  { unsafe { crate::dll::az_gl_context_ptr_enable(self, cap) } }
        /// Calls the `GlContextPtr::disable` function.
        pub fn disable(&self, cap: u32)  { unsafe { crate::dll::az_gl_context_ptr_disable(self, cap) } }
        /// Calls the `GlContextPtr::hint` function.
        pub fn hint(&self, param_name: u32, param_val: u32)  { unsafe { crate::dll::az_gl_context_ptr_hint(self, param_name, param_val) } }
        /// Calls the `GlContextPtr::is_enabled` function.
        pub fn is_enabled(&self, cap: u32)  -> u8 { unsafe { crate::dll::az_gl_context_ptr_is_enabled(self, cap) } }
        /// Calls the `GlContextPtr::is_shader` function.
        pub fn is_shader(&self, shader: u32)  -> u8 { unsafe { crate::dll::az_gl_context_ptr_is_shader(self, shader) } }
        /// Calls the `GlContextPtr::is_texture` function.
        pub fn is_texture(&self, texture: u32)  -> u8 { unsafe { crate::dll::az_gl_context_ptr_is_texture(self, texture) } }
        /// Calls the `GlContextPtr::is_framebuffer` function.
        pub fn is_framebuffer(&self, framebuffer: u32)  -> u8 { unsafe { crate::dll::az_gl_context_ptr_is_framebuffer(self, framebuffer) } }
        /// Calls the `GlContextPtr::is_renderbuffer` function.
        pub fn is_renderbuffer(&self, renderbuffer: u32)  -> u8 { unsafe { crate::dll::az_gl_context_ptr_is_renderbuffer(self, renderbuffer) } }
        /// Calls the `GlContextPtr::check_frame_buffer_status` function.
        pub fn check_frame_buffer_status(&self, target: u32)  -> u32 { unsafe { crate::dll::az_gl_context_ptr_check_frame_buffer_status(self, target) } }
        /// Calls the `GlContextPtr::enable_vertex_attrib_array` function.
        pub fn enable_vertex_attrib_array(&self, index: u32)  { unsafe { crate::dll::az_gl_context_ptr_enable_vertex_attrib_array(self, index) } }
        /// Calls the `GlContextPtr::disable_vertex_attrib_array` function.
        pub fn disable_vertex_attrib_array(&self, index: u32)  { unsafe { crate::dll::az_gl_context_ptr_disable_vertex_attrib_array(self, index) } }
        /// Calls the `GlContextPtr::uniform_1f` function.
        pub fn uniform_1f(&self, location: i32, v0: f32)  { unsafe { crate::dll::az_gl_context_ptr_uniform_1f(self, location, v0) } }
        /// Calls the `GlContextPtr::uniform_1fv` function.
        pub fn uniform_1fv(&self, location: i32, values: F32VecRef)  { unsafe { crate::dll::az_gl_context_ptr_uniform_1fv(self, location, values) } }
        /// Calls the `GlContextPtr::uniform_1i` function.
        pub fn uniform_1i(&self, location: i32, v0: i32)  { unsafe { crate::dll::az_gl_context_ptr_uniform_1i(self, location, v0) } }
        /// Calls the `GlContextPtr::uniform_1iv` function.
        pub fn uniform_1iv(&self, location: i32, values: I32VecRef)  { unsafe { crate::dll::az_gl_context_ptr_uniform_1iv(self, location, values) } }
        /// Calls the `GlContextPtr::uniform_1ui` function.
        pub fn uniform_1ui(&self, location: i32, v0: u32)  { unsafe { crate::dll::az_gl_context_ptr_uniform_1ui(self, location, v0) } }
        /// Calls the `GlContextPtr::uniform_2f` function.
        pub fn uniform_2f(&self, location: i32, v0: f32, v1: f32)  { unsafe { crate::dll::az_gl_context_ptr_uniform_2f(self, location, v0, v1) } }
        /// Calls the `GlContextPtr::uniform_2fv` function.
        pub fn uniform_2fv(&self, location: i32, values: F32VecRef)  { unsafe { crate::dll::az_gl_context_ptr_uniform_2fv(self, location, values) } }
        /// Calls the `GlContextPtr::uniform_2i` function.
        pub fn uniform_2i(&self, location: i32, v0: i32, v1: i32)  { unsafe { crate::dll::az_gl_context_ptr_uniform_2i(self, location, v0, v1) } }
        /// Calls the `GlContextPtr::uniform_2iv` function.
        pub fn uniform_2iv(&self, location: i32, values: I32VecRef)  { unsafe { crate::dll::az_gl_context_ptr_uniform_2iv(self, location, values) } }
        /// Calls the `GlContextPtr::uniform_2ui` function.
        pub fn uniform_2ui(&self, location: i32, v0: u32, v1: u32)  { unsafe { crate::dll::az_gl_context_ptr_uniform_2ui(self, location, v0, v1) } }
        /// Calls the `GlContextPtr::uniform_3f` function.
        pub fn uniform_3f(&self, location: i32, v0: f32, v1: f32, v2: f32)  { unsafe { crate::dll::az_gl_context_ptr_uniform_3f(self, location, v0, v1, v2) } }
        /// Calls the `GlContextPtr::uniform_3fv` function.
        pub fn uniform_3fv(&self, location: i32, values: F32VecRef)  { unsafe { crate::dll::az_gl_context_ptr_uniform_3fv(self, location, values) } }
        /// Calls the `GlContextPtr::uniform_3i` function.
        pub fn uniform_3i(&self, location: i32, v0: i32, v1: i32, v2: i32)  { unsafe { crate::dll::az_gl_context_ptr_uniform_3i(self, location, v0, v1, v2) } }
        /// Calls the `GlContextPtr::uniform_3iv` function.
        pub fn uniform_3iv(&self, location: i32, values: I32VecRef)  { unsafe { crate::dll::az_gl_context_ptr_uniform_3iv(self, location, values) } }
        /// Calls the `GlContextPtr::uniform_3ui` function.
        pub fn uniform_3ui(&self, location: i32, v0: u32, v1: u32, v2: u32)  { unsafe { crate::dll::az_gl_context_ptr_uniform_3ui(self, location, v0, v1, v2) } }
        /// Calls the `GlContextPtr::uniform_4f` function.
        pub fn uniform_4f(&self, location: i32, x: f32, y: f32, z: f32, w: f32)  { unsafe { crate::dll::az_gl_context_ptr_uniform_4f(self, location, x, y, z, w) } }
        /// Calls the `GlContextPtr::uniform_4i` function.
        pub fn uniform_4i(&self, location: i32, x: i32, y: i32, z: i32, w: i32)  { unsafe { crate::dll::az_gl_context_ptr_uniform_4i(self, location, x, y, z, w) } }
        /// Calls the `GlContextPtr::uniform_4iv` function.
        pub fn uniform_4iv(&self, location: i32, values: I32VecRef)  { unsafe { crate::dll::az_gl_context_ptr_uniform_4iv(self, location, values) } }
        /// Calls the `GlContextPtr::uniform_4ui` function.
        pub fn uniform_4ui(&self, location: i32, x: u32, y: u32, z: u32, w: u32)  { unsafe { crate::dll::az_gl_context_ptr_uniform_4ui(self, location, x, y, z, w) } }
        /// Calls the `GlContextPtr::uniform_4fv` function.
        pub fn uniform_4fv(&self, location: i32, values: F32VecRef)  { unsafe { crate::dll::az_gl_context_ptr_uniform_4fv(self, location, values) } }
        /// Calls the `GlContextPtr::uniform_matrix_2fv` function.
        pub fn uniform_matrix_2fv(&self, location: i32, transpose: bool, value: F32VecRef)  { unsafe { crate::dll::az_gl_context_ptr_uniform_matrix_2fv(self, location, transpose, value) } }
        /// Calls the `GlContextPtr::uniform_matrix_3fv` function.
        pub fn uniform_matrix_3fv(&self, location: i32, transpose: bool, value: F32VecRef)  { unsafe { crate::dll::az_gl_context_ptr_uniform_matrix_3fv(self, location, transpose, value) } }
        /// Calls the `GlContextPtr::uniform_matrix_4fv` function.
        pub fn uniform_matrix_4fv(&self, location: i32, transpose: bool, value: F32VecRef)  { unsafe { crate::dll::az_gl_context_ptr_uniform_matrix_4fv(self, location, transpose, value) } }
        /// Calls the `GlContextPtr::depth_mask` function.
        pub fn depth_mask(&self, flag: bool)  { unsafe { crate::dll::az_gl_context_ptr_depth_mask(self, flag) } }
        /// Calls the `GlContextPtr::depth_range` function.
        pub fn depth_range(&self, near: f64, far: f64)  { unsafe { crate::dll::az_gl_context_ptr_depth_range(self, near, far) } }
        /// Calls the `GlContextPtr::get_active_attrib` function.
        pub fn get_active_attrib(&self, program: u32, index: u32)  -> crate::gl::GetActiveAttribReturn { unsafe { crate::dll::az_gl_context_ptr_get_active_attrib(self, program, index) } }
        /// Calls the `GlContextPtr::get_active_uniform` function.
        pub fn get_active_uniform(&self, program: u32, index: u32)  -> crate::gl::GetActiveUniformReturn { unsafe { crate::dll::az_gl_context_ptr_get_active_uniform(self, program, index) } }
        /// Calls the `GlContextPtr::get_active_uniforms_iv` function.
        pub fn get_active_uniforms_iv(&self, program: u32, indices: GLuintVec, pname: u32)  -> crate::vec::GLintVec { unsafe { crate::dll::az_gl_context_ptr_get_active_uniforms_iv(self, program, indices, pname) } }
        /// Calls the `GlContextPtr::get_active_uniform_block_i` function.
        pub fn get_active_uniform_block_i(&self, program: u32, index: u32, pname: u32)  -> i32 { unsafe { crate::dll::az_gl_context_ptr_get_active_uniform_block_i(self, program, index, pname) } }
        /// Calls the `GlContextPtr::get_active_uniform_block_iv` function.
        pub fn get_active_uniform_block_iv(&self, program: u32, index: u32, pname: u32)  -> crate::vec::GLintVec { unsafe { crate::dll::az_gl_context_ptr_get_active_uniform_block_iv(self, program, index, pname) } }
        /// Calls the `GlContextPtr::get_active_uniform_block_name` function.
        pub fn get_active_uniform_block_name(&self, program: u32, index: u32)  -> crate::str::String { unsafe { crate::dll::az_gl_context_ptr_get_active_uniform_block_name(self, program, index) } }
        /// Calls the `GlContextPtr::get_attrib_location` function.
        pub fn get_attrib_location(&self, program: u32, name: Refstr)  -> i32 { unsafe { crate::dll::az_gl_context_ptr_get_attrib_location(self, program, name) } }
        /// Calls the `GlContextPtr::get_frag_data_location` function.
        pub fn get_frag_data_location(&self, program: u32, name: Refstr)  -> i32 { unsafe { crate::dll::az_gl_context_ptr_get_frag_data_location(self, program, name) } }
        /// Calls the `GlContextPtr::get_uniform_location` function.
        pub fn get_uniform_location(&self, program: u32, name: Refstr)  -> i32 { unsafe { crate::dll::az_gl_context_ptr_get_uniform_location(self, program, name) } }
        /// Calls the `GlContextPtr::get_program_info_log` function.
        pub fn get_program_info_log(&self, program: u32)  -> crate::str::String { unsafe { crate::dll::az_gl_context_ptr_get_program_info_log(self, program) } }
        /// Calls the `GlContextPtr::get_program_iv` function.
        pub fn get_program_iv(&self, program: u32, pname: u32, result: GLintVecRefMut)  { unsafe { crate::dll::az_gl_context_ptr_get_program_iv(self, program, pname, result) } }
        /// Calls the `GlContextPtr::get_program_binary` function.
        pub fn get_program_binary(&self, program: u32)  -> crate::gl::GetProgramBinaryReturn { unsafe { crate::dll::az_gl_context_ptr_get_program_binary(self, program) } }
        /// Calls the `GlContextPtr::program_binary` function.
        pub fn program_binary(&self, program: u32, format: u32, binary: U8VecRef)  { unsafe { crate::dll::az_gl_context_ptr_program_binary(self, program, format, binary) } }
        /// Calls the `GlContextPtr::program_parameter_i` function.
        pub fn program_parameter_i(&self, program: u32, pname: u32, value: i32)  { unsafe { crate::dll::az_gl_context_ptr_program_parameter_i(self, program, pname, value) } }
        /// Calls the `GlContextPtr::get_vertex_attrib_iv` function.
        pub fn get_vertex_attrib_iv(&self, index: u32, pname: u32, result: GLintVecRefMut)  { unsafe { crate::dll::az_gl_context_ptr_get_vertex_attrib_iv(self, index, pname, result) } }
        /// Calls the `GlContextPtr::get_vertex_attrib_fv` function.
        pub fn get_vertex_attrib_fv(&self, index: u32, pname: u32, result: GLfloatVecRefMut)  { unsafe { crate::dll::az_gl_context_ptr_get_vertex_attrib_fv(self, index, pname, result) } }
        /// Calls the `GlContextPtr::get_vertex_attrib_pointer_v` function.
        pub fn get_vertex_attrib_pointer_v(&self, index: u32, pname: u32)  -> isize { unsafe { crate::dll::az_gl_context_ptr_get_vertex_attrib_pointer_v(self, index, pname) } }
        /// Calls the `GlContextPtr::get_buffer_parameter_iv` function.
        pub fn get_buffer_parameter_iv(&self, target: u32, pname: u32)  -> i32 { unsafe { crate::dll::az_gl_context_ptr_get_buffer_parameter_iv(self, target, pname) } }
        /// Calls the `GlContextPtr::get_shader_info_log` function.
        pub fn get_shader_info_log(&self, shader: u32)  -> crate::str::String { unsafe { crate::dll::az_gl_context_ptr_get_shader_info_log(self, shader) } }
        /// Calls the `GlContextPtr::get_string` function.
        pub fn get_string(&self, which: u32)  -> crate::str::String { unsafe { crate::dll::az_gl_context_ptr_get_string(self, which) } }
        /// Calls the `GlContextPtr::get_string_i` function.
        pub fn get_string_i(&self, which: u32, index: u32)  -> crate::str::String { unsafe { crate::dll::az_gl_context_ptr_get_string_i(self, which, index) } }
        /// Calls the `GlContextPtr::get_shader_iv` function.
        pub fn get_shader_iv(&self, shader: u32, pname: u32, result: GLintVecRefMut)  { unsafe { crate::dll::az_gl_context_ptr_get_shader_iv(self, shader, pname, result) } }
        /// Calls the `GlContextPtr::get_shader_precision_format` function.
        pub fn get_shader_precision_format(&self, shader_type: u32, precision_type: u32)  -> crate::gl::GlShaderPrecisionFormatReturn { unsafe { crate::dll::az_gl_context_ptr_get_shader_precision_format(self, shader_type, precision_type) } }
        /// Calls the `GlContextPtr::compile_shader` function.
        pub fn compile_shader(&self, shader: u32)  { unsafe { crate::dll::az_gl_context_ptr_compile_shader(self, shader) } }
        /// Calls the `GlContextPtr::create_program` function.
        pub fn create_program(&self)  -> u32 { unsafe { crate::dll::az_gl_context_ptr_create_program(self) } }
        /// Calls the `GlContextPtr::delete_program` function.
        pub fn delete_program(&self, program: u32)  { unsafe { crate::dll::az_gl_context_ptr_delete_program(self, program) } }
        /// Calls the `GlContextPtr::create_shader` function.
        pub fn create_shader(&self, shader_type: u32)  -> u32 { unsafe { crate::dll::az_gl_context_ptr_create_shader(self, shader_type) } }
        /// Calls the `GlContextPtr::delete_shader` function.
        pub fn delete_shader(&self, shader: u32)  { unsafe { crate::dll::az_gl_context_ptr_delete_shader(self, shader) } }
        /// Calls the `GlContextPtr::detach_shader` function.
        pub fn detach_shader(&self, program: u32, shader: u32)  { unsafe { crate::dll::az_gl_context_ptr_detach_shader(self, program, shader) } }
        /// Calls the `GlContextPtr::link_program` function.
        pub fn link_program(&self, program: u32)  { unsafe { crate::dll::az_gl_context_ptr_link_program(self, program) } }
        /// Calls the `GlContextPtr::clear_color` function.
        pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32)  { unsafe { crate::dll::az_gl_context_ptr_clear_color(self, r, g, b, a) } }
        /// Calls the `GlContextPtr::clear` function.
        pub fn clear(&self, buffer_mask: u32)  { unsafe { crate::dll::az_gl_context_ptr_clear(self, buffer_mask) } }
        /// Calls the `GlContextPtr::clear_depth` function.
        pub fn clear_depth(&self, depth: f64)  { unsafe { crate::dll::az_gl_context_ptr_clear_depth(self, depth) } }
        /// Calls the `GlContextPtr::clear_stencil` function.
        pub fn clear_stencil(&self, s: i32)  { unsafe { crate::dll::az_gl_context_ptr_clear_stencil(self, s) } }
        /// Calls the `GlContextPtr::flush` function.
        pub fn flush(&self)  { unsafe { crate::dll::az_gl_context_ptr_flush(self) } }
        /// Calls the `GlContextPtr::finish` function.
        pub fn finish(&self)  { unsafe { crate::dll::az_gl_context_ptr_finish(self) } }
        /// Calls the `GlContextPtr::get_error` function.
        pub fn get_error(&self)  -> u32 { unsafe { crate::dll::az_gl_context_ptr_get_error(self) } }
        /// Calls the `GlContextPtr::stencil_mask` function.
        pub fn stencil_mask(&self, mask: u32)  { unsafe { crate::dll::az_gl_context_ptr_stencil_mask(self, mask) } }
        /// Calls the `GlContextPtr::stencil_mask_separate` function.
        pub fn stencil_mask_separate(&self, face: u32, mask: u32)  { unsafe { crate::dll::az_gl_context_ptr_stencil_mask_separate(self, face, mask) } }
        /// Calls the `GlContextPtr::stencil_func` function.
        pub fn stencil_func(&self, func: u32, ref_: i32, mask: u32)  { unsafe { crate::dll::az_gl_context_ptr_stencil_func(self, func, ref_, mask) } }
        /// Calls the `GlContextPtr::stencil_func_separate` function.
        pub fn stencil_func_separate(&self, face: u32, func: u32, ref_: i32, mask: u32)  { unsafe { crate::dll::az_gl_context_ptr_stencil_func_separate(self, face, func, ref_, mask) } }
        /// Calls the `GlContextPtr::stencil_op` function.
        pub fn stencil_op(&self, sfail: u32, dpfail: u32, dppass: u32)  { unsafe { crate::dll::az_gl_context_ptr_stencil_op(self, sfail, dpfail, dppass) } }
        /// Calls the `GlContextPtr::stencil_op_separate` function.
        pub fn stencil_op_separate(&self, face: u32, sfail: u32, dpfail: u32, dppass: u32)  { unsafe { crate::dll::az_gl_context_ptr_stencil_op_separate(self, face, sfail, dpfail, dppass) } }
        /// Calls the `GlContextPtr::egl_image_target_texture2d_oes` function.
        pub fn egl_image_target_texture2d_oes(&self, target: u32, image: *const c_void)  { unsafe { crate::dll::az_gl_context_ptr_egl_image_target_texture2d_oes(self, target, image) } }
        /// Calls the `GlContextPtr::generate_mipmap` function.
        pub fn generate_mipmap(&self, target: u32)  { unsafe { crate::dll::az_gl_context_ptr_generate_mipmap(self, target) } }
        /// Calls the `GlContextPtr::insert_event_marker_ext` function.
        pub fn insert_event_marker_ext(&self, message: Refstr)  { unsafe { crate::dll::az_gl_context_ptr_insert_event_marker_ext(self, message) } }
        /// Calls the `GlContextPtr::push_group_marker_ext` function.
        pub fn push_group_marker_ext(&self, message: Refstr)  { unsafe { crate::dll::az_gl_context_ptr_push_group_marker_ext(self, message) } }
        /// Calls the `GlContextPtr::pop_group_marker_ext` function.
        pub fn pop_group_marker_ext(&self)  { unsafe { crate::dll::az_gl_context_ptr_pop_group_marker_ext(self) } }
        /// Calls the `GlContextPtr::debug_message_insert_khr` function.
        pub fn debug_message_insert_khr(&self, source: u32, type_: u32, id: u32, severity: u32, message: Refstr)  { unsafe { crate::dll::az_gl_context_ptr_debug_message_insert_khr(self, source, type_, id, severity, message) } }
        /// Calls the `GlContextPtr::push_debug_group_khr` function.
        pub fn push_debug_group_khr(&self, source: u32, id: u32, message: Refstr)  { unsafe { crate::dll::az_gl_context_ptr_push_debug_group_khr(self, source, id, message) } }
        /// Calls the `GlContextPtr::pop_debug_group_khr` function.
        pub fn pop_debug_group_khr(&self)  { unsafe { crate::dll::az_gl_context_ptr_pop_debug_group_khr(self) } }
        /// Calls the `GlContextPtr::fence_sync` function.
        pub fn fence_sync(&self, condition: u32, flags: u32)  -> crate::gl::GLsyncPtr { unsafe { crate::dll::az_gl_context_ptr_fence_sync(self, condition, flags) } }
        /// Calls the `GlContextPtr::client_wait_sync` function.
        pub fn client_wait_sync(&self, sync: GLsyncPtr, flags: u32, timeout: u64)  -> u32 { unsafe { crate::dll::az_gl_context_ptr_client_wait_sync(self, sync, flags, timeout) } }
        /// Calls the `GlContextPtr::wait_sync` function.
        pub fn wait_sync(&self, sync: GLsyncPtr, flags: u32, timeout: u64)  { unsafe { crate::dll::az_gl_context_ptr_wait_sync(self, sync, flags, timeout) } }
        /// Calls the `GlContextPtr::delete_sync` function.
        pub fn delete_sync(&self, sync: GLsyncPtr)  { unsafe { crate::dll::az_gl_context_ptr_delete_sync(self, sync) } }
        /// Calls the `GlContextPtr::texture_range_apple` function.
        pub fn texture_range_apple(&self, target: u32, data: U8VecRef)  { unsafe { crate::dll::az_gl_context_ptr_texture_range_apple(self, target, data) } }
        /// Calls the `GlContextPtr::gen_fences_apple` function.
        pub fn gen_fences_apple(&self, n: i32)  -> crate::vec::GLuintVec { unsafe { crate::dll::az_gl_context_ptr_gen_fences_apple(self, n) } }
        /// Calls the `GlContextPtr::delete_fences_apple` function.
        pub fn delete_fences_apple(&self, fences: GLuintVecRef)  { unsafe { crate::dll::az_gl_context_ptr_delete_fences_apple(self, fences) } }
        /// Calls the `GlContextPtr::set_fence_apple` function.
        pub fn set_fence_apple(&self, fence: u32)  { unsafe { crate::dll::az_gl_context_ptr_set_fence_apple(self, fence) } }
        /// Calls the `GlContextPtr::finish_fence_apple` function.
        pub fn finish_fence_apple(&self, fence: u32)  { unsafe { crate::dll::az_gl_context_ptr_finish_fence_apple(self, fence) } }
        /// Calls the `GlContextPtr::test_fence_apple` function.
        pub fn test_fence_apple(&self, fence: u32)  { unsafe { crate::dll::az_gl_context_ptr_test_fence_apple(self, fence) } }
        /// Calls the `GlContextPtr::test_object_apple` function.
        pub fn test_object_apple(&self, object: u32, name: u32)  -> u8 { unsafe { crate::dll::az_gl_context_ptr_test_object_apple(self, object, name) } }
        /// Calls the `GlContextPtr::finish_object_apple` function.
        pub fn finish_object_apple(&self, object: u32, name: u32)  { unsafe { crate::dll::az_gl_context_ptr_finish_object_apple(self, object, name) } }
        /// Calls the `GlContextPtr::get_frag_data_index` function.
        pub fn get_frag_data_index(&self, program: u32, name: Refstr)  -> i32 { unsafe { crate::dll::az_gl_context_ptr_get_frag_data_index(self, program, name) } }
        /// Calls the `GlContextPtr::blend_barrier_khr` function.
        pub fn blend_barrier_khr(&self)  { unsafe { crate::dll::az_gl_context_ptr_blend_barrier_khr(self) } }
        /// Calls the `GlContextPtr::bind_frag_data_location_indexed` function.
        pub fn bind_frag_data_location_indexed(&self, program: u32, color_number: u32, index: u32, name: Refstr)  { unsafe { crate::dll::az_gl_context_ptr_bind_frag_data_location_indexed(self, program, color_number, index, name) } }
        /// Calls the `GlContextPtr::get_debug_messages` function.
        pub fn get_debug_messages(&self)  -> crate::vec::DebugMessageVec { unsafe { crate::dll::az_gl_context_ptr_get_debug_messages(self) } }
        /// Calls the `GlContextPtr::provoking_vertex_angle` function.
        pub fn provoking_vertex_angle(&self, mode: u32)  { unsafe { crate::dll::az_gl_context_ptr_provoking_vertex_angle(self, mode) } }
        /// Calls the `GlContextPtr::gen_vertex_arrays_apple` function.
        pub fn gen_vertex_arrays_apple(&self, n: i32)  -> crate::vec::GLuintVec { unsafe { crate::dll::az_gl_context_ptr_gen_vertex_arrays_apple(self, n) } }
        /// Calls the `GlContextPtr::bind_vertex_array_apple` function.
        pub fn bind_vertex_array_apple(&self, vao: u32)  { unsafe { crate::dll::az_gl_context_ptr_bind_vertex_array_apple(self, vao) } }
        /// Calls the `GlContextPtr::delete_vertex_arrays_apple` function.
        pub fn delete_vertex_arrays_apple(&self, vertex_arrays: GLuintVecRef)  { unsafe { crate::dll::az_gl_context_ptr_delete_vertex_arrays_apple(self, vertex_arrays) } }
        /// Calls the `GlContextPtr::copy_texture_chromium` function.
        pub fn copy_texture_chromium(&self, source_id: u32, source_level: i32, dest_target: u32, dest_id: u32, dest_level: i32, internal_format: i32, dest_type: u32, unpack_flip_y: u8, unpack_premultiply_alpha: u8, unpack_unmultiply_alpha: u8)  { unsafe { crate::dll::az_gl_context_ptr_copy_texture_chromium(self, source_id, source_level, dest_target, dest_id, dest_level, internal_format, dest_type, unpack_flip_y, unpack_premultiply_alpha, unpack_unmultiply_alpha) } }
        /// Calls the `GlContextPtr::copy_sub_texture_chromium` function.
        pub fn copy_sub_texture_chromium(&self, source_id: u32, source_level: i32, dest_target: u32, dest_id: u32, dest_level: i32, x_offset: i32, y_offset: i32, x: i32, y: i32, width: i32, height: i32, unpack_flip_y: u8, unpack_premultiply_alpha: u8, unpack_unmultiply_alpha: u8)  { unsafe { crate::dll::az_gl_context_ptr_copy_sub_texture_chromium(self, source_id, source_level, dest_target, dest_id, dest_level, x_offset, y_offset, x, y, width, height, unpack_flip_y, unpack_premultiply_alpha, unpack_unmultiply_alpha) } }
        /// Calls the `GlContextPtr::egl_image_target_renderbuffer_storage_oes` function.
        pub fn egl_image_target_renderbuffer_storage_oes(&self, target: u32, image: *const c_void)  { unsafe { crate::dll::az_gl_context_ptr_egl_image_target_renderbuffer_storage_oes(self, target, image) } }
        /// Calls the `GlContextPtr::copy_texture_3d_angle` function.
        pub fn copy_texture_3d_angle(&self, source_id: u32, source_level: i32, dest_target: u32, dest_id: u32, dest_level: i32, internal_format: i32, dest_type: u32, unpack_flip_y: u8, unpack_premultiply_alpha: u8, unpack_unmultiply_alpha: u8)  { unsafe { crate::dll::az_gl_context_ptr_copy_texture_3d_angle(self, source_id, source_level, dest_target, dest_id, dest_level, internal_format, dest_type, unpack_flip_y, unpack_premultiply_alpha, unpack_unmultiply_alpha) } }
        /// Calls the `GlContextPtr::copy_sub_texture_3d_angle` function.
        pub fn copy_sub_texture_3d_angle(&self, source_id: u32, source_level: i32, dest_target: u32, dest_id: u32, dest_level: i32, x_offset: i32, y_offset: i32, z_offset: i32, x: i32, y: i32, z: i32, width: i32, height: i32, depth: i32, unpack_flip_y: u8, unpack_premultiply_alpha: u8, unpack_unmultiply_alpha: u8)  { unsafe { crate::dll::az_gl_context_ptr_copy_sub_texture_3d_angle(self, source_id, source_level, dest_target, dest_id, dest_level, x_offset, y_offset, z_offset, x, y, z, width, height, depth, unpack_flip_y, unpack_premultiply_alpha, unpack_unmultiply_alpha) } }
        /// Calls the `GlContextPtr::buffer_storage` function.
        pub fn buffer_storage(&self, target: u32, size: isize, data: *const c_void, flags: u32)  { unsafe { crate::dll::az_gl_context_ptr_buffer_storage(self, target, size, data, flags) } }
        /// Calls the `GlContextPtr::flush_mapped_buffer_range` function.
        pub fn flush_mapped_buffer_range(&self, target: u32, offset: isize, length: isize)  { unsafe { crate::dll::az_gl_context_ptr_flush_mapped_buffer_range(self, target, offset, length) } }
    }

    impl Clone for GlContextPtr { fn clone(&self) -> Self { unsafe { crate::dll::az_gl_context_ptr_deep_copy(self) } } }
    impl Drop for GlContextPtr { fn drop(&mut self) { unsafe { crate::dll::az_gl_context_ptr_delete(self) } } }
    /// `Texture` struct
    
#[doc(inline)] pub use crate::dll::AzTexture as Texture;
    impl Drop for Texture { fn drop(&mut self) { unsafe { crate::dll::az_texture_delete(self) } } }
    /// `TextureFlags` struct
    
#[doc(inline)] pub use crate::dll::AzTextureFlags as TextureFlags;
    impl TextureFlags {
        /// Default texture flags (not opaque, not a video texture)
        pub fn default() -> Self { unsafe { crate::dll::az_texture_flags_default() } }
    }

}

pub mod resources {
    #![allow(dead_code, unused_imports)]
    //! Struct definition for image / font / text IDs
    use crate::dll::*;
    use core::ffi::c_void;
    use crate::vec::U8Vec;
    /// `ImageMask` struct
    
#[doc(inline)] pub use crate::dll::AzImageMask as ImageMask;
    /// `RawImageFormat` struct
    
#[doc(inline)] pub use crate::dll::AzRawImageFormat as RawImageFormat;
    /// `ImageId` struct
    
#[doc(inline)] pub use crate::dll::AzImageId as ImageId;
    impl ImageId {
        /// Creates a new, unique `ImageId`
        pub fn new() -> Self { unsafe { crate::dll::az_image_id_new() } }
    }

    /// `FontId` struct
    
#[doc(inline)] pub use crate::dll::AzFontId as FontId;
    impl FontId {
        /// Creates a new, unique `FontId`
        pub fn new() -> Self { unsafe { crate::dll::az_font_id_new() } }
    }

    /// `ImageSource` struct
    
#[doc(inline)] pub use crate::dll::AzImageSource as ImageSource;
    /// `FontSource` struct
    
#[doc(inline)] pub use crate::dll::AzFontSource as FontSource;
    /// `RawImage` struct
    
#[doc(inline)] pub use crate::dll::AzRawImage as RawImage;
    impl RawImage {
        /// Creates a new `RawImage` by loading the decoded bytes
        pub fn new(decoded_pixels: U8Vec, width: usize, height: usize, data_format: RawImageFormat) -> Self { unsafe { crate::dll::az_raw_image_new(decoded_pixels, width, height, data_format) } }
    }

}

pub mod svg {
    #![allow(dead_code, unused_imports)]
    //! SVG parsing and rendering functions
    use crate::dll::*;
    use core::ffi::c_void;
    use crate::gl::U8VecRef;
    /// `SvgMultiPolygon` struct
    
#[doc(inline)] pub use crate::dll::AzSvgMultiPolygon as SvgMultiPolygon;
    /// `SvgNode` struct
    
#[doc(inline)] pub use crate::dll::AzSvgNode as SvgNode;
    /// `SvgStyledNode` struct
    
#[doc(inline)] pub use crate::dll::AzSvgStyledNode as SvgStyledNode;
    /// `SvgCircle` struct
    
#[doc(inline)] pub use crate::dll::AzSvgCircle as SvgCircle;
    /// `SvgPath` struct
    
#[doc(inline)] pub use crate::dll::AzSvgPath as SvgPath;
    /// `SvgPathElement` struct
    
#[doc(inline)] pub use crate::dll::AzSvgPathElement as SvgPathElement;
    /// `SvgLine` struct
    
#[doc(inline)] pub use crate::dll::AzSvgLine as SvgLine;
    /// `SvgPoint` struct
    
#[doc(inline)] pub use crate::dll::AzSvgPoint as SvgPoint;
    /// `SvgVertex` struct
    
#[doc(inline)] pub use crate::dll::AzSvgVertex as SvgVertex;
    /// `SvgQuadraticCurve` struct
    
#[doc(inline)] pub use crate::dll::AzSvgQuadraticCurve as SvgQuadraticCurve;
    /// `SvgCubicCurve` struct
    
#[doc(inline)] pub use crate::dll::AzSvgCubicCurve as SvgCubicCurve;
    /// `SvgRect` struct
    
#[doc(inline)] pub use crate::dll::AzSvgRect as SvgRect;
    /// `TesselatedCPUSvgNode` struct
    
#[doc(inline)] pub use crate::dll::AzTesselatedCPUSvgNode as TesselatedCPUSvgNode;
    /// `SvgLineCap` struct
    
#[doc(inline)] pub use crate::dll::AzSvgLineCap as SvgLineCap;
    /// `SvgParseOptions` struct
    
#[doc(inline)] pub use crate::dll::AzSvgParseOptions as SvgParseOptions;
    impl SvgParseOptions {
        /// Creates a new `SvgParseOptions` instance.
        pub fn default() -> Self { unsafe { crate::dll::az_svg_parse_options_default() } }
    }

    /// `ShapeRendering` struct
    
#[doc(inline)] pub use crate::dll::AzShapeRendering as ShapeRendering;
    /// `TextRendering` struct
    
#[doc(inline)] pub use crate::dll::AzTextRendering as TextRendering;
    /// `ImageRendering` struct
    
#[doc(inline)] pub use crate::dll::AzImageRendering as ImageRendering;
    /// `FontDatabase` struct
    
#[doc(inline)] pub use crate::dll::AzFontDatabase as FontDatabase;
    /// `SvgRenderOptions` struct
    
#[doc(inline)] pub use crate::dll::AzSvgRenderOptions as SvgRenderOptions;
    impl SvgRenderOptions {
        /// Creates a new `SvgRenderOptions` instance.
        pub fn default() -> Self { unsafe { crate::dll::az_svg_render_options_default() } }
    }

    /// `SvgFitTo` struct
    
#[doc(inline)] pub use crate::dll::AzSvgFitTo as SvgFitTo;
    /// `Svg` struct
    
#[doc(inline)] pub use crate::dll::AzSvg as Svg;
    impl Svg {
        /// Creates a new `Svg` instance.
        pub fn parse(svg_bytes: U8VecRef, parse_options: SvgParseOptions) ->  crate::error::ResultSvgSvgParseError { unsafe { crate::dll::az_svg_parse(svg_bytes, parse_options) } }
    }

    impl Clone for Svg { fn clone(&self) -> Self { unsafe { crate::dll::az_svg_deep_copy(self) } } }
    impl Drop for Svg { fn drop(&mut self) { unsafe { crate::dll::az_svg_delete(self) } } }
    /// `SvgXmlNode` struct
    
#[doc(inline)] pub use crate::dll::AzSvgXmlNode as SvgXmlNode;
    impl Clone for SvgXmlNode { fn clone(&self) -> Self { unsafe { crate::dll::az_svg_xml_node_deep_copy(self) } } }
    impl Drop for SvgXmlNode { fn drop(&mut self) { unsafe { crate::dll::az_svg_xml_node_delete(self) } } }
    /// `SvgLineJoin` struct
    
#[doc(inline)] pub use crate::dll::AzSvgLineJoin as SvgLineJoin;
    /// `SvgDashPattern` struct
    
#[doc(inline)] pub use crate::dll::AzSvgDashPattern as SvgDashPattern;
    /// `SvgStyle` struct
    
#[doc(inline)] pub use crate::dll::AzSvgStyle as SvgStyle;
    /// `SvgFillStyle` struct
    
#[doc(inline)] pub use crate::dll::AzSvgFillStyle as SvgFillStyle;
    /// `SvgStrokeStyle` struct
    
#[doc(inline)] pub use crate::dll::AzSvgStrokeStyle as SvgStrokeStyle;
}

pub mod task {
    #![allow(dead_code, unused_imports)]
    //! Asyncronous timers / task / thread handlers for easy async loading
    use crate::dll::*;
    use core::ffi::c_void;
    use crate::callbacks::{RefAny, TimerCallbackType};
    use crate::time::Duration;
    /// `TimerId` struct
    
#[doc(inline)] pub use crate::dll::AzTimerId as TimerId;
    impl TimerId {
        /// Creates a new `TimerId` instance.
        pub fn unique() -> Self { unsafe { crate::dll::az_timer_id_unique() } }
    }

    /// `Timer` struct
    
#[doc(inline)] pub use crate::dll::AzTimer as Timer;
    impl Timer {
        /// Creates a new `Timer` instance.
        pub fn new(timer_data: RefAny, callback: TimerCallbackType, get_system_time_fn: GetSystemTimeFn) -> Self { unsafe { crate::dll::az_timer_new(timer_data, callback, get_system_time_fn) } }
        /// Calls the `Timer::with_delay` function.
        pub fn with_delay(self, delay: Duration)  -> crate::task::Timer { unsafe { crate::dll::az_timer_with_delay(self, delay) } }
        /// Calls the `Timer::with_interval` function.
        pub fn with_interval(self, interval: Duration)  -> crate::task::Timer { unsafe { crate::dll::az_timer_with_interval(self, interval) } }
        /// Calls the `Timer::with_timeout` function.
        pub fn with_timeout(self, timeout: Duration)  -> crate::task::Timer { unsafe { crate::dll::az_timer_with_timeout(self, timeout) } }
    }

    /// Should a timer terminate or not - used to remove active timers
    
#[doc(inline)] pub use crate::dll::AzTerminateTimer as TerminateTimer;
    /// `ThreadId` struct
    
#[doc(inline)] pub use crate::dll::AzThreadId as ThreadId;
    /// `Thread` struct
    
#[doc(inline)] pub use crate::dll::AzThread as Thread;
    /// `ThreadSender` struct
    
#[doc(inline)] pub use crate::dll::AzThreadSender as ThreadSender;
    impl ThreadSender {
        /// Calls the `ThreadSender::send` function.
        pub fn send(&mut self, msg: ThreadReceiveMsg)  -> bool { unsafe { crate::dll::az_thread_sender_send(self, msg) } }
    }

    impl Drop for ThreadSender { fn drop(&mut self) { unsafe { crate::dll::az_thread_sender_delete(self) } } }
    /// `ThreadReceiver` struct
    
#[doc(inline)] pub use crate::dll::AzThreadReceiver as ThreadReceiver;
    impl ThreadReceiver {
        /// Calls the `ThreadReceiver::receive` function.
        pub fn receive(&mut self)  -> crate::option::OptionThreadSendMsg { unsafe { crate::dll::az_thread_receiver_receive(self) } }
    }

    impl Drop for ThreadReceiver { fn drop(&mut self) { unsafe { crate::dll::az_thread_receiver_delete(self) } } }
    /// `ThreadSendMsg` struct
    
#[doc(inline)] pub use crate::dll::AzThreadSendMsg as ThreadSendMsg;
    /// `ThreadReceiveMsg` struct
    
#[doc(inline)] pub use crate::dll::AzThreadReceiveMsg as ThreadReceiveMsg;
    /// `ThreadWriteBackMsg` struct
    
#[doc(inline)] pub use crate::dll::AzThreadWriteBackMsg as ThreadWriteBackMsg;
    /// `CreateThreadFnType` struct
    
#[doc(inline)] pub use crate::dll::AzCreateThreadFnType as CreateThreadFnType;
    /// `CreateThreadFn` struct
    
#[doc(inline)] pub use crate::dll::AzCreateThreadFn as CreateThreadFn;
    /// `GetSystemTimeFnType` struct
    
#[doc(inline)] pub use crate::dll::AzGetSystemTimeFnType as GetSystemTimeFnType;
    /// Get the current system time, equivalent to `std::time::Instant::now()`, except it also works on systems that work with "ticks" instead of timers
    
#[doc(inline)] pub use crate::dll::AzGetSystemTimeFn as GetSystemTimeFn;
    /// `CheckThreadFinishedFnType` struct
    
#[doc(inline)] pub use crate::dll::AzCheckThreadFinishedFnType as CheckThreadFinishedFnType;
    /// Function called to check if the thread has finished
    
#[doc(inline)] pub use crate::dll::AzCheckThreadFinishedFn as CheckThreadFinishedFn;
    /// `LibrarySendThreadMsgFnType` struct
    
#[doc(inline)] pub use crate::dll::AzLibrarySendThreadMsgFnType as LibrarySendThreadMsgFnType;
    /// Function to send a message to the thread
    
#[doc(inline)] pub use crate::dll::AzLibrarySendThreadMsgFn as LibrarySendThreadMsgFn;
    /// `LibraryReceiveThreadMsgFnType` struct
    
#[doc(inline)] pub use crate::dll::AzLibraryReceiveThreadMsgFnType as LibraryReceiveThreadMsgFnType;
    /// Function to receive a message from the thread
    
#[doc(inline)] pub use crate::dll::AzLibraryReceiveThreadMsgFn as LibraryReceiveThreadMsgFn;
    /// `ThreadRecvFnType` struct
    
#[doc(inline)] pub use crate::dll::AzThreadRecvFnType as ThreadRecvFnType;
    /// Function that the running `Thread` can call to receive messages from the main UI thread
    
#[doc(inline)] pub use crate::dll::AzThreadRecvFn as ThreadRecvFn;
    /// `ThreadSendFnType` struct
    
#[doc(inline)] pub use crate::dll::AzThreadSendFnType as ThreadSendFnType;
    /// Function that the running `Thread` can call to receive messages from the main UI thread
    
#[doc(inline)] pub use crate::dll::AzThreadSendFn as ThreadSendFn;
    /// `ThreadDestructorFnType` struct
    
#[doc(inline)] pub use crate::dll::AzThreadDestructorFnType as ThreadDestructorFnType;
    /// Destructor of the `Thread`
    
#[doc(inline)] pub use crate::dll::AzThreadDestructorFn as ThreadDestructorFn;
    /// `ThreadReceiverDestructorFnType` struct
    
#[doc(inline)] pub use crate::dll::AzThreadReceiverDestructorFnType as ThreadReceiverDestructorFnType;
    /// Destructor of the `ThreadReceiver`
    
#[doc(inline)] pub use crate::dll::AzThreadReceiverDestructorFn as ThreadReceiverDestructorFn;
    /// `ThreadSenderDestructorFnType` struct
    
#[doc(inline)] pub use crate::dll::AzThreadSenderDestructorFnType as ThreadSenderDestructorFnType;
    /// Destructor of the `ThreadSender`
    
#[doc(inline)] pub use crate::dll::AzThreadSenderDestructorFn as ThreadSenderDestructorFn;
}

pub mod window {
    #![allow(dead_code, unused_imports)]
    //! Window creation / startup configuration
    use crate::dll::*;
    use core::ffi::c_void;

    impl LayoutSize {
        #[inline(always)]
        pub const fn new(width: isize, height: isize) -> Self { Self { width, height } }
        #[inline(always)]
        pub const fn zero() -> Self { Self::new(0, 0) }
    }

    impl LayoutPoint {
        #[inline(always)]
        pub const fn new(x: isize, y: isize) -> Self { Self { x, y } }
        #[inline(always)]
        pub const fn zero() -> Self { Self::new(0, 0) }
    }

    impl LayoutRect {
        #[inline(always)]
        pub const fn new(origin: LayoutPoint, size: LayoutSize) -> Self { Self { origin, size } }
        #[inline(always)]
        pub const fn zero() -> Self { Self::new(LayoutPoint::zero(), LayoutSize::zero()) }
        #[inline(always)]
        pub const fn max_x(&self) -> isize { self.origin.x + self.size.width }
        #[inline(always)]
        pub const fn min_x(&self) -> isize { self.origin.x }
        #[inline(always)]
        pub const fn max_y(&self) -> isize { self.origin.y + self.size.height }
        #[inline(always)]
        pub const fn min_y(&self) -> isize { self.origin.y }

        pub const fn contains(&self, other: &LayoutPoint) -> bool {
            self.min_x() <= other.x && other.x < self.max_x() &&
            self.min_y() <= other.y && other.y < self.max_y()
        }

        pub fn contains_f32(&self, other_x: f32, other_y: f32) -> bool {
            self.min_x() as f32 <= other_x && other_x < self.max_x() as f32 &&
            self.min_y() as f32 <= other_y && other_y < self.max_y() as f32
        }

        /// Same as `contains()`, but returns the (x, y) offset of the hit point
        ///
        /// On a regular computer this function takes ~3.2ns to run
        #[inline]
        pub const fn hit_test(&self, other: &LayoutPoint) -> Option<LayoutPoint> {
            let dx_left_edge = other.x - self.min_x();
            let dx_right_edge = self.max_x() - other.x;
            let dy_top_edge = other.y - self.min_y();
            let dy_bottom_edge = self.max_y() - other.y;
            if dx_left_edge > 0 &&
               dx_right_edge > 0 &&
               dy_top_edge > 0 &&
               dy_bottom_edge > 0
            {
                Some(LayoutPoint::new(dx_left_edge, dy_top_edge))
            } else {
                None
            }
        }

        // Returns if b overlaps a
        #[inline(always)]
        pub const fn contains_rect(&self, b: &LayoutRect) -> bool {

            let a = self;

            let a_x         = a.origin.x;
            let a_y         = a.origin.y;
            let a_width     = a.size.width;
            let a_height    = a.size.height;

            let b_x         = b.origin.x;
            let b_y         = b.origin.y;
            let b_width     = b.size.width;
            let b_height    = b.size.height;

            b_x >= a_x &&
            b_y >= a_y &&
            b_x + b_width <= a_x + a_width &&
            b_y + b_height <= a_y + a_height
        }
    }    use crate::callbacks::LayoutCallbackType;
    /// `RendererOptions` struct
    
#[doc(inline)] pub use crate::dll::AzRendererOptions as RendererOptions;
    /// `Vsync` struct
    
#[doc(inline)] pub use crate::dll::AzVsync as Vsync;
    /// `Srgb` struct
    
#[doc(inline)] pub use crate::dll::AzSrgb as Srgb;
    /// `HwAcceleration` struct
    
#[doc(inline)] pub use crate::dll::AzHwAcceleration as HwAcceleration;
    /// `LayoutPoint` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutPoint as LayoutPoint;
    /// `LayoutSize` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutSize as LayoutSize;
    /// `LayoutRect` struct
    
#[doc(inline)] pub use crate::dll::AzLayoutRect as LayoutRect;
    /// `RawWindowHandle` struct
    
#[doc(inline)] pub use crate::dll::AzRawWindowHandle as RawWindowHandle;
    /// `IOSHandle` struct
    
#[doc(inline)] pub use crate::dll::AzIOSHandle as IOSHandle;
    /// `MacOSHandle` struct
    
#[doc(inline)] pub use crate::dll::AzMacOSHandle as MacOSHandle;
    /// `XlibHandle` struct
    
#[doc(inline)] pub use crate::dll::AzXlibHandle as XlibHandle;
    /// `XcbHandle` struct
    
#[doc(inline)] pub use crate::dll::AzXcbHandle as XcbHandle;
    /// `WaylandHandle` struct
    
#[doc(inline)] pub use crate::dll::AzWaylandHandle as WaylandHandle;
    /// `WindowsHandle` struct
    
#[doc(inline)] pub use crate::dll::AzWindowsHandle as WindowsHandle;
    /// `WebHandle` struct
    
#[doc(inline)] pub use crate::dll::AzWebHandle as WebHandle;
    /// `AndroidHandle` struct
    
#[doc(inline)] pub use crate::dll::AzAndroidHandle as AndroidHandle;
    /// `TaskBarIcon` struct
    
#[doc(inline)] pub use crate::dll::AzTaskBarIcon as TaskBarIcon;
    /// `XWindowType` struct
    
#[doc(inline)] pub use crate::dll::AzXWindowType as XWindowType;
    /// `PhysicalPositionI32` struct
    
#[doc(inline)] pub use crate::dll::AzPhysicalPositionI32 as PhysicalPositionI32;
    /// `PhysicalSizeU32` struct
    
#[doc(inline)] pub use crate::dll::AzPhysicalSizeU32 as PhysicalSizeU32;
    /// `LogicalPosition` struct
    
#[doc(inline)] pub use crate::dll::AzLogicalPosition as LogicalPosition;
    /// `LogicalRect` struct
    
#[doc(inline)] pub use crate::dll::AzLogicalRect as LogicalRect;
    /// `IconKey` struct
    
#[doc(inline)] pub use crate::dll::AzIconKey as IconKey;
    /// `SmallWindowIconBytes` struct
    
#[doc(inline)] pub use crate::dll::AzSmallWindowIconBytes as SmallWindowIconBytes;
    /// `LargeWindowIconBytes` struct
    
#[doc(inline)] pub use crate::dll::AzLargeWindowIconBytes as LargeWindowIconBytes;
    /// `WindowIcon` struct
    
#[doc(inline)] pub use crate::dll::AzWindowIcon as WindowIcon;
    /// `VirtualKeyCode` struct
    
#[doc(inline)] pub use crate::dll::AzVirtualKeyCode as VirtualKeyCode;
    /// `AcceleratorKey` struct
    
#[doc(inline)] pub use crate::dll::AzAcceleratorKey as AcceleratorKey;
    /// `WindowSize` struct
    
#[doc(inline)] pub use crate::dll::AzWindowSize as WindowSize;
    /// `WindowFlags` struct
    
#[doc(inline)] pub use crate::dll::AzWindowFlags as WindowFlags;
    /// `DebugState` struct
    
#[doc(inline)] pub use crate::dll::AzDebugState as DebugState;
    /// `KeyboardState` struct
    
#[doc(inline)] pub use crate::dll::AzKeyboardState as KeyboardState;
    /// `MouseCursorType` struct
    
#[doc(inline)] pub use crate::dll::AzMouseCursorType as MouseCursorType;
    /// `CursorPosition` struct
    
#[doc(inline)] pub use crate::dll::AzCursorPosition as CursorPosition;
    /// `MouseState` struct
    
#[doc(inline)] pub use crate::dll::AzMouseState as MouseState;
    /// `PlatformSpecificOptions` struct
    
#[doc(inline)] pub use crate::dll::AzPlatformSpecificOptions as PlatformSpecificOptions;
    /// `WindowsWindowOptions` struct
    
#[doc(inline)] pub use crate::dll::AzWindowsWindowOptions as WindowsWindowOptions;
    /// `WaylandTheme` struct
    
#[doc(inline)] pub use crate::dll::AzWaylandTheme as WaylandTheme;
    /// `RendererType` struct
    
#[doc(inline)] pub use crate::dll::AzRendererType as RendererType;
    /// `StringPair` struct
    
#[doc(inline)] pub use crate::dll::AzStringPair as StringPair;
    /// `LinuxWindowOptions` struct
    
#[doc(inline)] pub use crate::dll::AzLinuxWindowOptions as LinuxWindowOptions;
    /// `MacWindowOptions` struct
    
#[doc(inline)] pub use crate::dll::AzMacWindowOptions as MacWindowOptions;
    /// `WasmWindowOptions` struct
    
#[doc(inline)] pub use crate::dll::AzWasmWindowOptions as WasmWindowOptions;
    /// `FullScreenMode` struct
    
#[doc(inline)] pub use crate::dll::AzFullScreenMode as FullScreenMode;
    /// `WindowTheme` struct
    
#[doc(inline)] pub use crate::dll::AzWindowTheme as WindowTheme;
    /// `WindowPosition` struct
    
#[doc(inline)] pub use crate::dll::AzWindowPosition as WindowPosition;
    /// `ImePosition` struct
    
#[doc(inline)] pub use crate::dll::AzImePosition as ImePosition;
    /// `TouchState` struct
    
#[doc(inline)] pub use crate::dll::AzTouchState as TouchState;
    /// `WindowState` struct
    
#[doc(inline)] pub use crate::dll::AzWindowState as WindowState;
    impl WindowState {
        /// Creates a new `WindowState` instance.
        pub fn new(layout_callback: LayoutCallbackType) -> Self { unsafe { crate::dll::az_window_state_new(layout_callback) } }
        /// Creates a new `WindowState` instance.
        pub fn default() -> Self { unsafe { crate::dll::az_window_state_default() } }
    }

    /// `LogicalSize` struct
    
#[doc(inline)] pub use crate::dll::AzLogicalSize as LogicalSize;
    /// `WindowCreateOptions` struct
    
#[doc(inline)] pub use crate::dll::AzWindowCreateOptions as WindowCreateOptions;
    impl WindowCreateOptions {
        /// Creates a new `WindowCreateOptions` instance.
        pub fn new(layout_callback: LayoutCallbackType) -> Self { unsafe { crate::dll::az_window_create_options_new(layout_callback) } }
        /// Creates a new `WindowCreateOptions` instance.
        pub fn default() -> Self { unsafe { crate::dll::az_window_create_options_default() } }
    }

}

