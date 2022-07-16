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
        // ----- Border -----

        // Border Shorthand
        // TODO: Support coloring and styling individual borders and enable this.
        // "border": Border(Border),

        // Border Color
        "border-color": BorderColor(BorderColor),
        // TODO: Support coloring individual borders.
        // "border-top-color": BorderTopColor(Color),
        // "border-right-color": BorderRightColor(Color),
        // "border-bottom-color": BorderBottomColor(Color),
        // "border-left-color": BorderLeftColor(Color),

        // Border Corner Shape
        "border-corner-shape": BorderCornerShape(BorderCornerShape),
        "border-top-left-shape": BorderTopLeftShape(BorderCornerShape),
        "border-top-right-shape": BorderTopRightShape(BorderCornerShape),
        "border-bottom-left-shape": BorderBottomLeftShape(BorderCornerShape),
        "border-bottom-right-shape": BorderBottomRightShape(BorderCornerShape),

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
        // Border Style
        // TODO: Support styling borders.
        // "border-style": BorderStyle(BorderStyle),
        // "border-top-style": BorderTopStyle(BorderStyleKeyword),
        // "border-right-style": BorderRightStyle(BorderStyleKeyword),
        // "border-bottom-style": BorderBottomStyle(BorderStyleKeyword),
        // "border-left-style": BorderLeftStyle(BorderStyleKeyword),

        // Border Width
        "border-width": BorderWidth(BorderWidth),
        "border-top-width": BorderTopWidth(BorderWidthValue),
        "border-right-width": BorderRightWidth(BorderWidthValue),
        "border-bottom-width": BorderBottomWidth(BorderWidthValue),
        "border-left-width": BorderLeftWidth(BorderWidthValue),


        // ----- Outline -----

        // Outline Shorthand
        // TODO: Support coloring and styling individual outlines.
        // "outline": Outline(Outline),

        // Outline Color
        "outline-color": OutlineColor(BorderColor),
        // TODO: Support coloring individual outlines.
        // "outline-top-color": OutlineTopColor(Color),
        // "outline-right-color": OutlineRightColor(Color),
        // "outline-bottom-color": OutlineBottomColor(Color),
        // "outline-left-color": OutlineLeftColor(Color),

        // Outline Style
        // TODO: Support styling outlines.
        // "outline-style": OutlineStyle(BorderStyle),
        // "outline-top-style": OutlineTopStyle(BorderStyleKeyword),
        // "outline-right-style": OutlineRightStyle(BorderStyleKeyword),
        // "outline-bottom-style": OutlineBottomStyle(BorderStyleKeyword),
        // "outline-left-style": OutlineLeftStyle(BorderStyleKeyword),

        // Outline Width
        "outline-width": OutlineWidth(BorderWidth),
        "outline-top-width": OutlineTopWidth(BorderWidthValue),
        "outline-right-width": OutlineRightWidth(BorderWidthValue),
        "outline-bottom-width": OutlineBottomWidth(BorderWidthValue),
        "outline-left-width": OutlineLeftWidth(BorderWidthValue),


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

#[cfg(test)]
mod tests {
    use cssparser::ParserInput;

    use super::*;

    #[test]
    fn parse_property() {
        let mut parser_input = ParserInput::new("red");
        let mut parser = Parser::new(&mut parser_input);
        let parsed_property = Property::parse_value("background-color", &mut parser);
        
        println!("{:?}", parsed_property);
    }
}
