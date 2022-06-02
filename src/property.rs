use crate::{
    border_corner_shape::BorderCornerShape, box_shadow::BoxShadow, color::Color,
    cursor_icon::CursorIcon, display::Display, layout_type::LayoutType, macros::define_property,
    overflow::Overflow, position_type::PositionType, transition::Transition, units::Units,
    visibility::Visibility, BorderRadius, CustomParseError, FontSize, Length, Opacity, Parse,
};
use cssparser::Parser;

define_property! {
    pub enum Property {
        // General
        "display": Display(Display),
        "visibility": Visibility(Visibility),
        "overflow": Overflow(Overflow),
        "opacity": Opacity(Opacity),

        // Positioning
        "layout-type": LayoutType(LayoutType),
        "position-type": PositionType(PositionType),

        // Position and Size
        "space": Space(Units),
        "left": Left(Units),
        "width": Width(Units),
        "right": Right(Units),
        "top": Top(Units),
        "height": Height(Units),
        "bottom": Bottom(Units),

        // Constraints
        "min-left": MinLeft(Units),
        "max-left": MaxLeft(Units),
        "min-width": MinWidth(Units),
        "max-width": MaxWidth(Units),
        "min-right": MinRight(Units),
        "max-right": MaxRight(Units),

        "min-top": MinTop(Units),
        "max-top": MaxTop(Units),
        "min-height": MinHeight(Units),
        "max-height": MaxHeight(Units),
        "min-bottom": MinBottom(Units),
        "max-bottom": MaxBottom(Units),

        // Child Spacing
        "child-space": ChildSpace(Units),
        "child-left": ChildLeft(Units),
        "child-right": ChildRight(Units),
        "child-top": ChildTop(Units),
        "child-bottom": ChildBottom(Units),
        "row-between": RowBetween(Units),
        "col-between": ColBetween(Units),

        // Border Radius
        "border-radius": BorderRadius(BorderRadius),
        "border-top-left-radius": BorderTopLeftRadius(Length),
        "border-top-right-radius": BorderTopRightRadius(Length),
        "border-bottom-left-radius": BorderBottomLeftRadius(Length),
        "border-bottom-right-radius": BorderBottomRightRadius(Length),

        // Border Width
        "border-width": BorderWidth(Length),

        // Border Color
        "border-color": BorderColor(Color),

        // Border Shape
        "border-corner-shape": BorderCornerShape(BorderCornerShape),
        "border-top-left-shape": BorderTopLeftShape(BorderCornerShape),
        "border-top-right-shape": BorderTopRightShape(BorderCornerShape),
        "border-bottom-left-shape": BorderBottomLeftShape(BorderCornerShape),
        "border-bottom-right-shape": BorderBottomRightShape(BorderCornerShape),

        // Background
        "background-color": BackgroundColor(Color),
        "background-image": BackgroundImage(String),
        // // TODO
        // //BackgroundGradient(LinearGradient),

        // Font
        "font-size": FontSize(FontSize),
        "font-color": FontColor(Color),
        "font": Font(String),
        "selection-color": SelectionColor(Color),
        "caret-color": CaretColor(Color),
        "text-wrap": TextWrap(bool),

        // Shadow
        "outer-shadow": OuterShadow(BoxShadow),
        "outer-shadow-h-offset": OuterShadowHOffset(Length),
        "outer-shadow-v-offset": OuterShadowVOffset(Length),
        "outer-shadow-blur": OuterShadowBlur(Length),
        "outer-shadow-color": OuterShadowColor(Color),

        "inner-shadow": InnerShadow(BoxShadow),
        "inner-shadow-h-offset": InnerShadowHOffset(Length),
        "inner-shadow-v-offset": InnerShadowVOffset(Length),
        "iner-shadow-blur": InnerShadowBlur(Length),
        "inner-shadow-color": InnerShadowColor(Color),
        "transition": Transition(Vec<Transition>),
        "z-index": ZIndex(i32),

        // // TODO
        // // Translate((f32, f32)),
        // // Rotate(f32),
        // // Scale((f32, f32)),
        "cursor": Cursor(CursorIcon),
    }
}
