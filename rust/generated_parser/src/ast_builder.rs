use ast::*;

pub struct AstBuilder {}

fn expression_to_assignment_target(a0: Box<Expression>) -> AssignmentTarget {
    match *a0 {
        Expression::IdentifierExpression(IdentifierExpression {
            var: VariableReference::BindingIdentifier(BindingIdentifier { name: a0 }),
        }) => AssignmentTarget::SimpleAssignmentTarget(
            SimpleAssignmentTarget::AssignmentTargetIdentifier(AssignmentTargetIdentifier::new(a0)),
        ),
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
        Expression::IdentifierExpression(IdentifierExpression {
            var: VariableReference::BindingIdentifier(BindingIdentifier { name: a0 }),
        }) => {
            SimpleAssignmentTarget::AssignmentTargetIdentifier(AssignmentTargetIdentifier::new(a0))
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
    pub fn identifier_reference(&self) -> Box<Identifier> {
        Box::new(Identifier::new("".to_string())) // TODO
    }
    // BindingIdentifier ::= "Identifier" => BindingIdentifier 0($0)
    pub fn binding_identifier_p0(&self) -> Box<BindingIdentifier> {
        Box::new(BindingIdentifier::new(Identifier::new("".to_string()))) // TODO
    }
    // BindingIdentifier ::= "yield" => BindingIdentifier 1($0)
    pub fn binding_identifier_p1(&self) -> Box<BindingIdentifier> {
        Box::new(BindingIdentifier::new(Identifier::new("yield".to_string())))
    }
    // BindingIdentifier ::= "await" => BindingIdentifier 2($0)
    pub fn binding_identifier_p2(&self) -> Box<BindingIdentifier> {
        Box::new(BindingIdentifier::new(Identifier::new("yield".to_string())))
    }
    // LabelIdentifier ::= "Identifier" => LabelIdentifier($0)
    pub fn label_identifier(&self) -> Box<Label> {
        Box::new(Label::new("".to_string())) // TODO
    }
    // PrimaryExpression ::= "this" => PrimaryExpression 0($0)
    pub fn primary_expression_p0(&self) -> Box<Expression> {
        Box::new(Expression::ThisExpression(ThisExpression::new()))
    }
    // PrimaryExpression ::= IdentifierReference => PrimaryExpression 1($0)
    pub fn primary_expression_p1(&self, a0: Box<Identifier>) -> Box<Expression> {
        Box::new(Expression::IdentifierExpression(IdentifierExpression::new(
            VariableReference::BindingIdentifier(BindingIdentifier::new(*a0)),
        )))
    }
    // PrimaryExpression ::= "RegularExpressionLiteral" => PrimaryExpression 10($0)
    pub fn primary_expression_p10(&self) -> Box<Expression> {
        // TODO
        let pattern: String = "".to_string();
        let global: bool = false;
        let ignore_case: bool = false;
        let multi_line: bool = false;
        let sticky: bool = false;
        let unicode: bool = false;
        Box::new(Expression::LiteralRegExpExpression(
            LiteralRegExpExpression::new(pattern, global, ignore_case, multi_line, sticky, unicode),
        ))
    }
    // PrimaryExpression ::= TemplateLiteral => PrimaryExpression 11($0)
    pub fn primary_expression_p11(&self, a0: Box<TemplateExpression>) -> Box<Expression> {
        Box::new(Expression::TemplateExpression(*a0))
    }
    // PrimaryExpression ::= CoverParenthesizedExpressionAndArrowParameterList => PrimaryExpression 12($0)
    pub fn primary_expression_p12(&self, a0: Box<Expression>) -> Box<Expression> {
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
    // Literal ::= "NullLiteral" => Literal 0($0)
    pub fn literal_p0(&self) -> Box<Expression> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // Literal ::= "BooleanLiteral" => Literal 1($0)
    pub fn literal_p1(&self) -> Box<Expression> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // Literal ::= "NumericLiteral" => Literal 2($0)
    pub fn literal_p2(&self) -> Box<Expression> {
        // TODO
        Box::new(Expression::LiteralNumericExpression(
            LiteralNumericExpression::new(0.0),
        ))
    }
    // Literal ::= "StringLiteral" => Literal 3($0)
    pub fn literal_p3(&self) -> Box<Expression> {
        // TODO
        Box::new(Expression::LiteralStringExpression(
            LiteralStringExpression::new("".to_string()),
        ))
    }
    // ArrayLiteral ::= "[" Elision "]" => ArrayLiteral 0($0, Some($1), $2)
    pub fn array_literal_p0(
        &self,
        a0: Option<Box<Vec<ArrayExpressionElement>>>,
    ) -> Box<Expression> {
        if let Some(a0) = a0 {
            Box::new(Expression::ArrayExpression(ArrayExpression::new(*a0)))
        } else {
            Box::new(Expression::ArrayExpression(
                ArrayExpression::new(Vec::new()),
            ))
        }
    }
    // ArrayLiteral ::= "[" ElementList "]" => ArrayLiteral 1($0, $1, $2)
    pub fn array_literal_p1(&self, a0: Box<Vec<ArrayExpressionElement>>) -> Box<Expression> {
        Box::new(Expression::ArrayExpression(ArrayExpression::new(*a0)))
    }
    // ArrayLiteral ::= "[" ElementList "," Elision "]" => ArrayLiteral 2($0, $1, $2, Some($3), $4)
    pub fn array_literal_p2(
        &self,
        mut a0: Box<Vec<ArrayExpressionElement>>,
        a1: Option<Box<Vec<ArrayExpressionElement>>>,
    ) -> Box<Expression> {
        if let Some(mut a1) = a1 {
            a0.append(&mut a1);
        }
        Box::new(Expression::ArrayExpression(ArrayExpression::new(*a0)))
    }
    // ElementList ::= Elision AssignmentExpression => ElementList 0(Some($0), $1)
    pub fn element_list_p0(
        &self,
        a0: Option<Box<Vec<ArrayExpressionElement>>>,
        a1: Box<Expression>,
    ) -> Box<Vec<ArrayExpressionElement>> {
        match a0 {
            Some(mut a0) => {
                a0.push(ArrayExpressionElement::Expression(a1));
                a0
            }
            None => Box::new(vec![ArrayExpressionElement::Expression(a1)]),
        }
    }
    // ElementList ::= Elision SpreadElement => ElementList 1(Some($0), $1)
    pub fn element_list_p1(&self, a0: Option<Box<Parameter>>, a1: Box<SpreadElement>) -> Box<Void> {
        unimplemented!(); // Box::new(ElementList::new())
    }
    // ElementList ::= ElementList "," Elision AssignmentExpression => ElementList 2($0, $1, Some($2), $3)
    pub fn element_list_p2(
        &self,
        mut a0: Box<Vec<ArrayExpressionElement>>,
        a1: Option<Box<Vec<ArrayExpressionElement>>>,
        a2: Box<Expression>,
    ) -> Box<Vec<ArrayExpressionElement>> {
        if let Some(mut a1) = a1 {
            a0.append(&mut a1);
        }
        a0.push(ArrayExpressionElement::Expression(a2));
        a0
    }
    // ElementList ::= ElementList "," Elision SpreadElement => ElementList 3($0, $1, Some($2), $3)
    pub fn element_list_p3(
        &self,
        a0: Box<Void>,
        a1: Option<Box<Void>>,
        a2: Box<Void>,
    ) -> Box<Void> {
        unimplemented!(); // Box::new(ElementList::new())
    }
    // Elision ::= "," => Elision 0($0)
    pub fn elision_p0(&self) -> Box<Void> {
        unimplemented!(); // Box::new(Elision::new())
    }
    // Elision ::= Elision "," => Elision 1($0, $1)
    pub fn elision_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Elision::new())
    }
    // SpreadElement ::= "..." AssignmentExpression => SpreadElement($0, $1)
    pub fn spread_element(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(SpreadElement::new())
    }
    // ObjectLiteral ::= "{" "}" => ObjectLiteral 0($0, $1)
    pub fn object_literal_p0(&self) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // ObjectLiteral ::= "{" PropertyDefinitionList "}" => ObjectLiteral 1($0, $1, $2)
    pub fn object_literal_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // ObjectLiteral ::= "{" PropertyDefinitionList "," "}" => ObjectLiteral 2($0, $1, $2, $3)
    pub fn object_literal_p2(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // PropertyDefinitionList ::= PropertyDefinition => PropertyDefinitionList 0($0)
    pub fn property_definition_list_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(PropertyDefinitionList::new())
    }
    // PropertyDefinitionList ::= PropertyDefinitionList "," PropertyDefinition => PropertyDefinitionList 1($0, $1, $2)
    pub fn property_definition_list_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(PropertyDefinitionList::new())
    }
    // PropertyDefinition ::= IdentifierReference => PropertyDefinition 0($0)
    pub fn property_definition_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(PropertyDefinition::new())
    }
    // PropertyDefinition ::= CoverInitializedName => PropertyDefinition 1($0)
    pub fn property_definition_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(PropertyDefinition::new())
    }
    // PropertyDefinition ::= PropertyName ":" AssignmentExpression => PropertyDefinition 2($0, $1, $2)
    pub fn property_definition_p2(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(PropertyDefinition::new())
    }
    // PropertyDefinition ::= MethodDefinition => PropertyDefinition 3($0)
    pub fn property_definition_p3(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(PropertyDefinition::new())
    }
    // PropertyDefinition ::= "..." AssignmentExpression => PropertyDefinition 4($0, $1)
    pub fn property_definition_p4(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(PropertyDefinition::new())
    }
    // PropertyName ::= LiteralPropertyName => PropertyName 0($0)
    pub fn property_name_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(PropertyName::new())
    }
    // PropertyName ::= ComputedPropertyName => PropertyName 1($0)
    pub fn property_name_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(PropertyName::new())
    }
    // LiteralPropertyName ::= "IdentifierName" => LiteralPropertyName 0($0)
    pub fn literal_property_name_p0(&self) -> Box<Void> {
        unimplemented!(); // Box::new(LiteralPropertyName::new())
    }
    // LiteralPropertyName ::= "StringLiteral" => LiteralPropertyName 1($0)
    pub fn literal_property_name_p1(&self) -> Box<Void> {
        unimplemented!(); // Box::new(LiteralPropertyName::new())
    }
    // LiteralPropertyName ::= "NumericLiteral" => LiteralPropertyName 2($0)
    pub fn literal_property_name_p2(&self) -> Box<Void> {
        unimplemented!(); // Box::new(LiteralPropertyName::new())
    }
    // ComputedPropertyName ::= "[" AssignmentExpression "]" => ComputedPropertyName($0, $1, $2)
    pub fn computed_property_name(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ComputedPropertyName::new())
    }
    // CoverInitializedName ::= IdentifierReference Initializer => CoverInitializedName($0, $1)
    pub fn cover_initialized_name(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(CoverInitializedName::new())
    }
    // Initializer ::= "=" AssignmentExpression => Initializer($0, $1)
    pub fn initializer(&self, a0: Box<Expression>) -> Box<Expression> {
        a0
    }
    // TemplateLiteral ::= "NoSubstitutionTemplate" => TemplateLiteral 0($0)
    pub fn template_literal_p0(&self) -> Box<Void> {
        unimplemented!(); // Box::new(TemplateLiteral::new())
    }
    // TemplateLiteral ::= SubstitutionTemplate => TemplateLiteral 1($0)
    pub fn template_literal_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(TemplateLiteral::new())
    }
    // SubstitutionTemplate ::= "TemplateHead" Expression TemplateSpans => SubstitutionTemplate($0, $1, $2)
    pub fn substitution_template(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(SubstitutionTemplate::new())
    }
    // TemplateSpans ::= "TemplateTail" => TemplateSpans 0($0)
    pub fn template_spans_p0(&self) -> Box<Void> {
        unimplemented!(); // Box::new(TemplateSpans::new())
    }
    // TemplateSpans ::= TemplateMiddleList "TemplateTail" => TemplateSpans 1($0, $1)
    pub fn template_spans_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(TemplateSpans::new())
    }
    // TemplateMiddleList ::= "TemplateMiddle" Expression => TemplateMiddleList 0($0, $1)
    pub fn template_middle_list_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(TemplateMiddleList::new())
    }
    // TemplateMiddleList ::= TemplateMiddleList "TemplateMiddle" Expression => TemplateMiddleList 1($0, $1, $2)
    pub fn template_middle_list_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(TemplateMiddleList::new())
    }
    // MemberExpression ::= MemberExpression "[" Expression "]" => MemberExpression 1($0, $1, $2, $3)
    pub fn member_expression_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // MemberExpression ::= MemberExpression "." "IdentifierName" => MemberExpression 2($0, $1, $2)
    pub fn member_expression_p2(&self, a0: Box<Expression>) -> Box<Expression> {
        // TODO
        Box::new(Expression::MemberExpression(
            MemberExpression::StaticMemberExpression(StaticMemberExpression::new(
                ExpressionOrSuper::Expression(a0),
                IdentifierName::new("".to_string()),
            )),
        ))
    }
    // MemberExpression ::= MemberExpression TemplateLiteral => MemberExpression 3($0, $1)
    pub fn member_expression_p3(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // MemberExpression ::= SuperProperty => MemberExpression 4($0)
    pub fn member_expression_p4(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // MemberExpression ::= MetaProperty => MemberExpression 5($0)
    pub fn member_expression_p5(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // MemberExpression ::= "new" MemberExpression Arguments => MemberExpression 6($0, $1, $2)
    pub fn member_expression_p6(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // SuperProperty ::= "super" "[" Expression "]" => SuperProperty 0($0, $1, $2, $3)
    pub fn super_property_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(SuperProperty::new())
    }
    // SuperProperty ::= "super" "." "IdentifierName" => SuperProperty 1($0, $1, $2)
    pub fn super_property_p1(&self) -> Box<Void> {
        unimplemented!(); // Box::new(SuperProperty::new())
    }
    // MetaProperty ::= NewTarget => MetaProperty($0)
    pub fn meta_property(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(MetaProperty::new())
    }
    // NewTarget ::= "new" "." "target" => NewTarget($0, $1, $2)
    pub fn new_target(&self) -> Box<Void> {
        unimplemented!(); // Box::new(NewTarget::new())
    }
    // NewExpression ::= "new" NewExpression => NewExpression 1($0, $1)
    pub fn new_expression_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // CallExpression ::= CoverCallExpressionAndAsyncArrowHead => CallExpression 0($0)
    pub fn call_expression_p0(&self, a0: Box<Expression>) -> Box<Expression> {
        a0
    }
    // CallExpression ::= SuperCall => CallExpression 1($0)
    pub fn call_expression_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // CallExpression ::= CallExpression Arguments => CallExpression 2($0, $1)
    pub fn call_expression_p2(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // CallExpression ::= CallExpression "[" Expression "]" => CallExpression 3($0, $1, $2, $3)
    pub fn call_expression_p3(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // CallExpression ::= CallExpression "." "IdentifierName" => CallExpression 4($0, $1, $2)
    pub fn call_expression_p4(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // CallExpression ::= CallExpression TemplateLiteral => CallExpression 5($0, $1)
    pub fn call_expression_p5(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // SuperCall ::= "super" Arguments => SuperCall($0, $1)
    pub fn super_call(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(SuperCall::new())
    }
    // Arguments ::= "(" ")" => Arguments 0($0, $1)
    pub fn arguments_p0(&self) -> Box<Arguments> {
        Box::new(Arguments::new(Vec::new()))
    }
    // Arguments ::= "(" ArgumentList ")" => Arguments 1($0, $1, $2)
    pub fn arguments_p1(&self, a0: Box<Arguments>) -> Box<Arguments> {
        a0
    }
    // Arguments ::= "(" ArgumentList "," ")" => Arguments 2($0, $1, $2, $3)
    pub fn arguments_p2(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Arguments::new())
    }
    // ArgumentList ::= AssignmentExpression => ArgumentList 0($0)
    pub fn argument_list_p0(&self, a0: Box<Expression>) -> Box<Arguments> {
        Box::new(Arguments::new(vec![Argument::Expression(a0)]))
    }
    // ArgumentList ::= "..." AssignmentExpression => ArgumentList 1($0, $1)
    pub fn argument_list_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ArgumentList::new())
    }
    // ArgumentList ::= ArgumentList "," AssignmentExpression => ArgumentList 2($0, $1, $2)
    pub fn argument_list_p2(&self, mut a0: Box<Arguments>, a1: Box<Expression>) -> Box<Arguments> {
        a0.args.push(Argument::Expression(a1));
        a0
    }
    // ArgumentList ::= ArgumentList "," "..." AssignmentExpression => ArgumentList 3($0, $1, $2, $3)
    pub fn argument_list_p3(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ArgumentList::new())
    }
    // UpdateExpression ::= LeftHandSideExpression "++" => UpdateExpression 1($0, $1)
    pub fn update_expression_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // UpdateExpression ::= LeftHandSideExpression "--" => UpdateExpression 2($0, $1)
    pub fn update_expression_p2(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // UpdateExpression ::= "++" UnaryExpression => UpdateExpression 3($0, $1)
    pub fn update_expression_p3(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // UpdateExpression ::= "--" UnaryExpression => UpdateExpression 4($0, $1)
    pub fn update_expression_p4(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // UnaryExpression ::= "delete" UnaryExpression => UnaryExpression 1($0, $1)
    pub fn unary_expression_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // UnaryExpression ::= "void" UnaryExpression => UnaryExpression 2($0, $1)
    pub fn unary_expression_p2(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // UnaryExpression ::= "typeof" UnaryExpression => UnaryExpression 3($0, $1)
    pub fn unary_expression_p3(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // UnaryExpression ::= "+" UnaryExpression => UnaryExpression 4($0, $1)
    pub fn unary_expression_p4(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // UnaryExpression ::= "-" UnaryExpression => UnaryExpression 5($0, $1)
    pub fn unary_expression_p5(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // UnaryExpression ::= "~" UnaryExpression => UnaryExpression 6($0, $1)
    pub fn unary_expression_p6(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // UnaryExpression ::= "!" UnaryExpression => UnaryExpression 7($0, $1)
    pub fn unary_expression_p7(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // ExponentiationExpression ::= UpdateExpression "**" ExponentiationExpression => ExponentiationExpression 1($0, $1, $2)
    pub fn exponentiation_expression_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // MultiplicativeExpression ::= MultiplicativeExpression MultiplicativeOperator ExponentiationExpression => MultiplicativeExpression 1($0, $1, $2)
    pub fn multiplicative_expression_p1(
        &self,
        a0: Box<Void>,
        a1: Box<Void>,
        a2: Box<Void>,
    ) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // MultiplicativeOperator ::= "*" => MultiplicativeOperator 0($0)
    pub fn multiplicative_operator_p0(&self) -> Box<Void> {
        unimplemented!(); // Box::new(MultiplicativeOperator::new())
    }
    // MultiplicativeOperator ::= "/" => MultiplicativeOperator 1($0)
    pub fn multiplicative_operator_p1(&self) -> Box<Void> {
        unimplemented!(); // Box::new(MultiplicativeOperator::new())
    }
    // MultiplicativeOperator ::= "%" => MultiplicativeOperator 2($0)
    pub fn multiplicative_operator_p2(&self) -> Box<Void> {
        unimplemented!(); // Box::new(MultiplicativeOperator::new())
    }
    // AdditiveExpression ::= AdditiveExpression "+" MultiplicativeExpression => AdditiveExpression 1($0, $1, $2)
    pub fn additive_expression_p1(
        &self,
        a0: Box<Expression>,
        a1: Box<Expression>,
    ) -> Box<Expression> {
        Box::new(Expression::BinaryExpression(BinaryExpression::new(
            BinaryOperator::Add,
            a0,
            a1,
        )))
    }
    // AdditiveExpression ::= AdditiveExpression "-" MultiplicativeExpression => AdditiveExpression 2($0, $1, $2)
    pub fn additive_expression_p2(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // ShiftExpression ::= ShiftExpression "<<" AdditiveExpression => ShiftExpression 1($0, $1, $2)
    pub fn shift_expression_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // ShiftExpression ::= ShiftExpression ">>" AdditiveExpression => ShiftExpression 2($0, $1, $2)
    pub fn shift_expression_p2(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // ShiftExpression ::= ShiftExpression ">>>" AdditiveExpression => ShiftExpression 3($0, $1, $2)
    pub fn shift_expression_p3(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // RelationalExpression ::= RelationalExpression "<" ShiftExpression => RelationalExpression 1($0, $1, $2)
    pub fn relational_expression_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // RelationalExpression ::= RelationalExpression ">" ShiftExpression => RelationalExpression 2($0, $1, $2)
    pub fn relational_expression_p2(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // RelationalExpression ::= RelationalExpression "<=" ShiftExpression => RelationalExpression 3($0, $1, $2)
    pub fn relational_expression_p3(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // RelationalExpression ::= RelationalExpression ">=" ShiftExpression => RelationalExpression 4($0, $1, $2)
    pub fn relational_expression_p4(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // RelationalExpression ::= RelationalExpression "instanceof" ShiftExpression => RelationalExpression 5($0, $1, $2)
    pub fn relational_expression_p5(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // EqualityExpression ::= EqualityExpression "==" RelationalExpression => EqualityExpression 1($0, $1, $2)
    pub fn equality_expression_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // EqualityExpression ::= EqualityExpression "!=" RelationalExpression => EqualityExpression 2($0, $1, $2)
    pub fn equality_expression_p2(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // EqualityExpression ::= EqualityExpression "===" RelationalExpression => EqualityExpression 3($0, $1, $2)
    pub fn equality_expression_p3(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // EqualityExpression ::= EqualityExpression "!==" RelationalExpression => EqualityExpression 4($0, $1, $2)
    pub fn equality_expression_p4(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // BitwiseANDExpression ::= BitwiseANDExpression "&" EqualityExpression => BitwiseANDExpression 1($0, $1, $2)
    pub fn bitwise_and_expression_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // BitwiseXORExpression ::= BitwiseXORExpression "^" BitwiseANDExpression => BitwiseXORExpression 1($0, $1, $2)
    pub fn bitwise_xor_expression_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // BitwiseORExpression ::= BitwiseORExpression "|" BitwiseXORExpression => BitwiseORExpression 1($0, $1, $2)
    pub fn bitwise_or_expression_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // LogicalANDExpression ::= LogicalANDExpression "&&" BitwiseORExpression => LogicalANDExpression 1($0, $1, $2)
    pub fn logical_and_expression_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // LogicalORExpression ::= LogicalORExpression "||" LogicalANDExpression => LogicalORExpression 1($0, $1, $2)
    pub fn logical_or_expression_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // ConditionalExpression ::= LogicalORExpression "?" AssignmentExpression ":" AssignmentExpression => ConditionalExpression 1($0, $1, $2, $3, $4)
    pub fn conditional_expression_p1(
        &self,
        a0: Box<Void>,
        a1: Box<Void>,
        a2: Box<Void>,
    ) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // AssignmentExpression ::= ArrowFunction => AssignmentExpression 2($0)
    pub fn assignment_expression_p2(&self, a0: Box<ArrowExpression>) -> Box<Expression> {
        Box::new(Expression::ArrowExpression(*a0))
    }
    // AssignmentExpression ::= AsyncArrowFunction => AssignmentExpression 3($0)
    pub fn assignment_expression_p3(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // AssignmentExpression ::= LeftHandSideExpression "=" AssignmentExpression => AssignmentExpression 4($0, $1, $2)
    pub fn assignment_expression_p4(
        &self,
        a0: Box<Expression>,
        a1: Box<Expression>,
    ) -> Box<Expression> {
        let binding = expression_to_assignment_target(a0);
        Box::new(Expression::AssignmentExpression(AssignmentExpression::new(
            binding, a1,
        )))
    }
    // AssignmentExpression ::= LeftHandSideExpression AssignmentOperator AssignmentExpression => AssignmentExpression 5($0, $1, $2)
    pub fn assignment_expression_p5(
        &self,
        a0: Box<Expression>,
        a1: Box<CompoundAssignmentOperator>,
        a2: Box<Expression>,
    ) -> Box<Expression> {
        let binding = expression_to_simple_assignment_target(a0);
        Box::new(Expression::CompoundAssignmentExpression(
            CompoundAssignmentExpression::new(*a1, binding, a2),
        ))
    }
    // AssignmentOperator ::= "*=" => AssignmentOperator 0($0)
    pub fn assignment_operator_p0(&self) -> Box<Void> {
        unimplemented!(); // Box::new(AssignmentOperator::new())
    }
    // AssignmentOperator ::= "/=" => AssignmentOperator 1($0)
    pub fn assignment_operator_p1(&self) -> Box<CompoundAssignmentOperator> {
        Box::new(CompoundAssignmentOperator::Div)
    }
    // AssignmentOperator ::= "%=" => AssignmentOperator 2($0)
    pub fn assignment_operator_p2(&self) -> Box<Void> {
        unimplemented!(); // Box::new(AssignmentOperator::new())
    }
    // AssignmentOperator ::= "+=" => AssignmentOperator 3($0)
    pub fn assignment_operator_p3(&self) -> Box<Void> {
        unimplemented!(); // Box::new(AssignmentOperator::new())
    }
    // AssignmentOperator ::= "-=" => AssignmentOperator 4($0)
    pub fn assignment_operator_p4(&self) -> Box<Void> {
        unimplemented!(); // Box::new(AssignmentOperator::new())
    }
    // AssignmentOperator ::= "<<=" => AssignmentOperator 5($0)
    pub fn assignment_operator_p5(&self) -> Box<Void> {
        unimplemented!(); // Box::new(AssignmentOperator::new())
    }
    // AssignmentOperator ::= ">>=" => AssignmentOperator 6($0)
    pub fn assignment_operator_p6(&self) -> Box<Void> {
        unimplemented!(); // Box::new(AssignmentOperator::new())
    }
    // AssignmentOperator ::= ">>>=" => AssignmentOperator 7($0)
    pub fn assignment_operator_p7(&self) -> Box<Void> {
        unimplemented!(); // Box::new(AssignmentOperator::new())
    }
    // AssignmentOperator ::= "&=" => AssignmentOperator 8($0)
    pub fn assignment_operator_p8(&self) -> Box<Void> {
        unimplemented!(); // Box::new(AssignmentOperator::new())
    }
    // AssignmentOperator ::= "^=" => AssignmentOperator 9($0)
    pub fn assignment_operator_p9(&self) -> Box<Void> {
        unimplemented!(); // Box::new(AssignmentOperator::new())
    }
    // AssignmentOperator ::= "|=" => AssignmentOperator 10($0)
    pub fn assignment_operator_p10(&self) -> Box<Void> {
        unimplemented!(); // Box::new(AssignmentOperator::new())
    }
    // AssignmentOperator ::= "**=" => AssignmentOperator 11($0)
    pub fn assignment_operator_p11(&self) -> Box<Void> {
        unimplemented!(); // Box::new(AssignmentOperator::new())
    }
    // Expression ::= Expression "," AssignmentExpression => Expression 1($0, $1, $2)
    pub fn expression_p1(&self, a0: Box<Expression>, a1: Box<Expression>) -> Box<Expression> {
        Box::new(Expression::BinaryExpression(BinaryExpression::new(
            BinaryOperator::Comma,
            a0,
            a1,
        )))
    }
    // BlockStatement ::= Block => BlockStatement($0)
    pub fn block_statement(&self, a0: Box<Block>) -> Box<Statement> {
        Box::new(Statement::BlockStatement(BlockStatement::new(*a0)))
    }
    // Block ::= "{" StatementList "}" => Block($0, Some($1), $2)
    pub fn block(&self, a0: Option<Box<Vec<Statement>>>) -> Box<Block> {
        match a0 {
            Some(a0) => Box::new(Block::new(*a0)),
            None => Box::new(Block::new(Vec::new())),
        }
    }
    // StatementList ::= StatementListItem => StatementList 0($0)
    pub fn statement_list_p0(&self, a0: Box<Statement>) -> Box<Vec<Statement>> {
        Box::new(vec![*a0])
    }
    // StatementList ::= StatementList StatementListItem => StatementList 1($0, $1)
    pub fn statement_list_p1(
        &self,
        mut a0: Box<Vec<Statement>>,
        a1: Box<Statement>,
    ) -> Box<Vec<Statement>> {
        a0.push(*a1);
        a0
    }
    // LexicalDeclaration ::= LetOrConst BindingList ErrorSymbol(asi) => LexicalDeclaration($0, $1)
    pub fn lexical_declaration(
        &self,
        a0: Box<VariableDeclarationKind>,
        a1: Box<Vec<VariableDeclarator>>,
    ) -> Box<Statement> {
        Box::new(Statement::VariableDeclarationStatement(
            VariableDeclaration::new(*a0, *a1),
        ))
    }
    // ForLexicalDeclaration ::= LetOrConst BindingList ";" => ForLexicalDeclaration($0, $1, $2)
    pub fn for_lexical_declaration(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ForLexicalDeclaration::new())
    }
    // LetOrConst ::= "let" => LetOrConst 0($0)
    pub fn let_or_const_p0(&self) -> Box<VariableDeclarationKind> {
        Box::new(VariableDeclarationKind::Let)
    }
    // LetOrConst ::= "const" => LetOrConst 1($0)
    pub fn let_or_const_p1(&self) -> Box<VariableDeclarationKind> {
        Box::new(VariableDeclarationKind::Const)
    }
    // BindingList ::= LexicalBinding => BindingList 0($0)
    pub fn binding_list_p0(&self, a0: Box<VariableDeclarator>) -> Box<Vec<VariableDeclarator>> {
        Box::new(vec![*a0])
    }
    // BindingList ::= BindingList "," LexicalBinding => BindingList 1($0, $1, $2)
    pub fn binding_list_p1(
        &self,
        mut a0: Box<Vec<VariableDeclarator>>,
        a1: Box<VariableDeclarator>,
    ) -> Box<Vec<VariableDeclarator>> {
        a0.push(*a1);
        a0
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
    // VariableStatement ::= "var" VariableDeclarationList ErrorSymbol(asi) => VariableStatement($0, $1)
    pub fn variable_statement(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // VariableDeclarationList ::= VariableDeclaration => VariableDeclarationList 0($0)
    pub fn variable_declaration_list_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(VariableDeclarationList::new())
    }
    // VariableDeclarationList ::= VariableDeclarationList "," VariableDeclaration => VariableDeclarationList 1($0, $1, $2)
    pub fn variable_declaration_list_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(VariableDeclarationList::new())
    }
    // VariableDeclaration ::= BindingIdentifier Initializer => VariableDeclaration 0($0, Some($1))
    pub fn variable_declaration_p0(&self, a0: Box<Void>, a1: Option<Box<Void>>) -> Box<Void> {
        unimplemented!(); // Box::new(VariableDeclaration::new())
    }
    // VariableDeclaration ::= BindingPattern Initializer => VariableDeclaration 1($0, $1)
    pub fn variable_declaration_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(VariableDeclaration::new())
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
    // EmptyStatement ::= ";" => EmptyStatement($0)
    pub fn empty_statement(&self) -> Box<Statement> {
        Box::new(Statement::EmptyStatement)
    }
    // ExpressionStatement ::= [lookahead not in {'{', 'function', 'async', 'class', 'let'}] Expression ErrorSymbol(asi) => ExpressionStatement($0)
    pub fn expression_statement(&self, a0: Box<Expression>) -> Box<Statement> {
        Box::new(Statement::ExpressionStatement(a0))
    }
    // IfStatement ::= "if" "(" Expression ")" Statement "else" Statement => IfStatement 0($0, $1, $2, $3, $4, $5, $6)
    pub fn if_statement_p0(
        &self,
        a0: Box<Expression>,
        a1: Box<Statement>,
        a2: Box<Statement>,
    ) -> Box<Statement> {
        Box::new(Statement::IfStatement(IfStatement::new(a0, a1, Some(a2))))
    }
    // IfStatement ::= "if" "(" Expression ")" Statement => IfStatement 1($0, $1, $2, $3, $4)
    pub fn if_statement_p1(&self, a0: Box<Expression>, a1: Box<Statement>) -> Box<Statement> {
        Box::new(Statement::IfStatement(IfStatement::new(a0, a1, None)))
    }
    // IterationStatement ::= "do" Statement "while" "(" Expression ")" ErrorSymbol(do_while_asi) => IterationStatement 0($0, $1, $2, $3, $4, $5)
    pub fn iteration_statement_p0(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // IterationStatement ::= "while" "(" Expression ")" Statement => IterationStatement 1($0, $1, $2, $3, $4)
    pub fn iteration_statement_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // IterationStatement ::= "for" "(" [lookahead != 'let'] Expression ";" Expression ";" Expression ")" Statement => IterationStatement 2($0, $1, Some($2), $3, Some($4), $5, Some($6), $7, $8)
    pub fn iteration_statement_p2(
        &self,
        a0: Option<Box<Void>>,
        a1: Option<Box<Void>>,
        a2: Option<Box<Void>>,
        a3: Box<Void>,
    ) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // IterationStatement ::= "for" "(" "var" VariableDeclarationList ";" Expression ";" Expression ")" Statement => IterationStatement 3($0, $1, $2, $3, $4, Some($5), $6, Some($7), $8, $9)
    pub fn iteration_statement_p3(
        &self,
        a0: Box<Void>,
        a1: Option<Box<Void>>,
        a2: Option<Box<Void>>,
        a3: Box<Void>,
    ) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // IterationStatement ::= "for" "(" ForLexicalDeclaration Expression ";" Expression ")" Statement => IterationStatement 4($0, $1, $2, Some($3), $4, Some($5), $6, $7)
    pub fn iteration_statement_p4(
        &self,
        a0: Box<Void>,
        a1: Option<Box<Void>>,
        a2: Option<Box<Void>>,
        a3: Box<Void>,
    ) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // IterationStatement ::= "for" "(" [lookahead != 'let'] LeftHandSideExpression "in" Expression ")" Statement => IterationStatement 5($0, $1, $2, $3, $4, $5, $6)
    pub fn iteration_statement_p5(&self, a0: Box<Void>, a1: Box<Void>, a2: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // IterationStatement ::= "for" "(" "var" ForBinding "in" Expression ")" Statement => IterationStatement 6($0, $1, $2, $3, $4, $5, $6, $7)
    pub fn iteration_statement_p6(&self, a0: Box<Void>, a1: Box<Void>, a2: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // IterationStatement ::= "for" "(" ForDeclaration "in" Expression ")" Statement => IterationStatement 7($0, $1, $2, $3, $4, $5, $6)
    pub fn iteration_statement_p7(&self, a0: Box<Void>, a1: Box<Void>, a2: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // IterationStatement ::= "for" "(" [lookahead != 'let'] LeftHandSideExpression "of" AssignmentExpression ")" Statement => IterationStatement 8($0, $1, $2, $3, $4, $5, $6)
    pub fn iteration_statement_p8(&self, a0: Box<Void>, a1: Box<Void>, a2: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // IterationStatement ::= "for" "(" "var" ForBinding "of" AssignmentExpression ")" Statement => IterationStatement 9($0, $1, $2, $3, $4, $5, $6, $7)
    pub fn iteration_statement_p9(&self, a0: Box<Void>, a1: Box<Void>, a2: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // IterationStatement ::= "for" "(" ForDeclaration "of" AssignmentExpression ")" Statement => IterationStatement 10($0, $1, $2, $3, $4, $5, $6)
    pub fn iteration_statement_p10(
        &self,
        a0: Box<Void>,
        a1: Box<Void>,
        a2: Box<Void>,
    ) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // IterationStatement ::= "for" "await" "(" [lookahead != 'let'] LeftHandSideExpression "of" AssignmentExpression ")" Statement => IterationStatement 11($0, $1, $2, $3, $4, $5, $6, $7)
    pub fn iteration_statement_p11(
        &self,
        a0: Box<Void>,
        a1: Box<Void>,
        a2: Box<Void>,
    ) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // IterationStatement ::= "for" "await" "(" "var" ForBinding "of" AssignmentExpression ")" Statement => IterationStatement 12($0, $1, $2, $3, $4, $5, $6, $7, $8)
    pub fn iteration_statement_p12(
        &self,
        a0: Box<Void>,
        a1: Box<Void>,
        a2: Box<Void>,
    ) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // IterationStatement ::= "for" "await" "(" ForDeclaration "of" AssignmentExpression ")" Statement => IterationStatement 13($0, $1, $2, $3, $4, $5, $6, $7)
    pub fn iteration_statement_p13(
        &self,
        a0: Box<Void>,
        a1: Box<Void>,
        a2: Box<Void>,
    ) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // ForDeclaration ::= LetOrConst ForBinding => ForDeclaration($0, $1)
    pub fn for_declaration(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ForDeclaration::new())
    }
    // ForBinding ::= BindingIdentifier => ForBinding 0($0)
    pub fn for_binding_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ForBinding::new())
    }
    // ForBinding ::= BindingPattern => ForBinding 1($0)
    pub fn for_binding_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ForBinding::new())
    }
    // ContinueStatement ::= "continue" ErrorSymbol(asi) => ContinueStatement 0($0)
    pub fn continue_statement_p0(&self) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // ContinueStatement ::= "continue" LabelIdentifier ErrorSymbol(asi) => ContinueStatement 1($0, $1)
    pub fn continue_statement_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // BreakStatement ::= "break" ErrorSymbol(asi) => BreakStatement 0($0)
    pub fn break_statement_p0(&self) -> Box<Statement> {
        Box::new(Statement::BreakStatement(BreakStatement::new(None)))
    }
    // BreakStatement ::= "break" LabelIdentifier ErrorSymbol(asi) => BreakStatement 1($0, $1)
    pub fn break_statement_p1(&self, a0: Box<Label>) -> Box<Statement> {
        Box::new(Statement::BreakStatement(BreakStatement::new(Some(*a0))))
    }
    // ReturnStatement ::= "return" ErrorSymbol(asi) => ReturnStatement 0($0)
    pub fn return_statement_p0(&self) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // ReturnStatement ::= "return" Expression ErrorSymbol(asi) => ReturnStatement 1($0, $1)
    pub fn return_statement_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // WithStatement ::= "with" "(" Expression ")" Statement => WithStatement($0, $1, $2, $3, $4)
    pub fn with_statement(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // SwitchStatement ::= "switch" "(" Expression ")" CaseBlock => SwitchStatement($0, $1, $2, $3, $4)
    pub fn switch_statement(
        &self,
        a0: Box<Expression>,
        a1: Box<Vec<SwitchCase>>,
    ) -> Box<Statement> {
        Box::new(Statement::SwitchStatement(SwitchStatement::new(a0, *a1)))
    }
    // CaseBlock ::= "{" CaseClauses "}" => CaseBlock 0($0, Some($1), $2)
    pub fn case_block_p0(&self, a0: Option<Box<Vec<SwitchCase>>>) -> Box<Vec<SwitchCase>> {
        match a0 {
            Some(a0) => a0,
            None => Box::new(Vec::new()),
        }
    }
    // CaseBlock ::= "{" CaseClauses DefaultClause CaseClauses "}" => CaseBlock 1($0, Some($1), $2, Some($3), $4)
    pub fn case_block_p1(
        &self,
        a0: Option<Box<Void>>,
        a1: Box<Void>,
        a2: Option<Box<Void>>,
    ) -> Box<Void> {
        unimplemented!(); // Box::new(CaseBlock::new())
    }
    // CaseClauses ::= CaseClause => CaseClauses 0($0)
    pub fn case_clauses_p0(&self, a0: Box<SwitchCase>) -> Box<Vec<SwitchCase>> {
        Box::new(vec![*a0])
    }
    // CaseClauses ::= CaseClauses CaseClause => CaseClauses 1($0, $1)
    pub fn case_clauses_p1(
        &self,
        mut a0: Box<Vec<SwitchCase>>,
        a1: Box<SwitchCase>,
    ) -> Box<Vec<SwitchCase>> {
        a0.push(*a1);
        a0
    }
    // CaseClause ::= "case" Expression ":" StatementList => CaseClause($0, $1, $2, Some($3))
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
    // DefaultClause ::= "default" ":" StatementList => DefaultClause($0, $1, Some($2))
    pub fn default_clause(&self, a0: Option<Box<Void>>) -> Box<Void> {
        unimplemented!(); // Box::new(DefaultClause::new())
    }
    // LabelledStatement ::= LabelIdentifier ":" LabelledItem => LabelledStatement($0, $1, $2)
    pub fn labelled_statement(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // LabelledItem ::= Statement => LabelledItem 0($0)
    pub fn labelled_item_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(LabelledItem::new())
    }
    // LabelledItem ::= FunctionDeclaration => LabelledItem 1($0)
    pub fn labelled_item_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(LabelledItem::new())
    }
    // ThrowStatement ::= "throw" Expression ErrorSymbol(asi) => ThrowStatement($0, $1)
    pub fn throw_statement(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // TryStatement ::= "try" Block Catch => TryStatement 0($0, $1, $2)
    pub fn try_statement_p0(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // TryStatement ::= "try" Block Finally => TryStatement 1($0, $1, $2)
    pub fn try_statement_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // TryStatement ::= "try" Block Catch Finally => TryStatement 2($0, $1, $2, $3)
    pub fn try_statement_p2(&self, a0: Box<Void>, a1: Box<Void>, a2: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // Catch ::= "catch" "(" CatchParameter ")" Block => Catch($0, $1, $2, $3, $4)
    pub fn catch(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Catch::new())
    }
    // Finally ::= "finally" Block => Finally($0, $1)
    pub fn finally(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Finally::new())
    }
    // CatchParameter ::= BindingIdentifier => CatchParameter 0($0)
    pub fn catch_parameter_p0(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(CatchParameter::new())
    }
    // CatchParameter ::= BindingPattern => CatchParameter 1($0)
    pub fn catch_parameter_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(CatchParameter::new())
    }
    // DebuggerStatement ::= "debugger" ErrorSymbol(asi) => DebuggerStatement($0)
    pub fn debugger_statement(&self) -> Box<Void> {
        unimplemented!(); // Box::new(ModuleItem::new())
    }
    // FunctionDeclaration ::= "function" BindingIdentifier "(" FormalParameters ")" "{" FunctionBody "}" => FunctionDeclaration 0($0, $1, $2, $3, $4, $5, $6, $7)
    pub fn function_declaration_p0(
        &self,
        a0: Box<BindingIdentifier>,
        a1: Box<FormalParameters>,
        a2: Box<FunctionBody>,
    ) -> Box<Statement> {
        Box::new(Statement::FunctionDeclaration(FunctionDeclaration::new(
            *a0, false, false, *a1, *a2,
        )))
    }
    // FunctionDeclaration ::= "function" "(" FormalParameters ")" "{" FunctionBody "}" => FunctionDeclaration 1($0, $1, $2, $3, $4, $5, $6)
    pub fn function_declaration_p1(
        &self,
        a0: Box<FormalParameters>,
        a1: Box<FunctionBody>,
    ) -> Box<Statement> {
        let ident = BindingIdentifier::new(Identifier::new("".to_string())); // TODO
        Box::new(Statement::FunctionDeclaration(FunctionDeclaration::new(
            ident, false, false, *a0, *a1,
        )))
    }
    // FunctionExpression ::= "function" BindingIdentifier "(" FormalParameters ")" "{" FunctionBody "}" => FunctionExpression($0, Some($1), $2, $3, $4, $5, $6, $7)
    pub fn function_expression(
        &self,
        a0: Option<Box<BindingIdentifier>>,
        a1: Box<FormalParameters>,
        a2: Box<FunctionBody>,
    ) -> Box<Expression> {
        match a0 {
            Some(a0) => Box::new(Expression::FunctionExpression(FunctionExpression::new(
                Some(*a0),
                false,
                false,
                *a1,
                *a2,
            ))),
            None => Box::new(Expression::FunctionExpression(FunctionExpression::new(
                None, false, false, *a1, *a2,
            ))),
        }
    }
    // UniqueFormalParameters ::= FormalParameters => UniqueFormalParameters($0)
    pub fn unique_formal_parameters(&self, a0: Box<FormalParameters>) -> Box<FormalParameters> {
        // TODO
        a0
    }
    // FormalParameters ::= [empty] => FormalParameters 0()
    pub fn formal_parameters_p0(&self) -> Box<FormalParameters> {
        Box::new(FormalParameters::new(Vec::new(), None))
    }
    // FormalParameters ::= FunctionRestParameter => FormalParameters 1($0)
    pub fn formal_parameters_p1(&self, a0: Box<Binding>) -> Box<FormalParameters> {
        Box::new(FormalParameters::new(Vec::new(), Some(*a0)))
    }
    // FormalParameters ::= FormalParameterList => FormalParameters 2($0)
    pub fn formal_parameters_p2(&self, a0: Box<FormalParameters>) -> Box<FormalParameters> {
        a0
    }
    // FormalParameters ::= FormalParameterList "," => FormalParameters 3($0, $1)
    pub fn formal_parameters_p3(&self, a0: Box<FormalParameters>) -> Box<FormalParameters> {
        a0
    }
    // FormalParameters ::= FormalParameterList "," FunctionRestParameter => FormalParameters 4($0, $1, $2)
    pub fn formal_parameters_p4(
        &self,
        mut a0: Box<FormalParameters>,
        a1: Box<Binding>,
    ) -> Box<FormalParameters> {
        debug_assert!(a0.rest.is_none());
        a0.rest = Some(*a1);
        a0
    }
    // FormalParameterList ::= FormalParameter => FormalParameterList 0($0)
    pub fn formal_parameter_list_p0(&self, a0: Box<Parameter>) -> Box<FormalParameters> {
        Box::new(FormalParameters::new(vec![*a0], None))
    }
    // FormalParameterList ::= FormalParameterList "," FormalParameter => FormalParameterList 1($0, $1, $2)
    pub fn formal_parameter_list_p1(
        &self,
        mut a0: Box<FormalParameters>,
        a1: Box<Parameter>,
    ) -> Box<FormalParameters> {
        a0.items.push(*a1);
        a0
    }
    // FunctionRestParameter ::= BindingRestElement => FunctionRestParameter($0)
    pub fn function_rest_parameter(&self, a0: Box<Binding>) -> Box<Binding> {
        a0
    }
    // FormalParameter ::= BindingElement => FormalParameter($0)
    pub fn formal_parameter(&self, a0: Box<Binding>) -> Box<Parameter> {
        Box::new(Parameter::Binding(*a0))
    }
    // FunctionBody ::= FunctionStatementList => FunctionBody($0)
    pub fn function_body(&self, a0: Box<Vec<Statement>>) -> Box<FunctionBody> {
        // TODO: Directives
        Box::new(FunctionBody::new(Vec::new(), *a0))
    }
    // FunctionStatementList ::= StatementList => FunctionStatementList(Some($0))
    pub fn function_statement_list(&self, a0: Option<Box<Vec<Statement>>>) -> Box<Vec<Statement>> {
        match a0 {
            Some(a0) => a0,
            None => Box::new(Vec::new()),
        }
    }
    // ArrowFunction ::= ArrowParameters "=>" ConciseBody => ArrowFunction($0, $1, $2)
    pub fn arrow_function(
        &self,
        a0: Box<FormalParameters>,
        a1: Box<FunctionBody>,
    ) -> Box<ArrowExpression> {
        // TODO: ArrowExpressionBody::Expression
        Box::new(ArrowExpression::new(
            false,
            *a0,
            ArrowExpressionBody::FunctionBody(*a1),
        ))
    }
    // ArrowParameters ::= BindingIdentifier => ArrowParameters 0($0)
    pub fn arrow_parameters_p0(&self, a0: Box<BindingIdentifier>) -> Box<FormalParameters> {
        Box::new(FormalParameters::new(
            vec![Parameter::Binding(Binding::BindingIdentifier(*a0))],
            None,
        ))
    }
    // ArrowParameters ::= CoverParenthesizedExpressionAndArrowParameterList => ArrowParameters 1($0)
    pub fn arrow_parameters_p1(&self, a0: Box<Expression>) -> Box<Expression> {
        unimplemented!()
    }
    // ConciseBody ::= [lookahead != '{'] AssignmentExpression => ConciseBody 0($0)
    pub fn concise_body_p0(&self, a0: Box<Expression>) -> Box<FunctionBody> {
        Box::new(FunctionBody::new(
            Vec::new(),
            vec![Statement::ExpressionStatement(a0)],
        ))
    }
    // ConciseBody ::= "{" FunctionBody "}" => ConciseBody 1($0, $1, $2)
    pub fn concise_body_p1(&self, a0: Box<FunctionBody>) -> Box<FunctionBody> {
        a0
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
    // GeneratorDeclaration ::= "function" "*" BindingIdentifier "(" FormalParameters ")" "{" GeneratorBody "}" => GeneratorDeclaration 0($0, $1, $2, $3, $4, $5, $6, $7, $8)
    pub fn generator_declaration_p0(
        &self,
        a0: Box<BindingIdentifier>,
        a1: Box<FormalParameters>,
        a2: Box<FunctionBody>,
    ) -> Box<Statement> {
        Box::new(Statement::FunctionDeclaration(FunctionDeclaration::new(
            *a0, false, true, *a1, *a2,
        )))
    }
    // GeneratorDeclaration ::= "function" "*" "(" FormalParameters ")" "{" GeneratorBody "}" => GeneratorDeclaration 1($0, $1, $2, $3, $4, $5, $6, $7)
    pub fn generator_declaration_p1(
        &self,
        a0: Box<FormalParameters>,
        a1: Box<FunctionBody>,
    ) -> Box<Statement> {
        let ident = BindingIdentifier::new(Identifier::new("".to_string())); // TODO
        Box::new(Statement::FunctionDeclaration(FunctionDeclaration::new(
            ident, false, true, *a0, *a1,
        )))
    }
    // GeneratorExpression ::= "function" "*" BindingIdentifier "(" FormalParameters ")" "{" GeneratorBody "}" => GeneratorExpression($0, $1, Some($2), $3, $4, $5, $6, $7, $8)
    pub fn generator_expression(
        &self,
        a0: Option<Box<BindingIdentifier>>,
        a1: Box<FormalParameters>,
        a2: Box<FunctionBody>,
    ) -> Box<Expression> {
        match a0 {
            Some(a0) => Box::new(Expression::FunctionExpression(FunctionExpression::new(
                Some(*a0),
                false,
                true,
                *a1,
                *a2,
            ))),
            None => Box::new(Expression::FunctionExpression(FunctionExpression::new(
                None, false, true, *a1, *a2,
            ))),
        }
    }
    // GeneratorBody ::= FunctionBody => GeneratorBody($0)
    pub fn generator_body(&self, a0: Box<FunctionBody>) -> Box<FunctionBody> {
        a0
    }
    // YieldExpression ::= "yield" => YieldExpression 0($0)
    pub fn yield_expression_p0(&self) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // YieldExpression ::= "yield" AssignmentExpression => YieldExpression 1($0, $1)
    pub fn yield_expression_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // YieldExpression ::= "yield" "*" AssignmentExpression => YieldExpression 2($0, $1, $2)
    pub fn yield_expression_p2(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
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
    // AsyncGeneratorDeclaration ::= "async" "function" "*" BindingIdentifier "(" FormalParameters ")" "{" AsyncGeneratorBody "}" => AsyncGeneratorDeclaration 0($0, $1, $2, $3, $4, $5, $6, $7, $8, $9)
    pub fn async_generator_declaration_p0(
        &self,
        a0: Box<BindingIdentifier>,
        a1: Box<FormalParameters>,
        a2: Box<FunctionBody>,
    ) -> Box<Statement> {
        Box::new(Statement::FunctionDeclaration(FunctionDeclaration::new(
            *a0, true, true, *a1, *a2,
        )))
    }
    // AsyncGeneratorDeclaration ::= "async" "function" "*" "(" FormalParameters ")" "{" AsyncGeneratorBody "}" => AsyncGeneratorDeclaration 1($0, $1, $2, $3, $4, $5, $6, $7, $8)
    pub fn async_generator_declaration_p1(
        &self,
        a0: Box<FormalParameters>,
        a1: Box<FunctionBody>,
    ) -> Box<Function> {
        Box::new(Function::new(true, true, *a0, *a1))
    }
    // AsyncGeneratorExpression ::= "async" "function" "*" BindingIdentifier "(" FormalParameters ")" "{" AsyncGeneratorBody "}" => AsyncGeneratorExpression($0, $1, $2, Some($3), $4, $5, $6, $7, $8, $9)
    pub fn async_generator_expression(
        &self,
        a0: Option<Box<BindingIdentifier>>,
        a1: Box<FormalParameters>,
        a2: Box<FunctionBody>,
    ) -> Box<Expression> {
        match a0 {
            Some(a0) => Box::new(Expression::FunctionExpression(FunctionExpression::new(
                Some(*a0),
                true,
                true,
                *a1,
                *a2,
            ))),
            None => Box::new(Expression::FunctionExpression(FunctionExpression::new(
                None, true, true, *a1, *a2,
            ))),
        }
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
    pub fn class_expression(&self, a0: Option<Box<Void>>, a1: Box<Void>) -> Box<Void> {
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
    // AsyncFunctionDeclaration ::= "async" "function" BindingIdentifier "(" FormalParameters ")" "{" AsyncFunctionBody "}" => AsyncFunctionDeclaration 0($0, $1, $2, $3, $4, $5, $6, $7, $8)
    pub fn async_function_declaration_p0(
        &self,
        a0: Box<BindingIdentifier>,
        a1: Box<FormalParameters>,
        a2: Box<FunctionBody>,
    ) -> Box<Statement> {
        Box::new(Statement::FunctionDeclaration(FunctionDeclaration::new(
            *a0, true, false, *a1, *a2,
        )))
    }
    // AsyncFunctionDeclaration ::= "async" "function" "(" FormalParameters ")" "{" AsyncFunctionBody "}" => AsyncFunctionDeclaration 1($0, $1, $2, $3, $4, $5, $6, $7)
    pub fn async_function_declaration_p1(
        &self,
        a0: Box<FormalParameters>,
        a1: Box<FunctionBody>,
    ) -> Box<Statement> {
        let ident = BindingIdentifier::new(Identifier::new("".to_string())); // TODO
        Box::new(Statement::FunctionDeclaration(FunctionDeclaration::new(
            ident, true, false, *a0, *a1,
        )))
    }
    // AsyncFunctionExpression ::= "async" "function" "(" FormalParameters ")" "{" AsyncFunctionBody "}" => AsyncFunctionExpression 0($0, $1, $2, $3, $4, $5, $6, $7)
    pub fn async_function_expression_p0(
        &self,
        a0: Box<FormalParameters>,
        a1: Box<FunctionBody>,
    ) -> Box<Expression> {
        Box::new(Expression::FunctionExpression(FunctionExpression::new(
            None, true, false, *a0, *a1,
        )))
    }
    // AsyncFunctionExpression ::= "async" "function" BindingIdentifier "(" FormalParameters ")" "{" AsyncFunctionBody "}" => AsyncFunctionExpression 1($0, $1, $2, $3, $4, $5, $6, $7, $8)
    pub fn async_function_expression_p1(
        &self,
        a0: Box<BindingIdentifier>,
        a1: Box<FormalParameters>,
        a2: Box<FunctionBody>,
    ) -> Box<Expression> {
        Box::new(Expression::FunctionExpression(FunctionExpression::new(
            Some(*a0),
            true,
            false,
            *a1,
            *a2,
        )))
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
    // AwaitExpression ::= "await" UnaryExpression => AwaitExpression($0, $1)
    pub fn await_expression(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(Expression::new())
    }
    // AsyncArrowFunction ::= "async" AsyncArrowBindingIdentifier "=>" AsyncConciseBody => AsyncArrowFunction 0($0, $1, $2, $3)
    pub fn async_arrow_function_p0(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(AsyncArrowFunction::new())
    }
    // AsyncArrowFunction ::= CoverCallExpressionAndAsyncArrowHead "=>" AsyncConciseBody => AsyncArrowFunction 1($0, $1, $2)
    pub fn async_arrow_function_p1(&self, a0: Box<Void>, a1: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(AsyncArrowFunction::new())
    }
    // AsyncConciseBody ::= [lookahead != '{'] AssignmentExpression => AsyncConciseBody 0($0)
    pub fn async_concise_body_p0(&self, a0: Box<Expression>) -> Box<FunctionBody> {
        // TODO
        Box::new(FunctionBody::new(
            Vec::new(),
            vec![Statement::ExpressionStatement(a0)],
        ))
    }
    // AsyncConciseBody ::= "{" AsyncFunctionBody "}" => AsyncConciseBody 1($0, $1, $2)
    pub fn async_concise_body_p1(&self, a0: Box<FunctionBody>) -> Box<FunctionBody> {
        a0
    }
    // AsyncArrowBindingIdentifier ::= BindingIdentifier => AsyncArrowBindingIdentifier($0)
    pub fn async_arrow_binding_identifier(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(AsyncArrowBindingIdentifier::new())
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
    // Script ::= ScriptBody => Script(Some($0))
    pub fn script(&self, a0: Option<Box<Script>>) -> Box<Script> {
        match a0 {
            Some(a0) => a0,
            None => Box::new(Script::new(Vec::new(), Vec::new())),
        }
    }
    // ScriptBody ::= StatementList => ScriptBody($0)
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
    // ImportSpecifier ::= "IdentifierName" "as" ImportedBinding => ImportSpecifier 1($0, $1, $2)
    pub fn import_specifier_p1(&self, a0: Box<Void>) -> Box<Void> {
        unimplemented!(); // Box::new(ImportSpecifier::new())
    }
    // ModuleSpecifier ::= "StringLiteral" => ModuleSpecifier($0)
    pub fn module_specifier(&self) -> Box<Void> {
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
    // ExportSpecifier ::= "IdentifierName" => ExportSpecifier 0($0)
    pub fn export_specifier_p0(&self) -> Box<Void> {
        unimplemented!(); // Box::new(ExportSpecifier::new())
    }
    // ExportSpecifier ::= "IdentifierName" "as" "IdentifierName" => ExportSpecifier 1($0, $1, $2)
    pub fn export_specifier_p1(&self) -> Box<Void> {
        unimplemented!(); // Box::new(ExportSpecifier::new())
    }
}
