use crate::Token;
use ast::*;

pub struct AstBuilder {}

fn expression_to_assignment_target(a0: Box<Expression>) -> AssignmentTarget {
    match *a0 {
        Expression::IdentifierExpression(IdentifierExpression { name }) => {
            AssignmentTarget::SimpleAssignmentTarget(
                SimpleAssignmentTarget::AssignmentTargetIdentifier(AssignmentTargetIdentifier {
                    name,
                }),
            )
        }
        Expression::MemberExpression(MemberExpression::StaticMemberExpression(
            StaticMemberExpression { object, property },
        )) => AssignmentTarget::SimpleAssignmentTarget(
            SimpleAssignmentTarget::MemberAssignmentTarget(
                MemberAssignmentTarget::StaticMemberAssignmentTarget(
                    StaticMemberAssignmentTarget::new(object, property),
                ),
            ),
        ),
        a0 => panic!("Unimplemented expression_to_assignment_target: {:?}", a0),
    }
}

fn expression_to_simple_assignment_target(a0: Box<Expression>) -> SimpleAssignmentTarget {
    match *a0 {
        Expression::IdentifierExpression(IdentifierExpression { name }) => {
            SimpleAssignmentTarget::AssignmentTargetIdentifier(AssignmentTargetIdentifier { name })
        }
        Expression::MemberExpression(MemberExpression::StaticMemberExpression(
            StaticMemberExpression { object, property },
        )) => SimpleAssignmentTarget::MemberAssignmentTarget(
            MemberAssignmentTarget::StaticMemberAssignmentTarget(
                StaticMemberAssignmentTarget::new(object, property),
            ),
        ),
        a0 => panic!(
            "Unimplemented expression_to_simple_assignment_target: {:?}",
            a0
        ),
    }
}

impl AstBuilder {
    // IdentifierReference ::= "Identifier" => IdentifierReference($0)
    pub fn identifier_reference(&self, a0: Box<Token>) -> Box<Identifier> {
        Box::new(self.identifier(a0))
    }
    // BindingIdentifier ::= "Identifier" => BindingIdentifier 0($0)
    pub fn binding_identifier_p0(&self, a0: Box<Token>) -> Box<BindingIdentifier> {
        Box::new(BindingIdentifier::new(self.identifier(a0)))
    }
    // BindingIdentifier ::= "yield" => BindingIdentifier 1($0)
    pub fn binding_identifier_p1(&self) -> Box<BindingIdentifier> {
        Box::new(BindingIdentifier::new(Identifier::new("yield".to_string())))
    }
    // BindingIdentifier ::= "await" => BindingIdentifier 2($0)
    pub fn binding_identifier_p2(&self) -> Box<BindingIdentifier> {
        Box::new(BindingIdentifier::new(Identifier::new("await".to_string())))
    }
    // LabelIdentifier ::= "Identifier" => LabelIdentifier($0)
    pub fn label_identifier(&self, a0: Box<Token>) -> Box<Label> {
        Box::new(Label::new(a0.value.unwrap()))
    }

    // PrimaryExpression : `this`
    pub fn this_expr(&self) -> Box<Expression> {
        Box::new(Expression::ThisExpression)
    }

    // PrimaryExpression : IdentifierReference
    pub fn identifier_expr(&self, name: Box<Identifier>) -> Box<Expression> {
        Box::new(Expression::IdentifierExpression(IdentifierExpression {
            name: *name,
        }))
    }

    // PrimaryExpression : RegularExpressionLiteral
    pub fn regexp_literal(&self, token: Box<Token>) -> Box<Expression> {
        let pattern: String = token.value.unwrap();
        let global: bool = false;
        let ignore_case: bool = false;
        let multi_line: bool = false;
        let sticky: bool = false;
        let unicode: bool = false;
        Box::new(Expression::LiteralRegExpExpression(
            LiteralRegExpExpression::new(pattern, global, ignore_case, multi_line, sticky, unicode),
        ))
    }

    // PrimaryExpression : TemplateLiteral
    pub fn untagged_template_expr(
        &self,
        template_literal: Box<TemplateExpression>,
    ) -> Box<Expression> {
        Box::new(Expression::TemplateExpression(*template_literal))
    }

    // PrimaryExpression ::= CoverParenthesizedExpressionAndArrowParameterList
    pub fn parenthesized_expr(&self, a0: Box<Expression>) -> Box<Expression> {
        // TODO
        a0
    }

    // CoverParenthesizedExpressionAndArrowParameterList ::= "(" Expression ")" => CoverParenthesizedExpressionAndArrowParameterList 0($0, $1, $2)
    pub fn cover_parenthesized_expression_and_arrow_parameter_list_p0(
        &self,
        a0: Box<Expression>,
    ) -> Box<Expression> {
        // TODO
        a0
    }
    // CoverParenthesizedExpressionAndArrowParameterList ::= "(" Expression "," ")" => CoverParenthesizedExpressionAndArrowParameterList 1($0, $1, $2, $3)
    pub fn cover_parenthesized_expression_and_arrow_parameter_list_p1(
        &self,
        a0: Box<Expression>,
    ) -> Box<Expression> {
        unimplemented!(); // Box::new(CoverParenthesizedExpressionAndArrowParameterList::new())
    }
    // CoverParenthesizedExpressionAndArrowParameterList ::= "(" ")" => CoverParenthesizedExpressionAndArrowParameterList 2($0, $1)
    pub fn cover_parenthesized_expression_and_arrow_parameter_list_p2(&self) -> Box<Expression> {
        unimplemented!(); // Box::new(CoverParenthesizedExpressionAndArrowParameterList::new())
    }
    // CoverParenthesizedExpressionAndArrowParameterList ::= "(" "..." BindingIdentifier ")" => CoverParenthesizedExpressionAndArrowParameterList 3($0, $1, $2, $3)
    pub fn cover_parenthesized_expression_and_arrow_parameter_list_p3(
        &self,
        a0: Box<BindingIdentifier>,
    ) -> Box<Expression> {
        unimplemented!(); // Box::new(CoverParenthesizedExpressionAndArrowParameterList::new())
    }
    // CoverParenthesizedExpressionAndArrowParameterList ::= "(" "..." BindingPattern ")" => CoverParenthesizedExpressionAndArrowParameterList 4($0, $1, $2, $3)
    pub fn cover_parenthesized_expression_and_arrow_parameter_list_p4(
        &self,
        a0: Box<BindingPattern>,
    ) -> Box<Expression> {
        unimplemented!(); // Box::new(CoverParenthesizedExpressionAndArrowParameterList::new())
    }
    // CoverParenthesizedExpressionAndArrowParameterList ::= "(" Expression "," "..." BindingIdentifier ")" => CoverParenthesizedExpressionAndArrowParameterList 5($0, $1, $2, $3, $4, $5)
    pub fn cover_parenthesized_expression_and_arrow_parameter_list_p5(
        &self,
        a0: Box<Expression>,
        a1: Box<BindingIdentifier>,
    ) -> Box<Expression> {
        unimplemented!(); // Box::new(CoverParenthesizedExpressionAndArrowParameterList::new())
    }
    // CoverParenthesizedExpressionAndArrowParameterList ::= "(" Expression "," "..." BindingPattern ")" => CoverParenthesizedExpressionAndArrowParameterList 6($0, $1, $2, $3, $4, $5)
    pub fn cover_parenthesized_expression_and_arrow_parameter_list_p6(
        &self,
        a0: Box<Expression>,
        a1: Box<BindingPattern>,
    ) -> Box<Expression> {
        unimplemented!(); // Box::new(CoverParenthesizedExpressionAndArrowParameterList::new())
    }

    // Literal : NullLiteral
    pub fn null_literal(&self) -> Box<Expression> {
        Box::new(Expression::LiteralNullExpression)
    }

    // Literal : BooleanLiteral
    pub fn boolean_literal(&self, token: Box<Token>) -> Box<Expression> {
        let s = token.value.unwrap();
        assert!(&s == "true" || &s == "false");

        Box::new(Expression::LiteralBooleanExpression(
            LiteralBooleanExpression {
                value: &s == "true",
            },
        ))
    }

    fn numeric_literal_value(token: Box<Token>) -> f64 {
        let s = token.value.unwrap();

        // BUG: Not all syntax is supported yet.
        s.parse::<f64>().unwrap_or(std::f64::NAN)
    }

    // Literal : NumericLiteral
    pub fn numeric_literal(&self, token: Box<Token>) -> Box<Expression> {
        Box::new(Expression::LiteralNumericExpression(
            LiteralNumericExpression::new(Self::numeric_literal_value(token)),
        ))
    }

    fn string_literal_value(token: Box<Token>) -> String {
        token.value.unwrap()
    }

    // Literal : StringLiteral
    pub fn string_literal(&self, token: Box<Token>) -> Box<Expression> {
        Box::new(Expression::LiteralStringExpression(
            LiteralStringExpression {
                value: Self::string_literal_value(token),
            },
        ))
    }

    // ArrayLiteral : `[` Elision? `]`
    pub fn array_literal_empty(&self, elision: Option<Box<ArrayExpression>>) -> Box<Expression> {
        Box::new(Expression::ArrayExpression(match elision {
            None => ArrayExpression { elements: vec![] },
            Some(array) => *array,
        }))
    }

    // ArrayLiteral : `[` ElementList `]`
    pub fn array_literal(&self, array: Box<ArrayExpression>) -> Box<Expression> {
        Box::new(Expression::ArrayExpression(*array))
    }

    // ArrayLiteral : `[` ElementList `,` Elision? `]`
    pub fn array_literal_with_trailing_elision(
        &self,
        mut array: Box<ArrayExpression>,
        elision: Option<Box<ArrayExpression>>,
    ) -> Box<Expression> {
        if let Some(mut more) = elision {
            array.elements.append(&mut more.elements);
        }
        Box::new(Expression::ArrayExpression(*array))
    }

    // ElementList : Elision? AssignmentExpression
    pub fn element_list_first(
        &self,
        elision: Option<Box<ArrayExpression>>,
        element: Box<Expression>,
    ) -> Box<ArrayExpression> {
        let mut array = elision.unwrap_or_else(|| Box::new(ArrayExpression { elements: vec![] }));
        array
            .elements
            .push(ArrayExpressionElement::Expression(element));
        array
    }

    // ElementList : Elision? SpreadElement
    pub fn element_list_first_spread(
        &self,
        elision: Option<Box<ArrayExpression>>,
        spread_element: Box<Expression>,
    ) -> Box<ArrayExpression> {
        let mut array = elision.unwrap_or_else(|| Box::new(ArrayExpression { elements: vec![] }));
        array
            .elements
            .push(ArrayExpressionElement::SpreadElement(spread_element));
        array
    }

    // ElementList : ElementList `,` Elision? AssignmentExpression
    pub fn element_list_append(
        &self,
        mut array: Box<ArrayExpression>,
        elision: Option<Box<ArrayExpression>>,
        element: Box<Expression>,
    ) -> Box<ArrayExpression> {
        if let Some(mut elision) = elision {
            array.elements.append(&mut elision.elements);
        }
        array
            .elements
            .push(ArrayExpressionElement::Expression(element));
        array
    }

    // ElementList : ElementList `,` Elision? SpreadElement
    pub fn element_list_append_spread(
        &self,
        mut array: Box<ArrayExpression>,
        elision: Option<Box<ArrayExpression>>,
        spread_element: Box<Expression>,
    ) -> Box<ArrayExpression> {
        if let Some(mut elision) = elision {
            array.elements.append(&mut elision.elements);
        }
        array
            .elements
            .push(ArrayExpressionElement::SpreadElement(spread_element));
        array
    }

    // Elision : `,`
    pub fn elision_single(&self) -> Box<ArrayExpression> {
        Box::new(ArrayExpression {
            elements: vec![ArrayExpressionElement::Elision],
        })
    }

    // Elision : Elision `,`
    pub fn elision_append(&self, mut array: Box<ArrayExpression>) -> Box<ArrayExpression> {
        array.elements.push(ArrayExpressionElement::Elision);
        array
    }

    // SpreadElement : `...` AssignmentExpression
    pub fn spread_element(&self, expr: Box<Expression>) -> Box<Expression> {
        expr
    }

    // ObjectLiteral : `{` `}`
    pub fn object_literal_empty(&self) -> Box<Expression> {
        Box::new(Expression::ObjectExpression(ObjectExpression::new(vec![])))
    }

    // ObjectLiteral : `{` PropertyDefinitionList `}`
    // ObjectLiteral : `{` PropertyDefinitionList `,` `}`
    pub fn object_literal(&self, object: Box<ObjectExpression>) -> Box<Expression> {
        Box::new(Expression::ObjectExpression(*object))
    }

    // PropertyDefinitionList : PropertyDefinition
    pub fn property_definition_list_single(
        &self,
        property: Box<ObjectProperty>,
    ) -> Box<ObjectExpression> {
        Box::new(ObjectExpression::new(vec![property]))
    }

    // PropertyDefinitionList : PropertyDefinitionList `,` PropertyDefinition
    pub fn property_definition_list_append(
        &self,
        mut object: Box<ObjectExpression>,
        property: Box<ObjectProperty>,
    ) -> Box<ObjectExpression> {
        object.properties.push(property);
        object
    }

    // PropertyDefinition : IdentifierReference
    pub fn shorthand_property(&self, name: Box<Identifier>) -> Box<ObjectProperty> {
        Box::new(ObjectProperty::ShorthandProperty(ShorthandProperty {
            name: IdentifierExpression { name: *name },
        }))
    }

    // PropertyDefinition : CoverInitializedName
    pub fn property_definition_cover(&self, a0: Box<Void>) -> Box<ObjectProperty> {
        // Awkward. This needs to be stored somehow until we reach an enclosing thing.
        unimplemented!();
    }

    // PropertyDefinition : PropertyName `:` AssignmentExpression
    pub fn property_definition(
        &self,
        name: Box<PropertyName>,
        expression: Box<Expression>,
    ) -> Box<ObjectProperty> {
        Box::new(ObjectProperty::NamedObjectProperty(
            NamedObjectProperty::DataProperty(DataProperty {
                property_name: *name,
                expression,
            }),
        ))
    }

    // PropertyDefinition : MethodDefinition
    pub fn property_definition_method(&self, method: Box<MethodDefinition>) -> Box<ObjectProperty> {
        Box::new(ObjectProperty::NamedObjectProperty(
            NamedObjectProperty::MethodDefinition(*method),
        ))
    }

    // PropertyDefinition : `...` AssignmentExpression
    pub fn property_definition_spread(&self, spread: Box<Expression>) -> Box<ObjectProperty> {
        unimplemented!();
    }

    // LiteralPropertyName : IdentifierName
    pub fn property_name_identifier(&self, token: Box<Token>) -> Box<PropertyName> {
        Box::new(PropertyName::StaticPropertyName(StaticPropertyName {
            value: token.value.unwrap(),
        }))
    }

    // LiteralPropertyName : StringLiteral
    pub fn property_name_string(&self, token: Box<Token>) -> Box<PropertyName> {
        Box::new(PropertyName::StaticPropertyName(StaticPropertyName {
            value: Self::string_literal_value(token),
        }))
    }

    // LiteralPropertyName : NumericLiteral
    pub fn property_name_numeric(&self, token: Box<Token>) -> Box<PropertyName> {
        Box::new(PropertyName::StaticPropertyName(StaticPropertyName {
            value: format!("{:?}", Self::numeric_literal_value(token)),
        }))
    }

    // ComputedPropertyName ::= "[" AssignmentExpression "]" => ComputedPropertyName($0, $1, $2)
    pub fn computed_property_name(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ComputedPropertyName::new())
    }
    // CoverInitializedName ::= IdentifierReference Initializer => CoverInitializedName($0, $1)
    pub fn cover_initialized_name(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(CoverInitializedName::new())
    }
    // TemplateLiteral ::= NoSubstitutionTemplate => TemplateLiteral 0($0)
    pub fn template_literal_p0(&self, a0: Box<Void>) -> Box<TemplateExpression> {
        unimplemented!(); // Box::new(TemplateLiteral::new())
    }
    // TemplateLiteral ::= SubstitutionTemplate => TemplateLiteral 1($0)
    pub fn template_literal_p1(&self, a0: Box<Void>) -> Box<TemplateExpression> {
        unimplemented!(); // Box::new(TemplateLiteral::new())
    }
    // SubstitutionTemplate ::= "TemplateHead" Expression TemplateSpans => SubstitutionTemplate($0, $1, $2)
    pub fn substitution_template(&self, a0: Box<Void>, a1: Box<Void>, a2: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(SubstitutionTemplate::new())
    }
    // TemplateSpans ::= "TemplateTail" => TemplateSpans 0($0)
    pub fn template_spans_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(TemplateSpans::new())
    }
    // TemplateSpans ::= TemplateMiddleList "TemplateTail" => TemplateSpans 1($0, $1)
    pub fn template_spans_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(TemplateSpans::new())
    }
    // TemplateMiddleList ::= "TemplateMiddle" Expression => TemplateMiddleList 0($0, $1)
    pub fn template_middle_list_p0(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(TemplateMiddleList::new())
    }
    // TemplateMiddleList ::= TemplateMiddleList "TemplateMiddle" Expression => TemplateMiddleList 1($0, $1, $2)
    pub fn template_middle_list_p1(
        &self,
        a0: Box<Void>,
        a1: Box<Void>,
        a2: Box<Void>,
    ) -> Box<Void> {
        unimplemented!(); // Box::new(TemplateMiddleList::new())
    }

    // MemberExpression : MemberExpression `[` Expression `]`
    // CallExpression : CallExpression `[` Expression `]`
    pub fn computed_member_expr(
        &self,
        object: Box<Expression>,
        expression: Box<Expression>,
    ) -> Box<Expression> {
        Box::new(Expression::MemberExpression(
            MemberExpression::ComputedMemberExpression(ComputedMemberExpression::new(
                ExpressionOrSuper::Expression(object),
                expression,
            )),
        ))
    }

    fn identifier(&self, token: Box<Token>) -> Identifier {
        Identifier::new(token.value.unwrap())
    }

    fn identifier_name(&self, token: Box<Token>) -> IdentifierName {
        IdentifierName::new(token.value.unwrap())
    }

    // MemberExpression : MemberExpression `.` IdentifierName
    // CallExpression : CallExpression `.` IdentifierName
    pub fn static_member_expr(
        &self,
        object: Box<Expression>,
        identifier_token: Box<Token>,
    ) -> Box<Expression> {
        Box::new(Expression::MemberExpression(
            MemberExpression::StaticMemberExpression(StaticMemberExpression::new(
                ExpressionOrSuper::Expression(object),
                self.identifier_name(identifier_token),
            )),
        ))
    }

    // MemberExpression : MemberExpression TemplateLiteral
    // CallExpression : CallExpression TemplateLiteral
    pub fn tagged_template_expr(
        &self,
        tag: Box<Expression>,
        mut template_literal: Box<TemplateExpression>,
    ) -> Box<Expression> {
        template_literal.tag = Some(tag);
        Box::new(Expression::TemplateExpression(*template_literal))
    }

    // MemberExpression : `new` MemberExpression Arguments
    pub fn new_expr_with_arguments(
        &self,
        callee: Box<Expression>,
        arguments: Box<Arguments>,
    ) -> Box<Expression> {
        Box::new(Expression::NewExpression(NewExpression {
            callee,
            arguments: *arguments,
        }))
    }

    // SuperProperty : `super` `[` Expression `]`
    pub fn super_property_computed(&self, expression: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::MemberExpression(
            MemberExpression::ComputedMemberExpression(ComputedMemberExpression {
                object: ExpressionOrSuper::Super,
                expression: expression,
            }),
        ))
    }

    // SuperProperty : `super` `.` IdentifierName
    pub fn super_property_static(&self, identifier_token: Box<Token>) -> Box<Expression> {
        Box::new(Expression::MemberExpression(
            MemberExpression::StaticMemberExpression(StaticMemberExpression {
                object: ExpressionOrSuper::Super,
                property: self.identifier_name(identifier_token),
            }),
        ))
    }

    // NewTarget : `new` `.` `target`
    pub fn new_target_expr(&self) -> Box<Expression> {
        return Box::new(Expression::NewTargetExpression);
    }

    // NewExpression : `new` NewExpression
    pub fn new_expr_without_arguments(&self, callee: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::NewExpression(NewExpression {
            callee,
            arguments: Arguments::new(vec![]),
        }))
    }

    // CallExpression : CallExpression Arguments
    // CoverCallExpressionAndAsyncArrowHead : MemberExpression Arguments
    // CallMemberExpression : MemberExpression Arguments
    pub fn call_expr(&self, callee: Box<Expression>, arguments: Box<Arguments>) -> Box<Expression> {
        Box::new(Expression::CallExpression(CallExpression {
            callee: ExpressionOrSuper::Expression(callee),
            arguments: *arguments,
        }))
    }

    // SuperCall : `super` Arguments
    pub fn super_call(&self, arguments: Box<Arguments>) -> Box<Expression> {
        Box::new(Expression::CallExpression(CallExpression {
            callee: ExpressionOrSuper::Super,
            arguments: *arguments,
        }))
    }

    // Arguments : `(` `)`
    pub fn arguments_empty(&self) -> Box<Arguments> {
        Box::new(Arguments::new(Vec::new()))
    }

    // ArgumentList : AssignmentExpression
    // ArgumentList : ArgumentList `,` AssignmentExpression
    pub fn arguments_append(
        &self,
        mut arguments: Box<Arguments>,
        expression: Box<Expression>,
    ) -> Box<Arguments> {
        arguments.args.push(Argument::Expression(expression));
        arguments
    }

    // ArgumentList : `...` AssignmentExpression
    // ArgumentList : ArgumentList `,` `...` AssignmentExpression
    pub fn arguments_append_spread(
        &self,
        mut arguments: Box<Arguments>,
        expression: Box<Expression>,
    ) -> Box<Arguments> {
        arguments.args.push(Argument::SpreadElement(expression));
        arguments
    }

    // UpdateExpression : LeftHandSideExpression `++`
    pub fn post_increment_expr(&self, operand: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::UpdateExpression(UpdateExpression::new(
            false,
            UpdateOperator::Increment,
            expression_to_simple_assignment_target(operand),
        )))
    }

    // UpdateExpression : LeftHandSideExpression `--`
    pub fn post_decrement_expr(&self, operand: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::UpdateExpression(UpdateExpression::new(
            false,
            UpdateOperator::Decrement,
            expression_to_simple_assignment_target(operand),
        )))
    }

    // UpdateExpression : `++` UnaryExpression
    pub fn pre_increment_expr(&self, operand: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::UpdateExpression(UpdateExpression::new(
            true,
            UpdateOperator::Increment,
            expression_to_simple_assignment_target(operand),
        )))
    }

    // UpdateExpression : `--` UnaryExpression
    pub fn pre_decrement_expr(&self, operand: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::UpdateExpression(UpdateExpression::new(
            true,
            UpdateOperator::Decrement,
            expression_to_simple_assignment_target(operand),
        )))
    }

    // UnaryExpression : `delete` UnaryExpression
    pub fn delete_expr(&self, operand: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::UnaryExpression(UnaryExpression::new(
            UnaryOperator::Delete,
            operand,
        )))
    }

    // UnaryExpression : `void` UnaryExpression
    pub fn void_expr(&self, operand: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::UnaryExpression(UnaryExpression::new(
            UnaryOperator::Void,
            operand,
        )))
    }

    // UnaryExpression : `typeof` UnaryExpression
    pub fn typeof_expr(&self, operand: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::UnaryExpression(UnaryExpression::new(
            UnaryOperator::Typeof,
            operand,
        )))
    }

    // UnaryExpression : `+` UnaryExpression
    pub fn unary_plus_expr(&self, operand: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::UnaryExpression(UnaryExpression::new(
            UnaryOperator::Plus,
            operand,
        )))
    }

    // UnaryExpression : `-` UnaryExpression
    pub fn unary_minus_expr(&self, operand: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::UnaryExpression(UnaryExpression::new(
            UnaryOperator::Minus,
            operand,
        )))
    }

    // UnaryExpression : `~` UnaryExpression
    pub fn bitwise_not_expr(&self, operand: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::UnaryExpression(UnaryExpression::new(
            UnaryOperator::BitwiseNot,
            operand,
        )))
    }

    // UnaryExpression : `!` UnaryExpression
    pub fn logical_not_expr(&self, operand: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::UnaryExpression(UnaryExpression::new(
            UnaryOperator::LogicalNot,
            operand,
        )))
    }

    pub fn equals_op(&self) -> BinaryOperator {
        BinaryOperator::Equals
    }
    pub fn not_equals_op(&self) -> BinaryOperator {
        BinaryOperator::NotEquals
    }
    pub fn strict_equals_op(&self) -> BinaryOperator {
        BinaryOperator::StrictEquals
    }
    pub fn strict_not_equals_op(&self) -> BinaryOperator {
        BinaryOperator::StrictNotEquals
    }
    pub fn less_than_op(&self) -> BinaryOperator {
        BinaryOperator::LessThan
    }
    pub fn less_than_or_equal_op(&self) -> BinaryOperator {
        BinaryOperator::LessThanOrEqual
    }
    pub fn greater_than_op(&self) -> BinaryOperator {
        BinaryOperator::GreaterThan
    }
    pub fn greater_than_or_equal_op(&self) -> BinaryOperator {
        BinaryOperator::GreaterThanOrEqual
    }
    pub fn in_op(&self) -> BinaryOperator {
        BinaryOperator::In
    }
    pub fn instanceof_op(&self) -> BinaryOperator {
        BinaryOperator::Instanceof
    }
    pub fn left_shift_op(&self) -> BinaryOperator {
        BinaryOperator::LeftShift
    }
    pub fn right_shift_op(&self) -> BinaryOperator {
        BinaryOperator::RightShift
    }
    pub fn right_shift_ext_op(&self) -> BinaryOperator {
        BinaryOperator::RightShiftExt
    }
    pub fn add_op(&self) -> BinaryOperator {
        BinaryOperator::Add
    }
    pub fn sub_op(&self) -> BinaryOperator {
        BinaryOperator::Sub
    }
    pub fn mul_op(&self) -> BinaryOperator {
        BinaryOperator::Mul
    }
    pub fn div_op(&self) -> BinaryOperator {
        BinaryOperator::Div
    }
    pub fn mod_op(&self) -> BinaryOperator {
        BinaryOperator::Mod
    }
    pub fn pow_op(&self) -> BinaryOperator {
        BinaryOperator::Pow
    }
    pub fn comma_op(&self) -> BinaryOperator {
        BinaryOperator::Comma
    }
    pub fn logical_or_op(&self) -> BinaryOperator {
        BinaryOperator::LogicalOr
    }
    pub fn logical_and_op(&self) -> BinaryOperator {
        BinaryOperator::LogicalAnd
    }
    pub fn bitwise_or_op(&self) -> BinaryOperator {
        BinaryOperator::BitwiseOr
    }
    pub fn bitwise_xor_op(&self) -> BinaryOperator {
        BinaryOperator::BitwiseXor
    }
    pub fn bitwise_and_op(&self) -> BinaryOperator {
        BinaryOperator::BitwiseAnd
    }

    // Due to limitations of the current parser generator,
    // MultiplicativeOperators and CompoundAssignmentOperators currently get
    // boxed.
    pub fn box_op(&self, op: BinaryOperator) -> Box<BinaryOperator> {
        Box::new(op)
    }

    // MultiplicativeExpression : MultiplicativeExpression MultiplicativeOperator ExponentiationExpression
    pub fn multiplicative_expr(
        &self,
        left: Box<Expression>,
        operator: Box<BinaryOperator>,
        right: Box<Expression>,
    ) -> Box<Expression> {
        self.binary_expr(*operator, left, right)
    }

    // ExponentiationExpression : UpdateExpression `**` ExponentiationExpression
    // AdditiveExpression : AdditiveExpression `+` MultiplicativeExpression
    // AdditiveExpression : AdditiveExpression `-` MultiplicativeExpression
    // ShiftExpression : ShiftExpression `<<` AdditiveExpression
    // ShiftExpression : ShiftExpression `>>` AdditiveExpression
    // ShiftExpression : ShiftExpression `>>>` AdditiveExpression
    // RelationalExpression : RelationalExpression `<` ShiftExpression
    // RelationalExpression : RelationalExpression `>` ShiftExpression
    // RelationalExpression : RelationalExpression `<=` ShiftExpression
    // RelationalExpression : RelationalExpression `>=` ShiftExpression
    // RelationalExpression : RelationalExpression `instanceof` ShiftExpression
    // RelationalExpression : [+In] RelationalExpression `in` ShiftExpression
    // EqualityExpression : EqualityExpression `==` RelationalExpression
    // EqualityExpression : EqualityExpression `!=` RelationalExpression
    // EqualityExpression : EqualityExpression `===` RelationalExpression
    // EqualityExpression : EqualityExpression `!==` RelationalExpression
    // BitwiseANDExpression : BitwiseANDExpression `&` EqualityExpression
    // BitwiseXORExpression : BitwiseXORExpression `^` BitwiseANDExpression
    // BitwiseORExpression : BitwiseORExpression `|` BitwiseXORExpression
    // LogicalANDExpression : LogicalANDExpression `&&` BitwiseORExpression
    // LogicalORExpression : LogicalORExpression `||` LogicalANDExpression
    pub fn binary_expr(
        &self,
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    ) -> Box<Expression> {
        Box::new(Expression::BinaryExpression(BinaryExpression {
            operator,
            left,
            right,
        }))
    }

    // ConditionalExpression : LogicalORExpression `?` AssignmentExpression `:` AssignmentExpression
    pub fn conditional_expr(
        &self,
        test: Box<Expression>,
        consequent: Box<Expression>,
        alternate: Box<Expression>,
    ) -> Box<Expression> {
        Box::new(Expression::ConditionalExpression(ConditionalExpression {
            test,
            consequent,
            alternate,
        }))
    }

    // AssignmentExpression : LeftHandSideExpression `=` AssignmentExpression
    pub fn assignment_expr(
        &self,
        left_hand_side: Box<Expression>,
        value: Box<Expression>,
    ) -> Box<Expression> {
        let target = expression_to_assignment_target(left_hand_side);
        Box::new(Expression::AssignmentExpression(AssignmentExpression::new(
            target, value,
        )))
    }

    pub fn add_assign_op(&self) -> CompoundAssignmentOperator {
        CompoundAssignmentOperator::Add
    }
    pub fn sub_assign_op(&self) -> CompoundAssignmentOperator {
        CompoundAssignmentOperator::Sub
    }
    pub fn mul_assign_op(&self) -> CompoundAssignmentOperator {
        CompoundAssignmentOperator::Mul
    }
    pub fn div_assign_op(&self) -> CompoundAssignmentOperator {
        CompoundAssignmentOperator::Div
    }
    pub fn mod_assign_op(&self) -> CompoundAssignmentOperator {
        CompoundAssignmentOperator::Mod
    }
    pub fn pow_assign_op(&self) -> CompoundAssignmentOperator {
        CompoundAssignmentOperator::Pow
    }
    pub fn left_shift_assign_op(&self) -> CompoundAssignmentOperator {
        CompoundAssignmentOperator::LeftShift
    }
    pub fn right_shift_assign_op(&self) -> CompoundAssignmentOperator {
        CompoundAssignmentOperator::RightShift
    }
    pub fn right_shift_ext_assign_op(&self) -> CompoundAssignmentOperator {
        CompoundAssignmentOperator::RightShiftExt
    }
    pub fn bitwise_or_assign_op(&self) -> CompoundAssignmentOperator {
        CompoundAssignmentOperator::Or
    }
    pub fn bitwise_xor_assign_op(&self) -> CompoundAssignmentOperator {
        CompoundAssignmentOperator::Xor
    }
    pub fn bitwise_and_assign_op(&self) -> CompoundAssignmentOperator {
        CompoundAssignmentOperator::And
    }

    pub fn box_assign_op(&self, op: CompoundAssignmentOperator) -> Box<CompoundAssignmentOperator> {
        Box::new(op)
    }

    // AssignmentExpression : LeftHandSideExpression AssignmentOperator AssignmentExpression
    pub fn compound_assignment_expr(
        &self,
        left_hand_side: Box<Expression>,
        operator: Box<CompoundAssignmentOperator>,
        value: Box<Expression>,
    ) -> Box<Expression> {
        let target = expression_to_simple_assignment_target(left_hand_side);
        Box::new(Expression::CompoundAssignmentExpression(
            CompoundAssignmentExpression::new(*operator, target, value),
        ))
    }

    // BlockStatement : Block
    pub fn block_statement(&self, a0: Box<Block>) -> Box<Statement> {
        Box::new(Statement::BlockStatement(BlockStatement::new(*a0)))
    }

    // Block : `{` StatementList? `}`
    pub fn block(&self, statements: Option<Box<Vec<Statement>>>) -> Box<Block> {
        Box::new(Block {
            statements: match statements {
                Some(statements) => *statements,
                None => vec![],
            },
            declarations: None,
        })
    }

    // StatementList : StatementListItem
    pub fn statement_list_single(&self, statement: Box<Statement>) -> Box<Vec<Statement>> {
        Box::new(vec![*statement])
    }

    // StatementList : StatementList StatementListItem
    pub fn statement_list_append(
        &self,
        mut list: Box<Vec<Statement>>,
        statement: Box<Statement>,
    ) -> Box<Vec<Statement>> {
        list.push(*statement);
        list
    }

    // LexicalDeclaration : LetOrConst BindingList `;`
    pub fn lexical_declaration(
        &self,
        a0: Box<VariableDeclarationKind>,
        a1: Box<Vec<VariableDeclarator>>,
    ) -> Box<Statement> {
        Box::new(Statement::VariableDeclarationStatement(
            VariableDeclaration::new(*a0, *a1),
        ))
    }

    // ForLexicalDeclaration : LetOrConst BindingList `;`
    pub fn for_lexical_declaration(
        &self,
        kind: Box<VariableDeclarationKind>,
        declarators: Box<Vec<VariableDeclarator>>,
    ) -> Box<VariableDeclarationOrExpression> {
        Box::new(VariableDeclarationOrExpression::VariableDeclaration(
            VariableDeclaration {
                kind: *kind,
                declarators: *declarators,
            },
        ))
    }

    // LetOrConst : `let`
    pub fn let_kind(&self) -> Box<VariableDeclarationKind> {
        Box::new(VariableDeclarationKind::Let)
    }

    // LetOrConst : `const`
    pub fn const_kind(&self) -> Box<VariableDeclarationKind> {
        Box::new(VariableDeclarationKind::Const)
    }

    // LexicalBinding ::= BindingIdentifier Initializer => LexicalBinding 0($0, Some($1))
    pub fn lexical_binding_p0(
        &self,
        a0: Box<BindingIdentifier>,
        a1: Option<Box<Expression>>,
    ) -> Box<VariableDeclarator> {
        Box::new(VariableDeclarator::new(Binding::BindingIdentifier(*a0), a1))
    }
    // LexicalBinding ::= BindingPattern Initializer => LexicalBinding 1($0, $1)
    pub fn lexical_binding_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(LexicalBinding::new())
    }

    // VariableStatement : `var` VariableDeclarationList `;`
    pub fn variable_statement(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }

    // VariableDeclarationList : VariableDeclaration
    // BindingList : LexicalBinding
    pub fn variable_declaration_list_single(
        &self,
        decl: Box<VariableDeclarator>,
    ) -> Box<Vec<VariableDeclarator>> {
        Box::new(vec![*decl])
    }

    // VariableDeclarationList : VariableDeclarationList `,` VariableDeclaration
    // BindingList : BindingList `,` LexicalBinding
    pub fn variable_declaration_list_append(
        &self,
        mut list: Box<Vec<VariableDeclarator>>,
        decl: Box<VariableDeclarator>,
    ) -> Box<Vec<VariableDeclarator>> {
        list.push(*decl);
        list
    }

    // VariableDeclaration : BindingIdentifier Initializer?
    // VariableDeclaration : BindingPattern Initializer
    pub fn variable_declaration(
        &self,
        binding: Box<Binding>,
        init: Option<Box<Expression>>,
    ) -> Box<VariableDeclarator> {
        Box::new(VariableDeclarator {
            binding: *binding,
            init,
        })
    }

    // BindingPattern ::= ObjectBindingPattern => BindingPattern 0($0)
    pub fn binding_pattern_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(BindingPattern::new())
    }
    // BindingPattern ::= ArrayBindingPattern => BindingPattern 1($0)
    pub fn binding_pattern_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(BindingPattern::new())
    }
    // ObjectBindingPattern ::= "{" "}" => ObjectBindingPattern 0($0, $1)
    pub fn object_binding_pattern_p0(&self) -> Box<Void> {
        unimplemented!(); // Box::new(ObjectBindingPattern::new())
    }
    // ObjectBindingPattern ::= "{" BindingRestProperty "}" => ObjectBindingPattern 1($0, $1, $2)
    pub fn object_binding_pattern_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ObjectBindingPattern::new())
    }
    // ObjectBindingPattern ::= "{" BindingPropertyList "}" => ObjectBindingPattern 2($0, $1, $2)
    pub fn object_binding_pattern_p2(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ObjectBindingPattern::new())
    }
    // ObjectBindingPattern ::= "{" BindingPropertyList "," BindingRestProperty "}" => ObjectBindingPattern 3($0, $1, $2, Some($3), $4)
    pub fn object_binding_pattern_p3(&self, a0: Box<Void>, a1: Option<Box<Void>>) -> Box<Void> {
        unimplemented!(); // Box::new(ObjectBindingPattern::new())
    }
    // ArrayBindingPattern ::= "[" Elision BindingRestElement "]" => ArrayBindingPattern 0($0, Some($1), Some($2), $3)
    pub fn array_binding_pattern_p0(
        &self,
        a0: Option<Box<Void>>,
        a1: Option<Box<Void>>,
    ) -> Box<Void> {
        unimplemented!(); // Box::new(ArrayBindingPattern::new())
    }
    // ArrayBindingPattern ::= "[" BindingElementList "]" => ArrayBindingPattern 1($0, $1, $2)
    pub fn array_binding_pattern_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ArrayBindingPattern::new())
    }
    // ArrayBindingPattern ::= "[" BindingElementList "," Elision BindingRestElement "]" => ArrayBindingPattern 2($0, $1, $2, Some($3), Some($4), $5)
    pub fn array_binding_pattern_p2(
        &self,
        a0: Box<Void>,
        a1: Option<Box<Void>>,
        a2: Option<Box<Void>>,
    ) -> Box<Void> {
        unimplemented!(); // Box::new(ArrayBindingPattern::new())
    }
    // BindingRestProperty ::= "..." BindingIdentifier => BindingRestProperty($0, $1)
    pub fn binding_rest_property(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(BindingRestProperty::new())
    }
    // BindingPropertyList ::= BindingProperty => BindingPropertyList 0($0)
    pub fn binding_property_list_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(BindingPropertyList::new())
    }
    // BindingPropertyList ::= BindingPropertyList "," BindingProperty => BindingPropertyList 1($0, $1, $2)
    pub fn binding_property_list_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(BindingPropertyList::new())
    }
    // BindingElementList ::= BindingElisionElement => BindingElementList 0($0)
    pub fn binding_element_list_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(BindingElementList::new())
    }
    // BindingElementList ::= BindingElementList "," BindingElisionElement => BindingElementList 1($0, $1, $2)
    pub fn binding_element_list_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(BindingElementList::new())
    }
    // BindingElisionElement ::= Elision BindingElement => BindingElisionElement(Some($0), $1)
    pub fn binding_elision_element(&self, a0: Option<Box<Void>>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(BindingElisionElement::new())
    }
    // BindingProperty ::= SingleNameBinding => BindingProperty 0($0)
    pub fn binding_property_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(BindingProperty::new())
    }
    // BindingProperty ::= PropertyName ":" BindingElement => BindingProperty 1($0, $1, $2)
    pub fn binding_property_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(BindingProperty::new())
    }
    // BindingElement ::= SingleNameBinding => BindingElement 0($0)
    pub fn binding_element_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(BindingElement::new())
    }
    // BindingElement ::= BindingPattern Initializer => BindingElement 1($0, Some($1))
    pub fn binding_element_p1(&self, a0: Box<Void>, a1: Option<Box<Void>>) -> Box<Void> {
        unimplemented!(); // Box::new(BindingElement::new())
    }
    // SingleNameBinding ::= BindingIdentifier Initializer => SingleNameBinding($0, Some($1))
    pub fn single_name_binding(&self, a0: Box<Void>, a1: Option<Box<Void>>) -> Box<Void> {
        unimplemented!(); // Box::new(SingleNameBinding::new())
    }
    // BindingRestElement ::= "..." BindingIdentifier => BindingRestElement 0($0, $1)
    pub fn binding_rest_element_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(BindingRestElement::new())
    }
    // BindingRestElement ::= "..." BindingPattern => BindingRestElement 1($0, $1)
    pub fn binding_rest_element_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(BindingRestElement::new())
    }

    // EmptyStatement : `;`
    pub fn empty_statement(&self) -> Box<Statement> {
        Box::new(Statement::EmptyStatement)
    }

    // ExpressionStatement : [lookahead not in {'{', 'function', 'async', 'class', 'let'}] Expression `;`
    pub fn expression_statement(&self, a0: Box<Expression>) -> Box<Statement> {
        Box::new(Statement::ExpressionStatement(a0))
    }

    // IfStatement : `if` `(` Expression `)` Statement `else` Statement
    // IfStatement : `if` `(` Expression `)` Statement
    pub fn if_statement(
        &self,
        test: Box<Expression>,
        consequent: Box<Statement>,
        alternate: Option<Box<Statement>>,
    ) -> Box<Statement> {
        Box::new(Statement::IfStatement(IfStatement {
            test,
            consequent,
            alternate,
        }))
    }

    // IterationStatement : `do` Statement `while` `(` Expression `)` `;`
    pub fn do_while_statement(
        &self,
        block: Box<Statement>,
        test: Box<Expression>,
    ) -> Box<Statement> {
        Box::new(Statement::DoWhileStatement(DoWhileStatement {
            block,
            test,
        }))
    }

    // IterationStatement : `while` `(` Expression `)` Statement
    pub fn while_statement(&self, test: Box<Expression>, block: Box<Statement>) -> Box<Statement> {
        Box::new(Statement::WhileStatement(WhileStatement { test, block }))
    }

    // IterationStatement : `for` `(` [lookahead != 'let'] Expression? `;` Expression? `;` Expression? `)` Statement
    // IterationStatement : `for` `(` `var` VariableDeclarationList `;` Expression? `;` Expression? `)` Statement
    // IterationStatement : `for` `(` ForLexicalDeclaration Expression? `;` Expression? `)` Statement
    pub fn for_statement(
        &self,
        init: Option<VariableDeclarationOrExpression>,
        test: Option<Box<Expression>>,
        update: Option<Box<Expression>>,
        block: Box<Statement>,
    ) -> Box<Statement> {
        Box::new(Statement::ForStatement(ForStatement {
            init,
            test,
            update,
            block,
        }))
    }

    pub fn for_expression(
        &self,
        expr: Option<Box<Expression>>,
    ) -> Option<VariableDeclarationOrExpression> {
        expr.map(|expr| VariableDeclarationOrExpression::Expression(expr))
    }

    pub fn for_var_declaration(
        &self,
        declarators: Box<Vec<VariableDeclarator>>,
    ) -> VariableDeclarationOrExpression {
        VariableDeclarationOrExpression::VariableDeclaration(VariableDeclaration {
            kind: VariableDeclarationKind::Var,
            declarators: *declarators,
        })
    }

    pub fn unbox_for_lexical_declaration(
        &self,
        declaration: Box<VariableDeclarationOrExpression>,
    ) -> VariableDeclarationOrExpression {
        *declaration
    }

    // IterationStatement : `for` `(` [lookahead != 'let'] LeftHandSideExpression `in` Expression `)` Statement
    // IterationStatement : `for` `(` `var` ForBinding `in` Expression `)` Statement
    // IterationStatement : `for` `(` ForDeclaration `in` Expression `)` Statement
    pub fn for_in_statement(
        &self,
        left: VariableDeclarationOrAssignmentTarget,
        right: Box<Expression>,
        block: Box<Statement>,
    ) -> Box<Statement> {
        Box::new(Statement::ForInStatement(ForInStatement {
            left,
            right,
            block,
        }))
    }

    pub fn for_in_or_of_var_declaration(
        &self,
        declarators: Box<Vec<VariableDeclarator>>,
    ) -> VariableDeclarationOrAssignmentTarget {
        VariableDeclarationOrAssignmentTarget::VariableDeclaration(VariableDeclaration {
            kind: VariableDeclarationKind::Var,
            declarators: *declarators,
        })
    }

    pub fn for_assignment_target(
        &self,
        expression: Box<Expression>,
    ) -> VariableDeclarationOrAssignmentTarget {
        VariableDeclarationOrAssignmentTarget::AssignmentTarget(expression_to_assignment_target(
            expression,
        ))
    }

    pub fn unbox_for_declaration(
        &self,
        declaration: Box<VariableDeclarationOrAssignmentTarget>,
    ) -> VariableDeclarationOrAssignmentTarget {
        *declaration
    }

    // IterationStatement : `for` `(` [lookahead != 'let'] LeftHandSideExpression `of` AssignmentExpression `)` Statement
    // IterationStatement : `for` `(` `var` ForBinding `of` AssignmentExpression `)` Statement
    // IterationStatement : `for` `(` ForDeclaration `of` AssignmentExpression `)` Statement
    pub fn for_of_statement(
        &self,
        left: VariableDeclarationOrAssignmentTarget,
        right: Box<Expression>,
        block: Box<Statement>,
    ) -> Box<Statement> {
        Box::new(Statement::ForOfStatement(ForOfStatement {
            left,
            right,
            block,
        }))
    }

    // IterationStatement : `for` `await` `(` [lookahead != 'let'] LeftHandSideExpression `of` AssignmentExpression `)` Statement
    // IterationStatement : `for` `await` `(` `var` ForBinding `of` AssignmentExpression `)` Statement
    // IterationStatement : `for` `await` `(` ForDeclaration `of` AssignmentExpression `)` Statement
    pub fn for_await_of_statement(
        &self,
        left: VariableDeclarationOrAssignmentTarget,
        right: Box<Expression>,
        block: Box<Statement>,
    ) -> Box<Statement> {
        panic!("not present in current AST");
    }

    // ForDeclaration : LetOrConst ForBinding => ForDeclaration($0, $1)
    pub fn for_declaration(
        &self,
        kind: Box<VariableDeclarationKind>,
        binding: Box<Binding>,
    ) -> Box<VariableDeclarationOrAssignmentTarget> {
        Box::new(VariableDeclarationOrAssignmentTarget::VariableDeclaration(
            VariableDeclaration {
                kind: *kind,
                declarators: vec![VariableDeclarator {
                    binding: *binding,
                    init: None,
                }],
            },
        ))
    }

    // CatchParameter : BindingIdentifier
    // ForBinding : BindingIdentifier
    pub fn binding_identifier(&self, a0: Box<BindingIdentifier>) -> Box<Binding> {
        Box::new(Binding::BindingIdentifier(*a0))
    }

    // CatchParameter : BindingPattern
    // ForBinding : BindingPattern
    pub fn binding_pattern(&self, a0: Box<BindingPattern>) -> Box<Binding> {
        Box::new(Binding::BindingPattern(*a0))
    }

    // ContinueStatement : `continue` `;`
    // ContinueStatement : `continue` LabelIdentifier `;`
    pub fn continue_statement(&self, label: Option<Box<Label>>) -> Box<Statement> {
        Box::new(Statement::ContinueStatement(ContinueStatement {
            label: label.map(|boxed| *boxed),
        }))
    }

    // BreakStatement : `break` `;`
    // BreakStatement : `break` LabelIdentifier `;`
    pub fn break_statement(&self, label: Option<Box<Label>>) -> Box<Statement> {
        Box::new(Statement::BreakStatement(BreakStatement {
            label: label.map(|boxed| *boxed),
        }))
    }

    // ReturnStatement : `return` `;`
    // ReturnStatement : `return` Expression `;`
    pub fn return_statement(&self, expression: Option<Box<Expression>>) -> Box<Statement> {
        Box::new(Statement::ReturnStatement(ReturnStatement { expression }))
    }

    // WithStatement : `with` `(` Expression `)` Statement
    pub fn with_statement(&self, object: Box<Expression>, body: Box<Statement>) -> Box<Statement> {
        Box::new(Statement::WithStatement(WithStatement { object, body }))
    }

    // SwitchStatement : `switch` `(` Expression `)` CaseBlock
    pub fn switch_statement(
        &self,
        discriminant: Box<Expression>,
        mut cases: Box<Statement>,
    ) -> Box<Statement> {
        match &mut *cases {
            Statement::SwitchStatement(stmt) => {
                stmt.discriminant = discriminant;
            }
            Statement::SwitchStatementWithDefault(stmt) => {
                stmt.discriminant = discriminant;
            }
            _ => {
                // Can't happen, grammatically.
                panic!("unrecognized switch statement");
            }
        }
        cases
    }

    // CaseBlock : `{` CaseClauses? `}`
    pub fn case_block(&self, cases: Option<Box<Vec<SwitchCase>>>) -> Box<Statement> {
        Box::new(Statement::SwitchStatement(SwitchStatement {
            discriminant: Box::new(Expression::LiteralNullExpression),
            cases: match cases {
                None => vec![],
                Some(boxed) => *boxed,
            },
        }))
    }

    // CaseBlock : `{` CaseClauses DefaultClause CaseClauses `}`
    pub fn case_block_with_default(
        &self,
        pre_default_cases: Option<Box<Vec<SwitchCase>>>,
        default_case: Box<SwitchDefault>,
        post_default_cases: Option<Box<Vec<SwitchCase>>>,
    ) -> Box<Statement> {
        Box::new(Statement::SwitchStatementWithDefault(
            SwitchStatementWithDefault {
                discriminant: Box::new(Expression::LiteralNullExpression),
                pre_default_cases: match pre_default_cases {
                    None => vec![],
                    Some(boxed) => *boxed,
                },
                default_case: *default_case,
                post_default_cases: match post_default_cases {
                    None => vec![],
                    Some(boxed) => *boxed,
                },
            },
        ))
    }

    // CaseClauses : CaseClause
    pub fn case_clauses_single(&self, a0: Box<SwitchCase>) -> Box<Vec<SwitchCase>> {
        Box::new(vec![*a0])
    }

    // CaseClauses : CaseClauses CaseClause
    pub fn case_clauses_append(
        &self,
        mut a0: Box<Vec<SwitchCase>>,
        a1: Box<SwitchCase>,
    ) -> Box<Vec<SwitchCase>> {
        a0.push(*a1);
        a0
    }

    // CaseClause : `case` Expression `:` StatementList
    pub fn case_clause(
        &self,
        a0: Box<Expression>,
        a1: Option<Box<Vec<Statement>>>,
    ) -> Box<SwitchCase> {
        if let Some(a1) = a1 {
            Box::new(SwitchCase::new(a0, *a1))
        } else {
            Box::new(SwitchCase::new(a0, Vec::new()))
        }
    }

    // DefaultClause : `default` `:` StatementList
    pub fn default_clause(&self, statements: Option<Box<Vec<Statement>>>) -> Box<SwitchDefault> {
        Box::new(SwitchDefault {
            consequent: match statements {
                None => vec![],
                Some(boxed) => *boxed,
            },
        })
    }

    // LabelledStatement : LabelIdentifier `:` LabelledItem
    pub fn labelled_statement(&self, label: Box<Label>, body: Box<Statement>) -> Box<Statement> {
        Box::new(Statement::LabeledStatement(LabeledStatement {
            label: *label,
            body,
        }))
    }

    // ThrowStatement : `throw` Expression `;`
    pub fn throw_statement(&self, expression: Box<Expression>) -> Box<Statement> {
        Box::new(Statement::ThrowStatement(ThrowStatement { expression }))
    }

    // TryStatement : `try` Block Catch
    // TryStatement : `try` Block Finally
    // TryStatement : `try` Block Catch Finally
    pub fn try_statement(
        &self,
        body: Box<Block>,
        catch_clause: Option<Box<CatchClause>>,
        finally_block: Option<Box<Block>>,
    ) -> Box<Statement> {
        match (catch_clause, finally_block) {
            (Some(catch_clause), None) => {
                Box::new(Statement::TryCatchStatement(TryCatchStatement {
                    body: *body,
                    catch_clause: *catch_clause,
                }))
            }
            (catch_clause, Some(finally_block)) => {
                Box::new(Statement::TryFinallyStatement(TryFinallyStatement {
                    body: *body,
                    catch_clause: catch_clause.map(|boxed| *boxed),
                    finalizer: *finally_block,
                }))
            }
            _ => {
                // can't happen, as the grammar won't accept a bare try-block
                panic!("try statement requires either a catch or finally block");
            }
        }
    }

    // Catch : `catch` `(` CatchParameter `)` Block
    pub fn catch(&self, binding: Box<Binding>, body: Box<Block>) -> Box<CatchClause> {
        Box::new(CatchClause {
            binding: *binding,
            body: *body,
        })
    }

    // DebuggerStatement : `debugger` `;`
    pub fn debugger_statement(&self) -> Box<Statement> {
        Box::new(Statement::DebuggerStatement)
    }

    pub fn function_decl(&self, f: Function) -> Box<Statement> {
        Box::new(Statement::FunctionDeclaration(f))
    }

    pub fn function_expr(&self, f: Function) -> Box<Expression> {
        Box::new(Expression::FunctionExpression(f))
    }

    // FunctionDeclaration : `function` BindingIdentifier `(` FormalParameters `)` `{` FunctionBody `}`
    // FunctionDeclaration : [+Default] `function` `(` FormalParameters `)` `{` FunctionBody `}`
    // FunctionExpression : `function` BindingIdentifier? `(` FormalParameters `)` `{` FunctionBody `}`
    pub fn function(
        &self,
        name: Option<Box<BindingIdentifier>>,
        params: Box<FormalParameters>,
        body: Box<FunctionBody>,
    ) -> Function {
        Function::new(name.map(|b| *b), false, false, *params, *body)
    }

    // AsyncFunctionDeclaration : `async` `function` BindingIdentifier `(` FormalParameters `)` `{` AsyncFunctionBody `}`
    // AsyncFunctionDeclaration : [+Default] `async` `function` `(` FormalParameters `)` `{` AsyncFunctionBody `}`
    // AsyncFunctionExpression : `async` `function` `(` FormalParameters `)` `{` AsyncFunctionBody `}`
    pub fn async_function(
        &self,
        name: Option<Box<BindingIdentifier>>,
        params: Box<FormalParameters>,
        body: Box<FunctionBody>,
    ) -> Function {
        Function::new(name.map(|b| *b), true, false, *params, *body)
    }

    // GeneratorDeclaration : `function` `*` BindingIdentifier `(` FormalParameters `)` `{` GeneratorBody `}`
    // GeneratorDeclaration : [+Default] `function` `*` `(` FormalParameters `)` `{` GeneratorBody `}`
    // GeneratorExpression : `function` `*` BindingIdentifier? `(` FormalParameters `)` `{` GeneratorBody `}`
    pub fn generator(
        &self,
        name: Option<Box<BindingIdentifier>>,
        params: Box<FormalParameters>,
        body: Box<FunctionBody>,
    ) -> Function {
        Function::new(name.map(|b| *b), false, true, *params, *body)
    }

    // AsyncGeneratorDeclaration : `async` `function` `*` BindingIdentifier `(` FormalParameters `)` `{` AsyncGeneratorBody `}`
    // AsyncGeneratorDeclaration : [+Default] `async` `function` `*` `(` FormalParameters `)` `{` AsyncGeneratorBody `}`
    // AsyncGeneratorExpression : `async` `function` `*` BindingIdentifier? `(` FormalParameters `)` `{` AsyncGeneratorBody `}`
    pub fn async_generator(
        &self,
        name: Option<Box<BindingIdentifier>>,
        params: Box<FormalParameters>,
        body: Box<FunctionBody>,
    ) -> Function {
        Function::new(name.map(|b| *b), true, true, *params, *body)
    }

    // UniqueFormalParameters : FormalParameters
    pub fn unique_formal_parameters(&self, a0: Box<FormalParameters>) -> Box<FormalParameters> {
        // TODO
        a0
    }

    // FormalParameters : [empty]
    pub fn empty_formal_parameters(&self) -> Box<FormalParameters> {
        Box::new(FormalParameters::new(Vec::new(), None))
    }

    // FormalParameters : FunctionRestParameter
    // FormalParameters : FormalParameterList `,` FunctionRestParameter
    pub fn with_rest_parameter(
        &self,
        mut params: Box<FormalParameters>,
        rest: Box<Binding>,
    ) -> Box<FormalParameters> {
        params.rest = Some(*rest);
        params
    }

    // FormalParameterList : FormalParameter
    pub fn singleton_formal_parameter_list(&self, a0: Box<Parameter>) -> Box<FormalParameters> {
        Box::new(FormalParameters::new(vec![*a0], None))
    }

    // FormalParameterList : FormalParameterList "," FormalParameter
    pub fn append_formal_parameter(
        &self,
        mut params: Box<FormalParameters>,
        next_param: Box<Parameter>,
    ) -> Box<FormalParameters> {
        params.items.push(*next_param);
        params
    }

    // FunctionRestParameter ::= BindingRestElement => FunctionRestParameter($0)
    pub fn function_rest_parameter(&self, a0: Box<Binding>) -> Box<Binding> {
        a0
    }
    // FormalParameter ::= BindingElement => FormalParameter($0)
    pub fn formal_parameter(&self, a0: Box<Binding>) -> Box<Parameter> {
        Box::new(Parameter::Binding(*a0))
    }

    // FunctionBody : FunctionStatementList
    pub fn function_body(&self, a0: Box<Vec<Statement>>) -> Box<FunctionBody> {
        // TODO: Directives
        Box::new(FunctionBody::new(Vec::new(), *a0))
    }

    // FunctionStatementList : StatementList?
    pub fn function_statement_list(&self, a0: Option<Box<Vec<Statement>>>) -> Box<Vec<Statement>> {
        match a0 {
            Some(a0) => a0,
            None => Box::new(Vec::new()),
        }
    }

    // ArrowFunction : ArrowParameters `=>` ConciseBody
    pub fn arrow_function(
        &self,
        params: Box<FormalParameters>,
        body: Box<ArrowExpressionBody>,
    ) -> Box<Expression> {
        Box::new(Expression::ArrowExpression(ArrowExpression {
            is_async: false,
            params: *params,
            body: *body,
        }))
    }

    // ArrowParameters : BindingIdentifier
    pub fn arrow_parameters_bare(
        &self,
        identifier: Box<BindingIdentifier>,
    ) -> Box<FormalParameters> {
        Box::new(FormalParameters {
            items: vec![Parameter::Binding(Binding::BindingIdentifier(*identifier))],
            rest: None,
        })
    }

    // ArrowParameters : CoverParenthesizedExpressionAndArrowParameterList
    pub fn arrow_parameters_parenthesized(&self, a0: Box<Expression>) -> Box<Expression> {
        unimplemented!()
    }

    // ConciseBody : [lookahead != `{`] AssignmentExpression
    pub fn concise_body_expression(&self, expression: Box<Expression>) -> Box<ArrowExpressionBody> {
        Box::new(ArrowExpressionBody::Expression(expression))
    }

    // ConciseBody : `{` FunctionBody `}`
    pub fn concise_body_block(&self, body: Box<FunctionBody>) -> Box<ArrowExpressionBody> {
        Box::new(ArrowExpressionBody::FunctionBody(*body))
    }

    // MethodDefinition ::= PropertyName "(" UniqueFormalParameters ")" "{" FunctionBody "}" => MethodDefinition 0($0, $1, $2, $3, $4, $5, $6)
    pub fn method_definition_p0(
        &self,
        a0: Box<PropertyName>,
        a1: Box<FormalParameters>,
        a2: Box<FunctionBody>,
    ) -> Box<MethodDefinition> {
        Box::new(MethodDefinition::Method(Method::new(
            *a0, false, false, *a1, *a2,
        )))
    }
    // MethodDefinition ::= "get" PropertyName "(" ")" "{" FunctionBody "}" => MethodDefinition 4($0, $1, $2, $3, $4, $5, $6)
    pub fn method_definition_p4(
        &self,
        a0: Box<PropertyName>,
        a1: Box<FunctionBody>,
    ) -> Box<MethodDefinition> {
        Box::new(MethodDefinition::Getter(Getter::new(*a0, *a1)))
    }
    // MethodDefinition ::= "set" PropertyName "(" PropertySetParameterList ")" "{" FunctionBody "}" => MethodDefinition 5($0, $1, $2, $3, $4, $5, $6, $7)
    pub fn method_definition_p5(
        &self,
        a0: Box<PropertyName>,
        a1: Box<Parameter>,
        a2: Box<FunctionBody>,
    ) -> Box<MethodDefinition> {
        Box::new(MethodDefinition::Setter(Setter::new(*a0, *a1, *a2)))
    }
    // PropertySetParameterList ::= FormalParameter => PropertySetParameterList($0)
    pub fn property_set_parameter_list(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(PropertySetParameterList::new())
    }
    // GeneratorMethod ::= "*" PropertyName "(" UniqueFormalParameters ")" "{" GeneratorBody "}" => GeneratorMethod($0, $1, $2, $3, $4, $5, $6, $7)
    pub fn generator_method(
        &self,
        a0: Box<PropertyName>,
        a1: Box<FormalParameters>,
        a2: Box<FunctionBody>,
    ) -> Box<MethodDefinition> {
        Box::new(MethodDefinition::Method(Method::new(
            *a0, false, true, *a1, *a2,
        )))
    }

    // GeneratorBody ::= FunctionBody => GeneratorBody($0)
    pub fn generator_body(&self, a0: Box<FunctionBody>) -> Box<FunctionBody> {
        a0
    }

    // YieldExpression : `yield`
    // YieldExpression : `yield` AssignmentExpression
    pub fn yield_expr(&self, operand: Option<Box<Expression>>) -> Box<Expression> {
        Box::new(Expression::YieldExpression(YieldExpression::new(operand)))
    }

    // YieldExpression : `yield` `*` AssignmentExpression
    pub fn yield_star_expr(&self, operand: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::YieldGeneratorExpression(
            YieldGeneratorExpression::new(operand),
        ))
    }

    // AsyncGeneratorMethod ::= "async" "*" PropertyName "(" UniqueFormalParameters ")" "{" AsyncGeneratorBody "}" => AsyncGeneratorMethod($0, $1, $2, $3, $4, $5, $6, $7, $8)
    pub fn async_generator_method(
        &self,
        a0: Box<PropertyName>,
        a1: Box<FormalParameters>,
        a2: Box<FunctionBody>,
    ) -> Box<MethodDefinition> {
        Box::new(MethodDefinition::Method(Method::new(
            *a0, true, false, *a1, *a2,
        )))
    }

    // AsyncGeneratorBody ::= FunctionBody => AsyncGeneratorBody($0)
    pub fn async_generator_body(&self, a0: Box<FunctionBody>) -> Box<FunctionBody> {
        a0
    }
    // ClassDeclaration ::= "class" BindingIdentifier ClassTail => ClassDeclaration 0($0, $1, $2)
    pub fn class_declaration_p0(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // ClassDeclaration ::= "class" ClassTail => ClassDeclaration 1($0, $1)
    pub fn class_declaration_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // ClassExpression ::= "class" BindingIdentifier ClassTail => ClassExpression($0, Some($1), $2)
    pub fn class_expression(&self, a0: Option<Box<Void>>, a1: Box<Void>) -> Box<Expression> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // ClassTail ::= ClassHeritage "{" ClassBody "}" => ClassTail(Some($0), $1, Some($2), $3)
    pub fn class_tail(&self, a0: Option<Box<Void>>, a1: Option<Box<Void>>) -> Box<Void> {
        unimplemented!(); // Box::new(ClassTail::new())
    }
    // ClassHeritage ::= "extends" LeftHandSideExpression => ClassHeritage($0, $1)
    pub fn class_heritage(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ClassHeritage::new())
    }
    // ClassBody ::= ClassElementList => ClassBody($0)
    pub fn class_body(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ClassBody::new())
    }
    // ClassElementList ::= ClassElement => ClassElementList 0($0)
    pub fn class_element_list_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ClassElementList::new())
    }
    // ClassElementList ::= ClassElementList ClassElement => ClassElementList 1($0, $1)
    pub fn class_element_list_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ClassElementList::new())
    }
    // ClassElement ::= MethodDefinition => ClassElement 0($0)
    pub fn class_element_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ClassElement::new())
    }
    // ClassElement ::= "static" MethodDefinition => ClassElement 1($0, $1)
    pub fn class_element_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ClassElement::new())
    }
    // ClassElement ::= ";" => ClassElement 2($0)
    pub fn class_element_p2(&self) -> Box<Void> {
        unimplemented!(); // Box::new(ClassElement::new())
    }

    // AsyncMethod ::= "async" PropertyName "(" UniqueFormalParameters ")" "{" AsyncFunctionBody "}" => AsyncMethod($0, $1, $2, $3, $4, $5, $6, $7)
    pub fn async_method(
        &self,
        a0: Box<PropertyName>,
        a1: Box<FormalParameters>,
        a2: Box<FunctionBody>,
    ) -> Box<MethodDefinition> {
        Box::new(MethodDefinition::Method(Method::new(
            *a0, true, false, *a1, *a2,
        )))
    }
    // AsyncFunctionBody ::= FunctionBody => AsyncFunctionBody($0)
    pub fn async_function_body(&self, a0: Box<FunctionBody>) -> Box<FunctionBody> {
        a0
    }

    // AwaitExpression : `await` UnaryExpression
    pub fn await_expr(&self, operand: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::AwaitExpression(AwaitExpression::new(operand)))
    }

    // AsyncArrowFunction : `async` AsyncArrowBindingIdentifier `=>` AsyncConciseBody
    // AsyncArrowFunction : CoverCallExpressionAndAsyncArrowHead `=>` AsyncConciseBody
    pub fn async_arrow_function(
        &self,
        params: Box<FormalParameters>,
        body: Box<ArrowExpressionBody>,
    ) -> Box<Expression> {
        Box::new(Expression::ArrowExpression(ArrowExpression {
            is_async: true,
            params: *params,
            body: *body,
        }))
    }

    pub fn async_arrow_parameters(
        &self,
        call_expression: Box<Expression>,
    ) -> Box<FormalParameters> {
        unimplemented!()
    }

    // CoverCallExpressionAndAsyncArrowHead ::= MemberExpression Arguments => CoverCallExpressionAndAsyncArrowHead($0, $1)
    pub fn cover_call_expression_and_async_arrow_head(
        &self,
        a0: Box<Expression>,
        a1: Box<Arguments>,
    ) -> Box<Expression> {
        // TODO
        Box::new(Expression::CallExpression(CallExpression::new(
            ExpressionOrSuper::Expression(Box::new(*a0)),
            *a1,
        )))
    }

    // Script : ScriptBody?
    pub fn script(&self, script: Option<Box<Script>>) -> Box<Script> {
        match script {
            Some(script) => script,
            None => Box::new(Script::new(Vec::new(), Vec::new())),
        }
    }

    // ScriptBody : StatementList
    pub fn script_body(&self, a0: Box<Vec<Statement>>) -> Box<Script> {
        // TODO: directives
        Box::new(Script::new(Vec::new(), *a0))
    }

    // Module ::= ModuleBody => Module(Some($0))
    pub fn module(&self, a0: Option<Box<Void>>) -> Box<Void> {
        unimplemented!(); // Box::new(Module::new())
    }
    // ModuleBody ::= ModuleItemList => ModuleBody($0)
    pub fn module_body(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleBody::new())
    }
    // ModuleItemList ::= ModuleItem => ModuleItemList 0($0)
    pub fn module_item_list_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItemList::new())
    }
    // ModuleItemList ::= ModuleItemList ModuleItem => ModuleItemList 1($0, $1)
    pub fn module_item_list_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItemList::new())
    }
    // ImportDeclaration ::= "import" ImportClause FromClause ErrorSymbol(asi) => ImportDeclaration 0($0, $1, $2)
    pub fn import_declaration_p0(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // ImportDeclaration ::= "import" ModuleSpecifier ErrorSymbol(asi) => ImportDeclaration 1($0, $1)
    pub fn import_declaration_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // ImportClause ::= ImportedDefaultBinding => ImportClause 0($0)
    pub fn import_clause_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ImportClause::new())
    }
    // ImportClause ::= NameSpaceImport => ImportClause 1($0)
    pub fn import_clause_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ImportClause::new())
    }
    // ImportClause ::= NamedImports => ImportClause 2($0)
    pub fn import_clause_p2(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ImportClause::new())
    }
    // ImportClause ::= ImportedDefaultBinding "," NameSpaceImport => ImportClause 3($0, $1, $2)
    pub fn import_clause_p3(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ImportClause::new())
    }
    // ImportClause ::= ImportedDefaultBinding "," NamedImports => ImportClause 4($0, $1, $2)
    pub fn import_clause_p4(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ImportClause::new())
    }
    // ImportedDefaultBinding ::= ImportedBinding => ImportedDefaultBinding($0)
    pub fn imported_default_binding(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ImportedDefaultBinding::new())
    }
    // NameSpaceImport ::= "*" "as" ImportedBinding => NameSpaceImport($0, $1, $2)
    pub fn name_space_import(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(NameSpaceImport::new())
    }
    // NamedImports ::= "{" "}" => NamedImports 0($0, $1)
    pub fn named_imports_p0(&self) -> Box<Void> {
        unimplemented!(); // Box::new(NamedImports::new())
    }
    // NamedImports ::= "{" ImportsList "}" => NamedImports 1($0, $1, $2)
    pub fn named_imports_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(NamedImports::new())
    }
    // NamedImports ::= "{" ImportsList "," "}" => NamedImports 2($0, $1, $2, $3)
    pub fn named_imports_p2(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(NamedImports::new())
    }
    // FromClause ::= "from" ModuleSpecifier => FromClause($0, $1)
    pub fn from_clause(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(FromClause::new())
    }
    // ImportsList ::= ImportSpecifier => ImportsList 0($0)
    pub fn imports_list_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ImportsList::new())
    }
    // ImportsList ::= ImportsList "," ImportSpecifier => ImportsList 1($0, $1, $2)
    pub fn imports_list_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ImportsList::new())
    }
    // ImportSpecifier ::= ImportedBinding => ImportSpecifier 0($0)
    pub fn import_specifier_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ImportSpecifier::new())
    }
    // ImportSpecifier ::= IdentifierName "as" ImportedBinding => ImportSpecifier 1($0, $1, $2)
    pub fn import_specifier_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ImportSpecifier::new())
    }
    // ModuleSpecifier ::= StringLiteral => ModuleSpecifier($0)
    pub fn module_specifier(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleSpecifier::new())
    }
    // ImportedBinding ::= BindingIdentifier => ImportedBinding($0)
    pub fn imported_binding(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ImportedBinding::new())
    }
    // ExportDeclaration ::= "export" "*" FromClause ErrorSymbol(asi) => ExportDeclaration 0($0, $1, $2)
    pub fn export_declaration_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // ExportDeclaration ::= "export" ExportClause FromClause ErrorSymbol(asi) => ExportDeclaration 1($0, $1, $2)
    pub fn export_declaration_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // ExportDeclaration ::= "export" ExportClause ErrorSymbol(asi) => ExportDeclaration 2($0, $1)
    pub fn export_declaration_p2(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // ExportDeclaration ::= "export" VariableStatement => ExportDeclaration 3($0, $1)
    pub fn export_declaration_p3(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // ExportDeclaration ::= "export" Declaration => ExportDeclaration 4($0, $1)
    pub fn export_declaration_p4(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // ExportDeclaration ::= "export" "default" HoistableDeclaration => ExportDeclaration 5($0, $1, $2)
    pub fn export_declaration_p5(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // ExportDeclaration ::= "export" "default" ClassDeclaration => ExportDeclaration 6($0, $1, $2)
    pub fn export_declaration_p6(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // ExportDeclaration ::= "export" "default" [lookahead not in {'function', 'async', 'class'}] AssignmentExpression ErrorSymbol(asi) => ExportDeclaration 7($0, $1, $2)
    pub fn export_declaration_p7(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // ExportClause ::= "{" "}" => ExportClause 0($0, $1)
    pub fn export_clause_p0(&self) -> Box<Void> {
        unimplemented!(); // Box::new(ExportClause::new())
    }
    // ExportClause ::= "{" ExportsList "}" => ExportClause 1($0, $1, $2)
    pub fn export_clause_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ExportClause::new())
    }
    // ExportClause ::= "{" ExportsList "," "}" => ExportClause 2($0, $1, $2, $3)
    pub fn export_clause_p2(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ExportClause::new())
    }
    // ExportsList ::= ExportSpecifier => ExportsList 0($0)
    pub fn exports_list_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ExportsList::new())
    }
    // ExportsList ::= ExportsList "," ExportSpecifier => ExportsList 1($0, $1, $2)
    pub fn exports_list_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ExportsList::new())
    }
    // ExportSpecifier ::= IdentifierName => ExportSpecifier 0($0)
    pub fn export_specifier_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ExportSpecifier::new())
    }
    // ExportSpecifier ::= IdentifierName "as" IdentifierName => ExportSpecifier 1($0, $1, $2)
    pub fn export_specifier_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ExportSpecifier::new())
    }
}
