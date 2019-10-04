use crate::error::{ParseError, Result};
use crate::Token;
use ast::{arena, types::*};
use bumpalo::{vec, Bump};

pub struct AstBuilder<'alloc> {
    pub allocator: &'alloc Bump,
}

impl<'alloc> AstBuilder<'alloc> {
    pub fn alloc<T>(&self, value: T) -> arena::Box<'alloc, T> {
        arena::alloc(self.allocator, value)
    }

    pub fn alloc_str(&self, s: &str) -> &'alloc str {
        arena::alloc_str(self.allocator, s)
    }

    fn new_vec<T>(&self) -> arena::Vec<'alloc, T> {
        arena::Vec::new_in(self.allocator)
    }

    fn new_vec_single<T>(&self, value: T) -> arena::Vec<'alloc, T> {
        vec![in self.allocator; value]
    }

    fn collect_vec_from_results<T, C>(&self, results: C) -> Result<'alloc, arena::Vec<'alloc, T>>
    where
        C: IntoIterator<Item = Result<'alloc, T>>,
    {
        let mut out = self.new_vec();
        for result in results {
            out.push(result?);
        }
        Ok(out)
    }

    fn push<T>(&self, list: &mut arena::Vec<'alloc, T>, value: T) {
        list.push(value);
    }

    fn append<T>(&self, list: &mut arena::Vec<'alloc, T>, elements: &mut arena::Vec<'alloc, T>) {
        list.append(elements);
    }

    // IdentifierReference : Identifier
    pub fn identifier_reference(
        &self,
        token: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, Identifier<'alloc>> {
        self.alloc(self.identifier(token))
    }

    // BindingIdentifier : Identifier
    pub fn binding_identifier(
        &self,
        token: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, BindingIdentifier<'alloc>> {
        self.alloc(BindingIdentifier {
            name: self.identifier(token),
        })
    }

    // BindingIdentifier : `yield`
    pub fn binding_identifier_yield(&self) -> arena::Box<'alloc, BindingIdentifier<'alloc>> {
        self.alloc(BindingIdentifier {
            name: Identifier { value: "yield" },
        })
    }

    // BindingIdentifier : `await`
    pub fn binding_identifier_await(&self) -> arena::Box<'alloc, BindingIdentifier<'alloc>> {
        self.alloc(BindingIdentifier {
            name: Identifier { value: "await" },
        })
    }

    // LabelIdentifier : Identifier
    pub fn label_identifier(
        &self,
        token: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, Label<'alloc>> {
        self.alloc(Label {
            value: token.value.unwrap(),
        })
    }

    // PrimaryExpression : `this`
    pub fn this_expr(&self) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::ThisExpression)
    }

    // PrimaryExpression : IdentifierReference
    pub fn identifier_expr(
        &self,
        name: arena::Box<'alloc, Identifier<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::IdentifierExpression(IdentifierExpression {
            name: name.unbox(),
        }))
    }

    // PrimaryExpression : RegularExpressionLiteral
    pub fn regexp_literal(
        &self,
        token: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        let pattern = token.value.unwrap();
        let global: bool = false;
        let ignore_case: bool = false;
        let multi_line: bool = false;
        let sticky: bool = false;
        let unicode: bool = false;
        self.alloc(Expression::LiteralRegExpExpression(
            LiteralRegExpExpression {
                pattern,
                global,
                ignore_case,
                multi_line,
                sticky,
                unicode,
            },
        ))
    }

    // PrimaryExpression : TemplateLiteral
    pub fn untagged_template_expr(
        &self,
        template_literal: arena::Box<'alloc, TemplateExpression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::TemplateExpression(template_literal.unbox()))
    }

    // PrimaryExpression : CoverParenthesizedExpressionAndArrowParameterList
    pub fn uncover_parenthesized_expression(
        &self,
        parenthesized: arena::Box<'alloc, CoverParenthesized<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        match parenthesized.unbox() {
            CoverParenthesized::Expression(expression) => expression,
            CoverParenthesized::Parameters(_parameters) => unimplemented!(), // TODO
        }
    }

    // CoverParenthesizedExpressionAndArrowParameterList : `(` Expression `)`
    pub fn cover_parenthesized_expression(
        &self,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, CoverParenthesized<'alloc>> {
        self.alloc(CoverParenthesized::Expression(expression))
    }

    // CoverParenthesizedExpressionAndArrowParameterList : `(` `)`
    pub fn empty_parameter_list(&self) -> arena::Vec<'alloc, Parameter<'alloc>> {
        self.new_vec()
    }

    /// Used when parsing `([a, b=2]=arr) =>` to reinterpret as parameter bindings
    /// the snippets `a` and `b=2`, which were previously parsed as assignment targets.
    fn assignment_target_maybe_default_to_binding(
        &self,
        target: AssignmentTargetMaybeDefault<'alloc>,
    ) -> Result<'alloc, Parameter<'alloc>> {
        match target {
            AssignmentTargetMaybeDefault::AssignmentTarget(target) => Ok(Parameter::Binding(
                self.assignment_target_to_binding(target)?,
            )),

            AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(
                AssignmentTargetWithDefault { binding, init },
            ) => Ok(Parameter::BindingWithDefault(BindingWithDefault {
                binding: self.assignment_target_to_binding(binding)?,
                init,
            })),
        }
    }

    fn assignment_target_property_to_binding_property(
        &self,
        target: AssignmentTargetProperty<'alloc>,
    ) -> Result<'alloc, BindingProperty<'alloc>> {
        Ok(match target {
            AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(
                AssignmentTargetPropertyIdentifier {
                    binding: AssignmentTargetIdentifier { name },
                    init,
                },
            ) => BindingProperty::BindingPropertyIdentifier(BindingPropertyIdentifier {
                binding: BindingIdentifier { name },
                init,
            }),

            AssignmentTargetProperty::AssignmentTargetPropertyProperty(
                AssignmentTargetPropertyProperty { name, binding },
            ) => BindingProperty::BindingPropertyProperty(BindingPropertyProperty {
                name,
                binding: self.assignment_target_maybe_default_to_binding(binding)?,
            }),
        })
    }

    /// Refine an AssignmentRestProperty into a BindingRestProperty.
    fn assignment_rest_property_to_binding_identifier(
        &self,
        target: AssignmentTarget<'alloc>,
    ) -> Result<'alloc, arena::Box<'alloc, BindingIdentifier<'alloc>>> {
        match target {
            // ({...x} = dv) => {}
            AssignmentTarget::SimpleAssignmentTarget(
                SimpleAssignmentTarget::AssignmentTargetIdentifier(AssignmentTargetIdentifier {
                    name,
                }),
            ) => Ok(self.alloc(BindingIdentifier { name })),

            // ({...x.y} = dv) => {}
            _ => Err(ParseError::ObjectBindingPatternWithInvalidRest),
        }
    }

    /// Refine the left-hand side of `=` to a parameter binding. The spec says:
    ///
    /// > When the production *ArrowParameters* :
    /// > *CoverParenthesizedExpressionAndArrowParameterList* is recognized,
    /// > the following grammar is used to refine the interpretation of
    /// > *CoverParenthesizedExpressionAndArrowParameterList*:
    /// >
    /// > *ArrowFormalParameters*\[Yield, Await\] :
    /// > `(` *UniqueFormalParameters*\[?Yield, ?Await\] `)`
    ///
    /// Of course, rather than actually reparsing the arrow function parameters,
    /// we work by refining the AST we already built.
    ///
    /// When parsing `(a = 1, [b, c] = obj) => {}`, the assignment targets `a`
    /// and `[b, c]` are passed to this method.
    fn assignment_target_to_binding(
        &self,
        target: AssignmentTarget<'alloc>,
    ) -> Result<'alloc, Binding<'alloc>> {
        match target {
            // (a = dv) => {}
            AssignmentTarget::SimpleAssignmentTarget(
                SimpleAssignmentTarget::AssignmentTargetIdentifier(AssignmentTargetIdentifier {
                    name,
                }),
            ) => Ok(Binding::BindingIdentifier(BindingIdentifier { name })),

            // This case is always an early SyntaxError.
            // (a.x = dv) => {}
            // (a[i] = dv) => {}
            AssignmentTarget::SimpleAssignmentTarget(
                SimpleAssignmentTarget::MemberAssignmentTarget(_),
            ) => Err(ParseError::InvalidParameter),

            // ([a, b] = dv) => {}
            AssignmentTarget::AssignmentTargetPattern(
                AssignmentTargetPattern::ArrayAssignmentTarget(ArrayAssignmentTarget {
                    elements,
                    rest,
                }),
            ) => {
                let elements: arena::Vec<'alloc, Option<AssignmentTargetMaybeDefault<'alloc>>> =
                    elements;
                let elements: arena::Vec<'alloc, Option<Parameter<'alloc>>> = self
                    .collect_vec_from_results(elements.into_iter().map(|maybe_target| {
                        maybe_target
                            .map(|target| self.assignment_target_maybe_default_to_binding(target))
                            .transpose()
                    }))?;
                let rest: Option<Result<'alloc, arena::Box<'alloc, Binding<'alloc>>>> = rest.map(
                    |rest_target| -> Result<'alloc, arena::Box<'alloc, Binding<'alloc>>> {
                        Ok(self.alloc(self.assignment_target_to_binding(rest_target.unbox())?))
                    },
                );
                let rest: Option<arena::Box<'alloc, Binding<'alloc>>> = rest.transpose()?;
                Ok(Binding::BindingPattern(BindingPattern::ArrayBinding(
                    ArrayBinding { elements, rest },
                )))
            }

            // ({a, b: c} = dv) => {}
            AssignmentTarget::AssignmentTargetPattern(
                AssignmentTargetPattern::ObjectAssignmentTarget(ObjectAssignmentTarget {
                    properties,
                    rest,
                }),
            ) => {
                let properties =
                    self.collect_vec_from_results(properties.into_iter().map(|target| {
                        self.assignment_target_property_to_binding_property(target)
                    }))?;

                let rest = if let Some(rest_target) = rest {
                    Some(self.assignment_rest_property_to_binding_identifier(rest_target.unbox())?)
                } else {
                    None
                };
                Ok(Binding::BindingPattern(BindingPattern::ObjectBinding(
                    ObjectBinding { properties, rest },
                )))
            }
        }
    }

    fn object_property_to_binding_property(
        &self,
        op: ObjectProperty<'alloc>,
    ) -> Result<'alloc, BindingProperty<'alloc>> {
        match op {
            ObjectProperty::NamedObjectProperty(NamedObjectProperty::DataProperty(
                DataProperty {
                    property_name,
                    expression,
                },
            )) => Ok(BindingProperty::BindingPropertyProperty(
                BindingPropertyProperty {
                    name: property_name,
                    binding: self.expression_to_parameter(expression.unbox())?,
                },
            )),

            ObjectProperty::NamedObjectProperty(NamedObjectProperty::MethodDefinition(_)) => {
                Err(ParseError::ObjectPatternWithMethod)
            }

            ObjectProperty::ShorthandProperty(ShorthandProperty {
                name: IdentifierExpression { name },
            }) => {
                // TODO - CoverInitializedName can't be represented in an
                // ObjectProperty, but we need it here.
                Ok(BindingProperty::BindingPropertyIdentifier(
                    BindingPropertyIdentifier {
                        binding: BindingIdentifier { name },
                        init: None,
                    },
                ))
            }

            ObjectProperty::SpreadProperty(_expression) => {
                Err(ParseError::ObjectPatternWithNonFinalRest)
            }
        }
    }

    /// Refine an instance of "*PropertyDefinition* : `...`
    /// *AssignmentExpression*" into a *BindingRestProperty*.
    fn spread_expression_to_rest_binding(
        &self,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Result<'alloc, arena::Box<'alloc, BindingIdentifier<'alloc>>> {
        Ok(match expression.unbox() {
            Expression::IdentifierExpression(IdentifierExpression { name }) => {
                self.alloc(BindingIdentifier { name })
            }
            _ => {
                return Err(ParseError::ObjectBindingPatternWithInvalidRest);
            }
        })
    }

    fn pop_trailing_spread_property(
        &self,
        properties: &mut arena::Vec<'alloc, arena::Box<'alloc, ObjectProperty<'alloc>>>,
    ) -> Option<arena::Box<'alloc, Expression<'alloc>>> {
        // Check whether we want to pop a PropertyDefinition
        match properties.last().map(|boxed| &**boxed) {
            Some(ObjectProperty::SpreadProperty(_)) => {}
            _ => return None,
        }

        // We do.
        match properties.pop().unwrap().unbox() {
            ObjectProperty::SpreadProperty(expression) => Some(expression),
            _ => panic!("bug"), // can't happen: we just checked this above
        }
    }

    /// Refine an *ObjectLiteral* into an *ObjectBindingPattern*.
    fn object_expression_to_object_binding(
        &self,
        object: ObjectExpression<'alloc>,
    ) -> Result<'alloc, ObjectBinding<'alloc>> {
        let mut properties = object.properties;
        let rest = self.pop_trailing_spread_property(&mut properties);
        Ok(ObjectBinding {
            properties: self.collect_vec_from_results(
                properties
                    .into_iter()
                    .map(|prop| self.object_property_to_binding_property(prop.unbox())),
            )?,
            rest: rest
                .map(|expression| self.spread_expression_to_rest_binding(expression))
                .transpose()?,
        })
    }

    fn array_elements_to_parameters(
        &self,
        elements: arena::Vec<'alloc, ArrayExpressionElement<'alloc>>,
    ) -> Result<'alloc, arena::Vec<'alloc, Option<Parameter<'alloc>>>> {
        self.collect_vec_from_results(elements.into_iter().map(|element| match element {
                ArrayExpressionElement::Expression(expr) =>
                    Ok(Some(self.expression_to_parameter(expr.unbox())?)),
                ArrayExpressionElement::SpreadElement(_expr) =>
                    // ([...a, b]) => {}
                    Err(ParseError::ArrayPatternWithNonFinalRest),
                ArrayExpressionElement::Elision => Ok(None),
            }))
    }

    fn pop_trailing_spread_element(
        &self,
        elements: &mut arena::Vec<'alloc, ArrayExpressionElement<'alloc>>,
    ) -> Option<arena::Box<'alloc, Expression<'alloc>>> {
        // Check whether we want to pop an element.
        match elements.last() {
            Some(ArrayExpressionElement::SpreadElement(_)) => {}
            _ => return None,
        }

        // We do.
        match elements.pop() {
            Some(ArrayExpressionElement::SpreadElement(expression)) => Some(expression),
            _ => panic!("bug"), // can't happen: we just checked this above
        }
    }

    fn expression_to_binding_no_default(
        &self,
        expression: Expression<'alloc>,
    ) -> Result<'alloc, Binding<'alloc>> {
        match expression {
            Expression::IdentifierExpression(IdentifierExpression { name }) => {
                Ok(Binding::BindingIdentifier(BindingIdentifier { name }))
            }

            Expression::ArrayExpression(ArrayExpression { mut elements }) => {
                let rest = self.pop_trailing_spread_element(&mut elements);
                let elements = self.array_elements_to_parameters(elements)?;
                let rest = rest
                    .map(|expr| match self.expression_to_parameter(expr.unbox())? {
                        Parameter::Binding(b) => Ok(self.alloc(b)),
                        Parameter::BindingWithDefault(_) => {
                            Err(ParseError::ArrayBindingPatternWithInvalidRest)
                        }
                    })
                    .transpose()?;
                Ok(Binding::BindingPattern(BindingPattern::ArrayBinding(
                    ArrayBinding { elements, rest },
                )))
            }

            Expression::ObjectExpression(object) => Ok(Binding::BindingPattern(
                BindingPattern::ObjectBinding(self.object_expression_to_object_binding(object)?),
            )),

            _ => Err(ParseError::InvalidParameter),
        }
    }

    fn expression_to_parameter(
        &self,
        expression: Expression<'alloc>,
    ) -> Result<'alloc, Parameter<'alloc>> {
        match expression {
            Expression::AssignmentExpression(AssignmentExpression {
                binding,
                expression,
            }) => Ok(Parameter::BindingWithDefault(BindingWithDefault {
                binding: self.assignment_target_to_binding(binding)?,
                init: expression,
            })),

            other => Ok(Parameter::Binding(
                self.expression_to_binding_no_default(other)?,
            )),
        }
    }

    // CoverParenthesizedExpressionAndArrowParameterList : `(` Expression `,` `)`
    // CoverParenthesizedExpressionAndArrowParameterList : `(` Expression `,` `...` BindingIdentifier `)`
    // CoverParenthesizedExpressionAndArrowParameterList : `(` Expression `,` `...` BindingPattern `)`
    pub fn expression_to_parameter_list(
        &self,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Result<'alloc, arena::Vec<'alloc, Parameter<'alloc>>> {
        // When the production
        // *ArrowParameters* `:` *CoverParenthesizedExpressionAndArrowParameterList*
        // is recognized the following grammar is used to refine the
        // interpretation of
        // *CoverParenthesizedExpressionAndArrowParameterList*:
        //
        //     ArrowFormalParameters[Yield, Await]:
        //         `(` UniqueFormalParameters[?Yield, ?Await] `)`
        match expression.unbox() {
            Expression::BinaryExpression(BinaryExpression {
                operator: BinaryOperator::Comma,
                left,
                right,
            }) => {
                let mut parameters = self.expression_to_parameter_list(left)?;
                self.push(
                    &mut parameters,
                    self.expression_to_parameter(right.unbox())?,
                );
                Ok(parameters)
            }
            other => Ok(self.new_vec_single(self.expression_to_parameter(other)?)),
        }
    }

    /// Used to convert `async(x, y, ...z)` from a *CallExpression* to async
    /// arrow function parameters.
    fn arguments_to_parameter_list(
        &self,
        arguments: arena::Vec<'alloc, Argument<'alloc>>,
    ) -> Result<'alloc, arena::Box<'alloc, FormalParameters<'alloc>>> {
        let mut items = self.new_vec();
        let mut rest: Option<Binding<'alloc>> = None;
        for arg in arguments {
            if rest.is_some() {
                return Err(ParseError::ArrowParametersWithNonFinalRest);
            }
            match arg {
                Argument::Expression(expr) => {
                    self.push(&mut items, self.expression_to_parameter(expr.unbox())?);
                }
                Argument::SpreadElement(spread_expr) => {
                    rest = Some(self.expression_to_binding_no_default(spread_expr.unbox())?);
                }
            }
        }
        Ok(self.alloc(FormalParameters { items, rest }))
    }

    // CoverParenthesizedExpressionAndArrowParameterList : `(` `)`
    // CoverParenthesizedExpressionAndArrowParameterList : `(` `...` BindingIdentifier `)`
    // CoverParenthesizedExpressionAndArrowParameterList : `(` `...` BindingPattern `)`
    // CoverParenthesizedExpressionAndArrowParameterList : `(` Expression `,` `...` BindingIdentifier `)`
    // CoverParenthesizedExpressionAndArrowParameterList : `(` Expression `,` `...` BindingPattern `)`
    pub fn cover_arrow_parameter_list(
        &self,
        parameters: arena::Vec<'alloc, Parameter<'alloc>>,
        rest: Option<arena::Box<'alloc, Binding<'alloc>>>,
    ) -> arena::Box<'alloc, CoverParenthesized<'alloc>> {
        self.alloc(CoverParenthesized::Parameters(self.alloc(
            FormalParameters {
                items: parameters,
                rest: rest.map(|boxed| boxed.unbox()),
            },
        )))
    }

    // Literal : NullLiteral
    pub fn null_literal(&self) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::LiteralNullExpression)
    }

    // Literal : BooleanLiteral
    pub fn boolean_literal(
        &self,
        token: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        let s = token.unbox().value.unwrap();
        assert!(s == "true" || s == "false");

        self.alloc(Expression::LiteralBooleanExpression(
            LiteralBooleanExpression { value: s == "true" },
        ))
    }

    fn numeric_literal_value(token: arena::Box<'alloc, Token<'alloc>>) -> f64 {
        let s = token.unbox().value.unwrap();

        // BUG: Not all syntax is supported yet.
        s.parse::<f64>().unwrap_or(std::f64::NAN)
    }

    // Literal : NumericLiteral
    pub fn numeric_literal(
        &self,
        token: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::LiteralNumericExpression(
            LiteralNumericExpression {
                value: Self::numeric_literal_value(token),
            },
        ))
    }

    // Literal : StringLiteral
    pub fn string_literal(
        &self,
        token: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::LiteralStringExpression(
            LiteralStringExpression {
                value: token.value.unwrap(),
            },
        ))
    }

    // ArrayLiteral : `[` Elision? `]`
    pub fn array_literal_empty(
        &self,
        elision: Option<arena::Box<'alloc, ArrayExpression<'alloc>>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::ArrayExpression(match elision {
            None => ArrayExpression {
                elements: self.new_vec(),
            },
            Some(array) => array.unbox(),
        }))
    }

    // ArrayLiteral : `[` ElementList `]`
    pub fn array_literal(
        &self,
        array: arena::Box<'alloc, ArrayExpression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::ArrayExpression(array.unbox()))
    }

    // ArrayLiteral : `[` ElementList `,` Elision? `]`
    pub fn array_literal_with_trailing_elision(
        &self,
        mut array: arena::Box<'alloc, ArrayExpression<'alloc>>,
        elision: Option<arena::Box<'alloc, ArrayExpression<'alloc>>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        if let Some(mut more) = elision {
            self.append(&mut array.elements, &mut more.elements);
        }
        self.alloc(Expression::ArrayExpression(array.unbox()))
    }

    // ElementList : Elision? AssignmentExpression
    pub fn element_list_first(
        &self,
        elision: Option<arena::Box<'alloc, ArrayExpression<'alloc>>>,
        element: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, ArrayExpression<'alloc>> {
        let mut array = elision.unwrap_or_else(|| {
            self.alloc(ArrayExpression {
                elements: self.new_vec(),
            })
        });
        self.push(
            &mut array.elements,
            ArrayExpressionElement::Expression(element),
        );
        array
    }

    // ElementList : Elision? SpreadElement
    pub fn element_list_first_spread(
        &self,
        elision: Option<arena::Box<'alloc, ArrayExpression<'alloc>>>,
        spread_element: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, ArrayExpression<'alloc>> {
        let mut array = elision.unwrap_or_else(|| {
            self.alloc(ArrayExpression {
                elements: self.new_vec(),
            })
        });
        self.push(
            &mut array.elements,
            ArrayExpressionElement::SpreadElement(spread_element),
        );
        array
    }

    // ElementList : ElementList `,` Elision? AssignmentExpression
    pub fn element_list_append(
        &self,
        mut array: arena::Box<'alloc, ArrayExpression<'alloc>>,
        elision: Option<arena::Box<'alloc, ArrayExpression<'alloc>>>,
        element: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, ArrayExpression<'alloc>> {
        if let Some(mut elision) = elision {
            self.append(&mut array.elements, &mut elision.elements);
        }
        self.push(
            &mut array.elements,
            ArrayExpressionElement::Expression(element),
        );
        array
    }

    // ElementList : ElementList `,` Elision? SpreadElement
    pub fn element_list_append_spread(
        &self,
        mut array: arena::Box<'alloc, ArrayExpression<'alloc>>,
        elision: Option<arena::Box<'alloc, ArrayExpression<'alloc>>>,
        spread_element: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, ArrayExpression<'alloc>> {
        if let Some(mut elision) = elision {
            self.append(&mut array.elements, &mut elision.elements);
        }
        self.push(
            &mut array.elements,
            ArrayExpressionElement::SpreadElement(spread_element),
        );
        array
    }

    // Elision : `,`
    pub fn elision_single(&self) -> arena::Box<'alloc, ArrayExpression<'alloc>> {
        self.alloc(ArrayExpression {
            elements: self.new_vec_single(ArrayExpressionElement::Elision),
        })
    }

    // Elision : Elision `,`
    pub fn elision_append(
        &self,
        mut array: arena::Box<'alloc, ArrayExpression<'alloc>>,
    ) -> arena::Box<'alloc, ArrayExpression<'alloc>> {
        self.push(&mut array.elements, ArrayExpressionElement::Elision);
        array
    }

    // SpreadElement : `...` AssignmentExpression
    pub fn spread_element(
        &self,
        expr: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        expr
    }

    // ObjectLiteral : `{` `}`
    pub fn object_literal_empty(&self) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::ObjectExpression(ObjectExpression {
            properties: self.new_vec(),
        }))
    }

    // ObjectLiteral : `{` PropertyDefinitionList `}`
    // ObjectLiteral : `{` PropertyDefinitionList `,` `}`
    pub fn object_literal(
        &self,
        object: arena::Box<'alloc, ObjectExpression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::ObjectExpression(object.unbox()))
    }

    // PropertyDefinitionList : PropertyDefinition
    pub fn property_definition_list_single(
        &self,
        property: arena::Box<'alloc, ObjectProperty<'alloc>>,
    ) -> arena::Box<'alloc, ObjectExpression<'alloc>> {
        self.alloc(ObjectExpression {
            properties: self.new_vec_single(property),
        })
    }

    // PropertyDefinitionList : PropertyDefinitionList `,` PropertyDefinition
    pub fn property_definition_list_append(
        &self,
        mut object: arena::Box<'alloc, ObjectExpression<'alloc>>,
        property: arena::Box<'alloc, ObjectProperty<'alloc>>,
    ) -> arena::Box<'alloc, ObjectExpression<'alloc>> {
        self.push(&mut object.properties, property);
        object
    }

    // PropertyDefinition : IdentifierReference
    pub fn shorthand_property(
        &self,
        name: arena::Box<'alloc, Identifier<'alloc>>,
    ) -> arena::Box<'alloc, ObjectProperty<'alloc>> {
        self.alloc(ObjectProperty::ShorthandProperty(ShorthandProperty {
            name: IdentifierExpression { name: name.unbox() },
        }))
    }

    // PropertyDefinition : CoverInitializedName
    pub fn property_definition_cover(
        &self,
        _a0: arena::Box<'alloc, Void>,
    ) -> arena::Box<'alloc, ObjectProperty<'alloc>> {
        // Awkward. This needs to be stored somehow until we reach an enclosing thing.
        unimplemented!();
    }

    // PropertyDefinition : PropertyName `:` AssignmentExpression
    pub fn property_definition(
        &self,
        name: arena::Box<'alloc, PropertyName<'alloc>>,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, ObjectProperty<'alloc>> {
        self.alloc(ObjectProperty::NamedObjectProperty(
            NamedObjectProperty::DataProperty(DataProperty {
                property_name: name.unbox(),
                expression,
            }),
        ))
    }

    // PropertyDefinition : MethodDefinition
    pub fn property_definition_method(
        &self,
        method: arena::Box<'alloc, MethodDefinition<'alloc>>,
    ) -> arena::Box<'alloc, ObjectProperty<'alloc>> {
        self.alloc(ObjectProperty::NamedObjectProperty(
            NamedObjectProperty::MethodDefinition(method.unbox()),
        ))
    }

    // PropertyDefinition : `...` AssignmentExpression
    pub fn property_definition_spread(
        &self,
        spread: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, ObjectProperty<'alloc>> {
        self.alloc(ObjectProperty::SpreadProperty(spread))
    }

    // LiteralPropertyName : IdentifierName
    pub fn property_name_identifier(
        &self,
        token: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, PropertyName<'alloc>> {
        self.alloc(PropertyName::StaticPropertyName(StaticPropertyName {
            value: token.value.unwrap(),
        }))
    }

    // LiteralPropertyName : StringLiteral
    pub fn property_name_string(
        &self,
        token: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, PropertyName<'alloc>> {
        self.alloc(PropertyName::StaticPropertyName(StaticPropertyName {
            value: token.value.unwrap(),
        }))
    }

    // LiteralPropertyName : NumericLiteral
    pub fn property_name_numeric(
        &self,
        token: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, PropertyName<'alloc>> {
        self.alloc(PropertyName::StaticPropertyName(StaticPropertyName {
            value: self.alloc_str(&format!("{:?}", Self::numeric_literal_value(token))),
        }))
    }

    // ComputedPropertyName : `[` AssignmentExpression `]`
    pub fn computed_property_name(
        &self,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, PropertyName<'alloc>> {
        self.alloc(PropertyName::ComputedPropertyName(ComputedPropertyName {
            expression,
        }))
    }

    // CoverInitializedName : IdentifierReference Initializer
    pub fn cover_initialized_name(
        &self,
        _a0: arena::Box<'alloc, Identifier<'alloc>>,
        _a1: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Void> {
        // Awkward. This needs to be stored somehow until we reach an enclosing
        // context where it can be reinterpreted as a default value in an
        // object destructuring assignment pattern.
        unimplemented!();
    }

    // TemplateLiteral : NoSubstitutionTemplate
    pub fn template_literal(
        &self,
        token: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, TemplateExpression<'alloc>> {
        self.alloc(TemplateExpression {
            tag: None,
            elements: self.new_vec_single(TemplateExpressionElement::TemplateElement(
                TemplateElement {
                    raw_value: token.value.unwrap(),
                },
            )),
        })
    }

    // SubstitutionTemplate : TemplateHead Expression TemplateSpans
    pub fn substitution_template(
        &self,
        _head: arena::Box<'alloc, Token<'alloc>>,
        _expression: arena::Box<'alloc, Expression<'alloc>>,
        _spans: arena::Box<'alloc, Void>,
    ) -> arena::Box<'alloc, TemplateExpression<'alloc>> {
        unimplemented!();
    }

    // TemplateSpans : TemplateTail
    // TemplateSpans : TemplateMiddleList TemplateTail
    pub fn template_spans(
        &self,
        _middle_list: Option<arena::Box<'alloc, Void>>,
        _tail: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // TemplateMiddleList : TemplateMiddle Expression
    pub fn template_middle_list_single(
        &self,
        _middle: arena::Box<'alloc, Token<'alloc>>,
        _expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // TemplateMiddleList : TemplateMiddleList TemplateMiddle Expression
    pub fn template_middle_list_append(
        &self,
        _middle_list: arena::Box<'alloc, Void>,
        _middle: arena::Box<'alloc, Token<'alloc>>,
        _expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // MemberExpression : MemberExpression `[` Expression `]`
    // CallExpression : CallExpression `[` Expression `]`
    pub fn computed_member_expr(
        &self,
        object: arena::Box<'alloc, Expression<'alloc>>,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::MemberExpression(
            MemberExpression::ComputedMemberExpression(ComputedMemberExpression {
                object: ExpressionOrSuper::Expression(object),
                expression,
            }),
        ))
    }

    fn identifier(&self, token: arena::Box<'alloc, Token<'alloc>>) -> Identifier<'alloc> {
        Identifier {
            value: token.value.unwrap(),
        }
    }

    fn identifier_name(&self, token: arena::Box<'alloc, Token<'alloc>>) -> IdentifierName<'alloc> {
        IdentifierName {
            value: token.value.unwrap(),
        }
    }

    // MemberExpression : MemberExpression `.` IdentifierName
    // CallExpression : CallExpression `.` IdentifierName
    pub fn static_member_expr(
        &self,
        object: arena::Box<'alloc, Expression<'alloc>>,
        identifier_token: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::MemberExpression(
            MemberExpression::StaticMemberExpression(StaticMemberExpression {
                object: ExpressionOrSuper::Expression(object),
                property: self.identifier_name(identifier_token),
            }),
        ))
    }

    // MemberExpression : MemberExpression TemplateLiteral
    // CallExpression : CallExpression TemplateLiteral
    pub fn tagged_template_expr(
        &self,
        tag: arena::Box<'alloc, Expression<'alloc>>,
        mut template_literal: arena::Box<'alloc, TemplateExpression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        template_literal.tag = Some(tag);
        self.alloc(Expression::TemplateExpression(template_literal.unbox()))
    }

    // MemberExpression : `new` MemberExpression Arguments
    pub fn new_expr_with_arguments(
        &self,
        callee: arena::Box<'alloc, Expression<'alloc>>,
        arguments: arena::Box<'alloc, Arguments<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::NewExpression(NewExpression {
            callee,
            arguments: arguments.unbox(),
        }))
    }

    // SuperProperty : `super` `[` Expression `]`
    pub fn super_property_computed(
        &self,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::MemberExpression(
            MemberExpression::ComputedMemberExpression(ComputedMemberExpression {
                object: ExpressionOrSuper::Super,
                expression: expression,
            }),
        ))
    }

    // SuperProperty : `super` `.` IdentifierName
    pub fn super_property_static(
        &self,
        identifier_token: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::MemberExpression(
            MemberExpression::StaticMemberExpression(StaticMemberExpression {
                object: ExpressionOrSuper::Super,
                property: self.identifier_name(identifier_token),
            }),
        ))
    }

    // NewTarget : `new` `.` `target`
    pub fn new_target_expr(&self) -> arena::Box<'alloc, Expression<'alloc>> {
        return self.alloc(Expression::NewTargetExpression);
    }

    // NewExpression : `new` NewExpression
    pub fn new_expr_without_arguments(
        &self,
        callee: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::NewExpression(NewExpression {
            callee,
            arguments: Arguments {
                args: self.new_vec(),
            },
        }))
    }

    // CallExpression : CallExpression Arguments
    // CoverCallExpressionAndAsyncArrowHead : MemberExpression Arguments
    // CallMemberExpression : MemberExpression Arguments
    pub fn call_expr(
        &self,
        callee: arena::Box<'alloc, Expression<'alloc>>,
        arguments: arena::Box<'alloc, Arguments<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::CallExpression(CallExpression {
            callee: ExpressionOrSuper::Expression(callee),
            arguments: arguments.unbox(),
        }))
    }

    // SuperCall : `super` Arguments
    pub fn super_call(
        &self,
        arguments: arena::Box<'alloc, Arguments<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::CallExpression(CallExpression {
            callee: ExpressionOrSuper::Super,
            arguments: arguments.unbox(),
        }))
    }

    // ImportCall : `import` `(` AssignmentExpression `)`
    pub fn import_call(
        &self,
        argument: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::ImportCallExpression(ImportCallExpression {
            argument,
        }))
    }

    // Arguments : `(` `)`
    pub fn arguments_empty(&self) -> arena::Box<'alloc, Arguments<'alloc>> {
        self.alloc(Arguments {
            args: self.new_vec(),
        })
    }

    // ArgumentList : AssignmentExpression
    // ArgumentList : ArgumentList `,` AssignmentExpression
    pub fn arguments_append(
        &self,
        mut arguments: arena::Box<'alloc, Arguments<'alloc>>,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Arguments<'alloc>> {
        self.push(&mut arguments.args, Argument::Expression(expression));
        arguments
    }

    // ArgumentList : `...` AssignmentExpression
    // ArgumentList : ArgumentList `,` `...` AssignmentExpression
    pub fn arguments_append_spread(
        &self,
        mut arguments: arena::Box<'alloc, Arguments<'alloc>>,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Arguments<'alloc>> {
        self.push(&mut arguments.args, Argument::SpreadElement(expression));
        arguments
    }

    // UpdateExpression : LeftHandSideExpression `++`
    pub fn post_increment_expr(
        &self,
        operand: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Result<'alloc, arena::Box<'alloc, Expression<'alloc>>> {
        Ok(self.alloc(Expression::UpdateExpression(UpdateExpression {
            is_prefix: false,
            operator: UpdateOperator::Increment,
            operand: self.expression_to_simple_assignment_target(operand)?,
        })))
    }

    // UpdateExpression : LeftHandSideExpression `--`
    pub fn post_decrement_expr(
        &self,
        operand: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Result<'alloc, arena::Box<'alloc, Expression<'alloc>>> {
        Ok(self.alloc(Expression::UpdateExpression(UpdateExpression {
            is_prefix: false,
            operator: UpdateOperator::Decrement,
            operand: self.expression_to_simple_assignment_target(operand)?,
        })))
    }

    // UpdateExpression : `++` UnaryExpression
    pub fn pre_increment_expr(
        &self,
        operand: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Result<'alloc, arena::Box<'alloc, Expression<'alloc>>> {
        Ok(self.alloc(Expression::UpdateExpression(UpdateExpression {
            is_prefix: true,
            operator: UpdateOperator::Increment,
            operand: self.expression_to_simple_assignment_target(operand)?,
        })))
    }

    // UpdateExpression : `--` UnaryExpression
    pub fn pre_decrement_expr(
        &self,
        operand: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Result<'alloc, arena::Box<'alloc, Expression<'alloc>>> {
        Ok(self.alloc(Expression::UpdateExpression(UpdateExpression {
            is_prefix: true,
            operator: UpdateOperator::Decrement,
            operand: self.expression_to_simple_assignment_target(operand)?,
        })))
    }

    // UnaryExpression : `delete` UnaryExpression
    pub fn delete_expr(
        &self,
        operand: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::UnaryExpression(UnaryExpression {
            operator: UnaryOperator::Delete,
            operand,
        }))
    }

    // UnaryExpression : `void` UnaryExpression
    pub fn void_expr(
        &self,
        operand: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::UnaryExpression(UnaryExpression {
            operator: UnaryOperator::Void,
            operand,
        }))
    }

    // UnaryExpression : `typeof` UnaryExpression
    pub fn typeof_expr(
        &self,
        operand: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::UnaryExpression(UnaryExpression {
            operator: UnaryOperator::Typeof,
            operand,
        }))
    }

    // UnaryExpression : `+` UnaryExpression
    pub fn unary_plus_expr(
        &self,
        operand: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::UnaryExpression(UnaryExpression {
            operator: UnaryOperator::Plus,
            operand,
        }))
    }

    // UnaryExpression : `-` UnaryExpression
    pub fn unary_minus_expr(
        &self,
        operand: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::UnaryExpression(UnaryExpression {
            operator: UnaryOperator::Minus,
            operand,
        }))
    }

    // UnaryExpression : `~` UnaryExpression
    pub fn bitwise_not_expr(
        &self,
        operand: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::UnaryExpression(UnaryExpression {
            operator: UnaryOperator::BitwiseNot,
            operand,
        }))
    }

    // UnaryExpression : `!` UnaryExpression
    pub fn logical_not_expr(
        &self,
        operand: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::UnaryExpression(UnaryExpression {
            operator: UnaryOperator::LogicalNot,
            operand,
        }))
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
    pub fn box_op(&self, op: BinaryOperator) -> arena::Box<'alloc, BinaryOperator> {
        self.alloc(op)
    }

    // MultiplicativeExpression : MultiplicativeExpression MultiplicativeOperator ExponentiationExpression
    pub fn multiplicative_expr(
        &self,
        left: arena::Box<'alloc, Expression<'alloc>>,
        operator: arena::Box<'alloc, BinaryOperator>,
        right: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.binary_expr(operator.unbox(), left, right)
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
        left: arena::Box<'alloc, Expression<'alloc>>,
        right: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::BinaryExpression(BinaryExpression {
            operator,
            left,
            right,
        }))
    }

    // ConditionalExpression : LogicalORExpression `?` AssignmentExpression `:` AssignmentExpression
    pub fn conditional_expr(
        &self,
        test: arena::Box<'alloc, Expression<'alloc>>,
        consequent: arena::Box<'alloc, Expression<'alloc>>,
        alternate: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::ConditionalExpression(ConditionalExpression {
            test,
            consequent,
            alternate,
        }))
    }

    /// Refine an *ArrayLiteral* into an *ArrayAssignmentPattern*.
    fn array_expression_to_array_assignment_target(
        &self,
        mut elements: arena::Vec<'alloc, ArrayExpressionElement<'alloc>>,
    ) -> Result<'alloc, ArrayAssignmentTarget<'alloc>> {
        let spread = self.pop_trailing_spread_element(&mut elements);
        let elements = self.collect_vec_from_results(elements.into_iter().map(|element| {
            Ok(match element {
                ArrayExpressionElement::SpreadElement(_) => unimplemented!(),
                ArrayExpressionElement::Expression(expression) => {
                    Some(self.expression_to_assignment_target_maybe_default(expression)?)
                }
                ArrayExpressionElement::Elision => None,
            })
        }))?;
        let rest: Option<Result<'alloc, arena::Box<'alloc, AssignmentTarget<'alloc>>>> =
            spread.map(|expr| Ok(self.alloc(self.expression_to_assignment_target(expr)?)));
        let rest = rest.transpose()?;
        Ok(ArrayAssignmentTarget { elements, rest })
    }

    fn object_property_to_assignment_target_property(
        &self,
        property: arena::Box<'alloc, ObjectProperty<'alloc>>,
    ) -> Result<'alloc, AssignmentTargetProperty<'alloc>> {
        Ok(match property.unbox() {
            ObjectProperty::NamedObjectProperty(NamedObjectProperty::MethodDefinition(_)) => {
                return Err(ParseError::ObjectPatternWithMethod)
            }

            ObjectProperty::NamedObjectProperty(NamedObjectProperty::DataProperty(
                DataProperty {
                    property_name,
                    expression,
                },
            )) => AssignmentTargetProperty::AssignmentTargetPropertyProperty(
                AssignmentTargetPropertyProperty {
                    name: property_name,
                    binding: self.expression_to_assignment_target_maybe_default(expression)?,
                },
            ),

            ObjectProperty::ShorthandProperty(ShorthandProperty {
                name: IdentifierExpression { name },
            }) => {
                // TODO - support CoverInitializedName
                AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(
                    AssignmentTargetPropertyIdentifier {
                        binding: AssignmentTargetIdentifier { name },
                        init: None,
                    },
                )
            }

            ObjectProperty::SpreadProperty(_expression) => {
                return Err(ParseError::ObjectPatternWithNonFinalRest)
            }
        })
    }

    // Refine an *ObjectLiteral* into an *ObjectAssignmentPattern*.
    fn object_expression_to_object_assignment_target(
        &self,
        mut properties: arena::Vec<'alloc, arena::Box<'alloc, ObjectProperty<'alloc>>>,
    ) -> Result<'alloc, ObjectAssignmentTarget<'alloc>> {
        let spread = self.pop_trailing_spread_property(&mut properties);
        let properties = self.collect_vec_from_results(
            properties
                .into_iter()
                .map(|p| self.object_property_to_assignment_target_property(p)),
        )?;
        let rest: Option<Result<'alloc, arena::Box<'alloc, AssignmentTarget<'alloc>>>> =
            spread.map(|expr| Ok(self.alloc(self.expression_to_assignment_target(expr)?)));
        let rest = rest.transpose()?;
        Ok(ObjectAssignmentTarget { properties, rest })
    }

    fn expression_to_assignment_target_maybe_default(
        &self,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Result<'alloc, AssignmentTargetMaybeDefault<'alloc>> {
        Ok(match expression.unbox() {
            Expression::AssignmentExpression(AssignmentExpression {
                binding,
                expression,
            }) => AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(
                AssignmentTargetWithDefault {
                    binding,
                    init: expression,
                },
            ),

            other => AssignmentTargetMaybeDefault::AssignmentTarget(
                self.expression_to_assignment_target(self.alloc(other))?,
            ),
        })
    }

    fn expression_to_assignment_target(
        &self,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Result<'alloc, AssignmentTarget<'alloc>> {
        Ok(match expression.unbox() {
            Expression::ArrayExpression(ArrayExpression { elements }) => {
                AssignmentTarget::AssignmentTargetPattern(
                    AssignmentTargetPattern::ArrayAssignmentTarget(
                        self.array_expression_to_array_assignment_target(elements)?,
                    ),
                )
            }

            Expression::ObjectExpression(ObjectExpression { properties }) => {
                AssignmentTarget::AssignmentTargetPattern(
                    AssignmentTargetPattern::ObjectAssignmentTarget(
                        self.object_expression_to_object_assignment_target(properties)?,
                    ),
                )
            }

            other => AssignmentTarget::SimpleAssignmentTarget(
                self.expression_to_simple_assignment_target(self.alloc(other))?,
            ),
        })
    }

    fn expression_to_simple_assignment_target(
        &self,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Result<'alloc, SimpleAssignmentTarget<'alloc>> {
        Ok(match expression.unbox() {
            Expression::IdentifierExpression(IdentifierExpression { name }) => {
                SimpleAssignmentTarget::AssignmentTargetIdentifier(AssignmentTargetIdentifier {
                    name,
                })
            }

            Expression::MemberExpression(MemberExpression::StaticMemberExpression(
                StaticMemberExpression { object, property },
            )) => SimpleAssignmentTarget::MemberAssignmentTarget(
                MemberAssignmentTarget::StaticMemberAssignmentTarget(
                    StaticMemberAssignmentTarget { object, property },
                ),
            ),
            Expression::MemberExpression(MemberExpression::ComputedMemberExpression(
                ComputedMemberExpression { object, expression },
            )) => SimpleAssignmentTarget::MemberAssignmentTarget(
                MemberAssignmentTarget::ComputedMemberAssignmentTarget(
                    ComputedMemberAssignmentTarget { object, expression },
                ),
            ),

            _ => {
                return Err(ParseError::InvalidAssignmentTarget);
            }
        })
    }

    // AssignmentExpression : LeftHandSideExpression `=` AssignmentExpression
    pub fn assignment_expr(
        &self,
        left_hand_side: arena::Box<'alloc, Expression<'alloc>>,
        value: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Result<'alloc, arena::Box<'alloc, Expression<'alloc>>> {
        let target = self.expression_to_assignment_target(left_hand_side)?;
        Ok(
            self.alloc(Expression::AssignmentExpression(AssignmentExpression {
                binding: target,
                expression: value,
            })),
        )
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

    pub fn box_assign_op(
        &self,
        op: CompoundAssignmentOperator,
    ) -> arena::Box<'alloc, CompoundAssignmentOperator> {
        self.alloc(op)
    }

    // AssignmentExpression : LeftHandSideExpression AssignmentOperator AssignmentExpression
    pub fn compound_assignment_expr(
        &self,
        left_hand_side: arena::Box<'alloc, Expression<'alloc>>,
        operator: arena::Box<'alloc, CompoundAssignmentOperator>,
        value: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Result<'alloc, arena::Box<'alloc, Expression<'alloc>>> {
        let target = self.expression_to_simple_assignment_target(left_hand_side)?;
        Ok(self.alloc(Expression::CompoundAssignmentExpression(
            CompoundAssignmentExpression {
                operator: operator.unbox(),
                binding: target,
                expression: value,
            },
        )))
    }

    // BlockStatement : Block
    pub fn block_statement(
        &self,
        block: arena::Box<'alloc, Block<'alloc>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::BlockStatement(BlockStatement {
            block: block.unbox(),
        }))
    }

    // Block : `{` StatementList? `}`
    pub fn block(
        &self,
        statements: Option<arena::Box<'alloc, arena::Vec<'alloc, Statement<'alloc>>>>,
    ) -> arena::Box<'alloc, Block<'alloc>> {
        self.alloc(Block {
            statements: match statements {
                Some(statements) => statements.unbox(),
                None => self.new_vec(),
            },
            declarations: None,
        })
    }

    // StatementList : StatementListItem
    pub fn statement_list_single(
        &self,
        statement: arena::Box<'alloc, Statement<'alloc>>,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, Statement<'alloc>>> {
        self.alloc(self.new_vec_single(statement.unbox()))
    }

    // StatementList : StatementList StatementListItem
    pub fn statement_list_append(
        &self,
        mut list: arena::Box<'alloc, arena::Vec<'alloc, Statement<'alloc>>>,
        statement: arena::Box<'alloc, Statement<'alloc>>,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, Statement<'alloc>>> {
        self.push(&mut list, statement.unbox());
        list
    }

    // LexicalDeclaration : LetOrConst BindingList `;`
    pub fn lexical_declaration(
        &self,
        kind: arena::Box<'alloc, VariableDeclarationKind>,
        declarators: arena::Box<'alloc, arena::Vec<'alloc, VariableDeclarator<'alloc>>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::VariableDeclarationStatement(
            VariableDeclaration {
                kind: kind.unbox(),
                declarators: declarators.unbox(),
            },
        ))
    }

    // ForLexicalDeclaration : LetOrConst BindingList `;`
    pub fn for_lexical_declaration(
        &self,
        kind: arena::Box<'alloc, VariableDeclarationKind>,
        declarators: arena::Box<'alloc, arena::Vec<'alloc, VariableDeclarator<'alloc>>>,
    ) -> arena::Box<'alloc, VariableDeclarationOrExpression<'alloc>> {
        self.alloc(VariableDeclarationOrExpression::VariableDeclaration(
            VariableDeclaration {
                kind: kind.unbox(),
                declarators: declarators.unbox(),
            },
        ))
    }

    // LetOrConst : `let`
    pub fn let_kind(&self) -> arena::Box<'alloc, VariableDeclarationKind> {
        self.alloc(VariableDeclarationKind::Let)
    }

    // LetOrConst : `const`
    pub fn const_kind(&self) -> arena::Box<'alloc, VariableDeclarationKind> {
        self.alloc(VariableDeclarationKind::Const)
    }

    // VariableStatement : `var` VariableDeclarationList `;`
    pub fn variable_statement(
        &self,
        declarators: arena::Box<'alloc, arena::Vec<'alloc, VariableDeclarator<'alloc>>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::VariableDeclarationStatement(
            VariableDeclaration {
                kind: VariableDeclarationKind::Var,
                declarators: declarators.unbox(),
            },
        ))
    }

    // VariableDeclarationList : VariableDeclaration
    // BindingList : LexicalBinding
    pub fn variable_declaration_list_single(
        &self,
        decl: arena::Box<'alloc, VariableDeclarator<'alloc>>,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, VariableDeclarator<'alloc>>> {
        self.alloc(self.new_vec_single(decl.unbox()))
    }

    // VariableDeclarationList : VariableDeclarationList `,` VariableDeclaration
    // BindingList : BindingList `,` LexicalBinding
    pub fn variable_declaration_list_append(
        &self,
        mut list: arena::Box<'alloc, arena::Vec<'alloc, VariableDeclarator<'alloc>>>,
        decl: arena::Box<'alloc, VariableDeclarator<'alloc>>,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, VariableDeclarator<'alloc>>> {
        self.push(&mut list, decl.unbox());
        list
    }

    // VariableDeclaration : BindingIdentifier Initializer?
    // VariableDeclaration : BindingPattern Initializer
    pub fn variable_declaration(
        &self,
        binding: arena::Box<'alloc, Binding<'alloc>>,
        init: Option<arena::Box<'alloc, Expression<'alloc>>>,
    ) -> arena::Box<'alloc, VariableDeclarator<'alloc>> {
        self.alloc(VariableDeclarator {
            binding: binding.unbox(),
            init,
        })
    }

    // ObjectBindingPattern : `{` `}`
    // ObjectBindingPattern : `{` BindingRestProperty `}`
    // ObjectBindingPattern : `{` BindingPropertyList `}`
    // ObjectBindingPattern : `{` BindingPropertyList `,` BindingRestProperty? `}`
    pub fn object_binding_pattern(
        &self,
        properties: arena::Box<'alloc, arena::Vec<'alloc, BindingProperty<'alloc>>>,
        rest: Option<arena::Box<'alloc, BindingIdentifier<'alloc>>>,
    ) -> arena::Box<'alloc, Binding<'alloc>> {
        self.alloc(Binding::BindingPattern(BindingPattern::ObjectBinding(
            ObjectBinding {
                properties: properties.unbox(),
                rest,
            },
        )))
    }

    pub fn binding_element_list_empty(
        &self,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, Option<Parameter<'alloc>>>> {
        self.alloc(self.new_vec())
    }

    // ArrayBindingPattern : `[` Elision? BindingRestElement? `]`
    // ArrayBindingPattern : `[` BindingElementList `]`
    // ArrayBindingPattern : `[` BindingElementList `,` Elision? BindingRestElement? `]`
    pub fn array_binding_pattern(
        &self,
        mut elements: arena::Box<'alloc, arena::Vec<'alloc, Option<Parameter<'alloc>>>>,
        elision: Option<arena::Box<'alloc, ArrayExpression<'alloc>>>,
        rest: Option<arena::Box<'alloc, Binding<'alloc>>>,
    ) -> arena::Box<'alloc, Binding<'alloc>> {
        if let Some(elision) = elision {
            for _ in 0..elision.elements.len() {
                self.push(&mut elements, None);
            }
        }

        self.alloc(Binding::BindingPattern(BindingPattern::ArrayBinding(
            ArrayBinding {
                elements: elements.unbox(),
                rest,
            },
        )))
    }

    pub fn binding_property_list_empty(
        &self,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, BindingProperty<'alloc>>> {
        self.alloc(self.new_vec())
    }

    // BindingPropertyList : BindingProperty
    pub fn binding_property_list_single(
        &self,
        property: arena::Box<'alloc, BindingProperty<'alloc>>,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, BindingProperty<'alloc>>> {
        self.alloc(self.new_vec_single(property.unbox()))
    }

    // BindingPropertyList : BindingPropertyList `,` BindingProperty
    pub fn binding_property_list_append(
        &self,
        mut list: arena::Box<'alloc, arena::Vec<'alloc, BindingProperty<'alloc>>>,
        property: arena::Box<'alloc, BindingProperty<'alloc>>,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, BindingProperty<'alloc>>> {
        self.push(&mut list, property.unbox());
        list
    }

    // BindingElementList : BindingElementList `,` BindingElisionElement
    pub fn binding_element_list_append(
        &self,
        mut list: arena::Box<'alloc, arena::Vec<'alloc, Option<Parameter<'alloc>>>>,
        mut element: arena::Box<'alloc, arena::Vec<'alloc, Option<Parameter<'alloc>>>>,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, Option<Parameter<'alloc>>>> {
        self.append(&mut list, &mut element);
        list
    }

    // BindingElisionElement : Elision? BindingElement
    pub fn binding_elision_element(
        &self,
        elision: Option<arena::Box<'alloc, ArrayExpression<'alloc>>>,
        element: arena::Box<'alloc, Parameter<'alloc>>,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, Option<Parameter<'alloc>>>> {
        let elision_count = elision.map(|v| v.elements.len()).unwrap_or(0);
        let mut result = self.alloc(self.new_vec());
        for _ in 0..elision_count {
            self.push(&mut result, None);
        }
        self.push(&mut result, Some(element.unbox()));
        result
    }

    // BindingProperty : SingleNameBinding
    pub fn binding_property_shorthand(
        &self,
        binding: arena::Box<'alloc, Parameter<'alloc>>,
    ) -> arena::Box<'alloc, BindingProperty<'alloc>> {
        // Previous parsing interpreted this as a Parameter. We need to take
        // all the pieces out of that box and put them in a new box.
        let (binding, init) = match binding.unbox() {
            Parameter::Binding(binding) => (binding, None),
            Parameter::BindingWithDefault(BindingWithDefault { binding, init }) => {
                (binding, Some(init.unbox()))
            }
        };

        let binding = match binding {
            Binding::BindingIdentifier(bi) => bi,
            _ => {
                // The grammar ensures that the parser always passes a valid
                // argument to this method.
                panic!("invalid argument: binding_property_shorthand requires a Binding::BindingIdentifier");
            }
        };

        self.alloc(BindingProperty::BindingPropertyIdentifier(
            BindingPropertyIdentifier {
                binding,
                init: init.map(|x| self.alloc(x)),
            },
        ))
    }

    // BindingProperty : PropertyName `:` BindingElement
    pub fn binding_property(
        &self,
        name: arena::Box<'alloc, PropertyName<'alloc>>,
        binding: arena::Box<'alloc, Parameter<'alloc>>,
    ) -> arena::Box<'alloc, BindingProperty<'alloc>> {
        self.alloc(BindingProperty::BindingPropertyProperty(
            BindingPropertyProperty {
                name: name.unbox(),
                binding: binding.unbox(),
            },
        ))
    }

    // BindingElement : BindingPattern Initializer?
    pub fn binding_element_pattern(
        &self,
        binding: arena::Box<'alloc, Binding<'alloc>>,
        init: Option<arena::Box<'alloc, Expression<'alloc>>>,
    ) -> arena::Box<'alloc, Parameter<'alloc>> {
        self.alloc(match init {
            None => Parameter::Binding(binding.unbox()),
            Some(init) => Parameter::BindingWithDefault(BindingWithDefault {
                binding: binding.unbox(),
                init,
            }),
        })
    }

    // SingleNameBinding : BindingIdentifier Initializer?
    pub fn single_name_binding(
        &self,
        name: arena::Box<'alloc, BindingIdentifier<'alloc>>,
        init: Option<arena::Box<'alloc, Expression<'alloc>>>,
    ) -> arena::Box<'alloc, Parameter<'alloc>> {
        let binding = Binding::BindingIdentifier(name.unbox());
        self.alloc(match init {
            None => Parameter::Binding(binding),
            Some(init) => Parameter::BindingWithDefault(BindingWithDefault { binding, init }),
        })
    }

    // BindingRestElement : `...` BindingIdentifier
    pub fn binding_rest_element(
        &self,
        name: arena::Box<'alloc, BindingIdentifier<'alloc>>,
    ) -> arena::Box<'alloc, Binding<'alloc>> {
        self.alloc(Binding::BindingIdentifier(name.unbox()))
    }

    // EmptyStatement : `;`
    pub fn empty_statement(&self) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::EmptyStatement)
    }

    // ExpressionStatement : [lookahead not in {'{', 'function', 'async', 'class', 'let'}] Expression `;`
    pub fn expression_statement(
        &self,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::ExpressionStatement(expression))
    }

    // IfStatement : `if` `(` Expression `)` Statement `else` Statement
    // IfStatement : `if` `(` Expression `)` Statement
    pub fn if_statement(
        &self,
        test: arena::Box<'alloc, Expression<'alloc>>,
        consequent: arena::Box<'alloc, Statement<'alloc>>,
        alternate: Option<arena::Box<'alloc, Statement<'alloc>>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::IfStatement(IfStatement {
            test,
            consequent,
            alternate,
        }))
    }

    // IterationStatement : `do` Statement `while` `(` Expression `)` `;`
    pub fn do_while_statement(
        &self,
        block: arena::Box<'alloc, Statement<'alloc>>,
        test: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::DoWhileStatement(DoWhileStatement {
            block,
            test,
        }))
    }

    // IterationStatement : `while` `(` Expression `)` Statement
    pub fn while_statement(
        &self,
        test: arena::Box<'alloc, Expression<'alloc>>,
        block: arena::Box<'alloc, Statement<'alloc>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::WhileStatement(WhileStatement { test, block }))
    }

    // IterationStatement : `for` `(` [lookahead != 'let'] Expression? `;` Expression? `;` Expression? `)` Statement
    // IterationStatement : `for` `(` `var` VariableDeclarationList `;` Expression? `;` Expression? `)` Statement
    // IterationStatement : `for` `(` ForLexicalDeclaration Expression? `;` Expression? `)` Statement
    pub fn for_statement(
        &self,
        init: Option<VariableDeclarationOrExpression<'alloc>>,
        test: Option<arena::Box<'alloc, Expression<'alloc>>>,
        update: Option<arena::Box<'alloc, Expression<'alloc>>>,
        block: arena::Box<'alloc, Statement<'alloc>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::ForStatement(ForStatement {
            init,
            test,
            update,
            block,
        }))
    }

    pub fn for_expression(
        &self,
        expr: Option<arena::Box<'alloc, Expression<'alloc>>>,
    ) -> Option<VariableDeclarationOrExpression<'alloc>> {
        expr.map(|expr| VariableDeclarationOrExpression::Expression(expr))
    }

    pub fn for_var_declaration(
        &self,
        declarators: arena::Box<'alloc, arena::Vec<'alloc, VariableDeclarator<'alloc>>>,
    ) -> VariableDeclarationOrExpression<'alloc> {
        VariableDeclarationOrExpression::VariableDeclaration(VariableDeclaration {
            kind: VariableDeclarationKind::Var,
            declarators: declarators.unbox(),
        })
    }

    pub fn unbox_for_lexical_declaration(
        &self,
        declaration: arena::Box<'alloc, VariableDeclarationOrExpression<'alloc>>,
    ) -> VariableDeclarationOrExpression<'alloc> {
        declaration.unbox()
    }

    // IterationStatement : `for` `(` [lookahead != 'let'] LeftHandSideExpression `in` Expression `)` Statement
    // IterationStatement : `for` `(` `var` ForBinding `in` Expression `)` Statement
    // IterationStatement : `for` `(` ForDeclaration `in` Expression `)` Statement
    pub fn for_in_statement(
        &self,
        left: VariableDeclarationOrAssignmentTarget<'alloc>,
        right: arena::Box<'alloc, Expression<'alloc>>,
        block: arena::Box<'alloc, Statement<'alloc>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::ForInStatement(ForInStatement {
            left,
            right,
            block,
        }))
    }

    pub fn for_in_or_of_var_declaration(
        &self,
        binding: arena::Box<'alloc, Binding<'alloc>>,
    ) -> VariableDeclarationOrAssignmentTarget<'alloc> {
        VariableDeclarationOrAssignmentTarget::VariableDeclaration(VariableDeclaration {
            kind: VariableDeclarationKind::Var,
            declarators: self.new_vec_single(VariableDeclarator {
                binding: binding.unbox(),
                init: None,
            }),
        })
    }

    pub fn for_assignment_target(
        &self,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Result<'alloc, VariableDeclarationOrAssignmentTarget<'alloc>> {
        Ok(VariableDeclarationOrAssignmentTarget::AssignmentTarget(
            self.expression_to_assignment_target(expression)?,
        ))
    }

    pub fn unbox_for_declaration(
        &self,
        declaration: arena::Box<'alloc, VariableDeclarationOrAssignmentTarget<'alloc>>,
    ) -> VariableDeclarationOrAssignmentTarget<'alloc> {
        declaration.unbox()
    }

    // IterationStatement : `for` `(` [lookahead != 'let'] LeftHandSideExpression `of` AssignmentExpression `)` Statement
    // IterationStatement : `for` `(` `var` ForBinding `of` AssignmentExpression `)` Statement
    // IterationStatement : `for` `(` ForDeclaration `of` AssignmentExpression `)` Statement
    pub fn for_of_statement(
        &self,
        left: VariableDeclarationOrAssignmentTarget<'alloc>,
        right: arena::Box<'alloc, Expression<'alloc>>,
        block: arena::Box<'alloc, Statement<'alloc>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::ForOfStatement(ForOfStatement {
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
        _left: VariableDeclarationOrAssignmentTarget,
        _right: arena::Box<'alloc, Expression<'alloc>>,
        _block: arena::Box<'alloc, Statement<'alloc>>,
    ) -> Result<'alloc, arena::Box<'alloc, Statement<'alloc>>> {
        Err(ParseError::NotImplemented(
            "for await statement (missing from AST)",
        ))
    }

    // ForDeclaration : LetOrConst ForBinding => ForDeclaration($0, $1)
    pub fn for_declaration(
        &self,
        kind: arena::Box<'alloc, VariableDeclarationKind>,
        binding: arena::Box<'alloc, Binding<'alloc>>,
    ) -> arena::Box<'alloc, VariableDeclarationOrAssignmentTarget<'alloc>> {
        self.alloc(VariableDeclarationOrAssignmentTarget::VariableDeclaration(
            VariableDeclaration {
                kind: kind.unbox(),
                declarators: self.new_vec_single(VariableDeclarator {
                    binding: binding.unbox(),
                    init: None,
                }),
            },
        ))
    }

    // CatchParameter : BindingIdentifier
    // ForBinding : BindingIdentifier
    // LexicalBinding : BindingIdentifier Initializer?
    // VariableDeclaration : BindingIdentifier Initializer?
    pub fn binding_identifier_to_binding(
        &self,
        identifier: arena::Box<'alloc, BindingIdentifier<'alloc>>,
    ) -> arena::Box<'alloc, Binding<'alloc>> {
        self.alloc(Binding::BindingIdentifier(identifier.unbox()))
    }

    // ContinueStatement : `continue` `;`
    // ContinueStatement : `continue` LabelIdentifier `;`
    pub fn continue_statement(
        &self,
        label: Option<arena::Box<'alloc, Label<'alloc>>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::ContinueStatement(ContinueStatement {
            label: label.map(|boxed| boxed.unbox()),
        }))
    }

    // BreakStatement : `break` `;`
    // BreakStatement : `break` LabelIdentifier `;`
    pub fn break_statement(
        &self,
        label: Option<arena::Box<'alloc, Label<'alloc>>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::BreakStatement(BreakStatement {
            label: label.map(|boxed| boxed.unbox()),
        }))
    }

    // ReturnStatement : `return` `;`
    // ReturnStatement : `return` Expression `;`
    pub fn return_statement(
        &self,
        expression: Option<arena::Box<'alloc, Expression<'alloc>>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::ReturnStatement(ReturnStatement { expression }))
    }

    // WithStatement : `with` `(` Expression `)` Statement
    pub fn with_statement(
        &self,
        object: arena::Box<'alloc, Expression<'alloc>>,
        body: arena::Box<'alloc, Statement<'alloc>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::WithStatement(WithStatement { object, body }))
    }

    // SwitchStatement : `switch` `(` Expression `)` CaseBlock
    pub fn switch_statement(
        &self,
        discriminant: arena::Box<'alloc, Expression<'alloc>>,
        mut cases: arena::Box<'alloc, Statement<'alloc>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        match &mut *cases {
            Statement::SwitchStatement(stmt) => {
                stmt.discriminant = discriminant;
            }
            Statement::SwitchStatementWithDefault(stmt) => {
                stmt.discriminant = discriminant;
            }
            _ => {
                // The grammar ensures that the parser always passes a valid
                // argument to this method.
                panic!("invalid argument: argument 2 must be a SwitchStatement");
            }
        }
        cases
    }

    // CaseBlock : `{` CaseClauses? `}`
    pub fn case_block(
        &self,
        cases: Option<arena::Box<'alloc, arena::Vec<'alloc, SwitchCase<'alloc>>>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::SwitchStatement(SwitchStatement {
            discriminant: self.alloc(Expression::LiteralNullExpression),
            cases: match cases {
                None => self.new_vec(),
                Some(boxed) => boxed.unbox(),
            },
        }))
    }

    // CaseBlock : `{` CaseClauses DefaultClause CaseClauses `}`
    pub fn case_block_with_default(
        &self,
        pre_default_cases: Option<arena::Box<'alloc, arena::Vec<'alloc, SwitchCase<'alloc>>>>,
        default_case: arena::Box<'alloc, SwitchDefault<'alloc>>,
        post_default_cases: Option<arena::Box<'alloc, arena::Vec<'alloc, SwitchCase<'alloc>>>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::SwitchStatementWithDefault(
            SwitchStatementWithDefault {
                discriminant: self.alloc(Expression::LiteralNullExpression),
                pre_default_cases: match pre_default_cases {
                    None => self.new_vec(),
                    Some(boxed) => boxed.unbox(),
                },
                default_case: default_case.unbox(),
                post_default_cases: match post_default_cases {
                    None => self.new_vec(),
                    Some(boxed) => boxed.unbox(),
                },
            },
        ))
    }

    // CaseClauses : CaseClause
    pub fn case_clauses_single(
        &self,
        case: arena::Box<'alloc, SwitchCase<'alloc>>,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, SwitchCase<'alloc>>> {
        self.alloc(self.new_vec_single(case.unbox()))
    }

    // CaseClauses : CaseClauses CaseClause
    pub fn case_clauses_append(
        &self,
        mut cases: arena::Box<'alloc, arena::Vec<'alloc, SwitchCase<'alloc>>>,
        case: arena::Box<'alloc, SwitchCase<'alloc>>,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, SwitchCase<'alloc>>> {
        self.push(&mut cases, case.unbox());
        cases
    }

    // CaseClause : `case` Expression `:` StatementList
    pub fn case_clause(
        &self,
        expression: arena::Box<'alloc, Expression<'alloc>>,
        statements: Option<arena::Box<'alloc, arena::Vec<'alloc, Statement<'alloc>>>>,
    ) -> arena::Box<'alloc, SwitchCase<'alloc>> {
        if let Some(statements) = statements {
            self.alloc(SwitchCase {
                test: expression,
                consequent: statements.unbox(),
            })
        } else {
            self.alloc(SwitchCase {
                test: expression,
                consequent: self.new_vec(),
            })
        }
    }

    // DefaultClause : `default` `:` StatementList
    pub fn default_clause(
        &self,
        statements: Option<arena::Box<'alloc, arena::Vec<'alloc, Statement<'alloc>>>>,
    ) -> arena::Box<'alloc, SwitchDefault<'alloc>> {
        self.alloc(SwitchDefault {
            consequent: match statements {
                None => self.new_vec(),
                Some(boxed) => boxed.unbox(),
            },
        })
    }

    // LabelledStatement : LabelIdentifier `:` LabelledItem
    pub fn labelled_statement(
        &self,
        label: arena::Box<'alloc, Label<'alloc>>,
        body: arena::Box<'alloc, Statement<'alloc>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::LabeledStatement(LabeledStatement {
            label: label.unbox(),
            body,
        }))
    }

    // ThrowStatement : `throw` Expression `;`
    pub fn throw_statement(
        &self,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::ThrowStatement(ThrowStatement { expression }))
    }

    // TryStatement : `try` Block Catch
    // TryStatement : `try` Block Finally
    // TryStatement : `try` Block Catch Finally
    pub fn try_statement(
        &self,
        body: arena::Box<'alloc, Block<'alloc>>,
        catch_clause: Option<arena::Box<'alloc, CatchClause<'alloc>>>,
        finally_block: Option<arena::Box<'alloc, Block<'alloc>>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        match (catch_clause, finally_block) {
            (Some(catch_clause), None) => {
                self.alloc(Statement::TryCatchStatement(TryCatchStatement {
                    body: body.unbox(),
                    catch_clause: catch_clause.unbox(),
                }))
            }
            (catch_clause, Some(finally_block)) => {
                self.alloc(Statement::TryFinallyStatement(TryFinallyStatement {
                    body: body.unbox(),
                    catch_clause: catch_clause.map(|boxed| boxed.unbox()),
                    finalizer: finally_block.unbox(),
                }))
            }
            _ => {
                // The grammar won't accept a bare try-block, so the parser always
                // a catch clause, a finally block, or both.
                panic!("invalid argument: try_statement requires a catch or finally block");
            }
        }
    }

    // Catch : `catch` `(` CatchParameter `)` Block
    pub fn catch(
        &self,
        binding: Option<arena::Box<'alloc, Binding<'alloc>>>,
        body: arena::Box<'alloc, Block<'alloc>>,
    ) -> arena::Box<'alloc, CatchClause<'alloc>> {
        self.alloc(CatchClause {
            binding,
            body: body.unbox(),
        })
    }

    // DebuggerStatement : `debugger` `;`
    pub fn debugger_statement(&self) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::DebuggerStatement)
    }

    pub fn function_decl(&self, f: Function<'alloc>) -> arena::Box<'alloc, Statement<'alloc>> {
        self.alloc(Statement::FunctionDeclaration(f))
    }

    pub fn function_expr(&self, f: Function<'alloc>) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::FunctionExpression(f))
    }

    // FunctionDeclaration : `function` BindingIdentifier `(` FormalParameters `)` `{` FunctionBody `}`
    // FunctionDeclaration : [+Default] `function` `(` FormalParameters `)` `{` FunctionBody `}`
    // FunctionExpression : `function` BindingIdentifier? `(` FormalParameters `)` `{` FunctionBody `}`
    pub fn function(
        &self,
        name: Option<arena::Box<'alloc, BindingIdentifier<'alloc>>>,
        params: arena::Box<'alloc, FormalParameters<'alloc>>,
        body: arena::Box<'alloc, FunctionBody<'alloc>>,
    ) -> Function<'alloc> {
        Function {
            name: name.map(|b| b.unbox()),
            is_async: false,
            is_generator: false,
            params: params.unbox(),
            body: body.unbox(),
        }
    }

    // AsyncFunctionDeclaration : `async` `function` BindingIdentifier `(` FormalParameters `)` `{` AsyncFunctionBody `}`
    // AsyncFunctionDeclaration : [+Default] `async` `function` `(` FormalParameters `)` `{` AsyncFunctionBody `}`
    // AsyncFunctionExpression : `async` `function` `(` FormalParameters `)` `{` AsyncFunctionBody `}`
    pub fn async_function(
        &self,
        name: Option<arena::Box<'alloc, BindingIdentifier<'alloc>>>,
        params: arena::Box<'alloc, FormalParameters<'alloc>>,
        body: arena::Box<'alloc, FunctionBody<'alloc>>,
    ) -> Function<'alloc> {
        Function {
            name: name.map(|b| b.unbox()),
            is_async: true,
            is_generator: false,
            params: params.unbox(),
            body: body.unbox(),
        }
    }

    // GeneratorDeclaration : `function` `*` BindingIdentifier `(` FormalParameters `)` `{` GeneratorBody `}`
    // GeneratorDeclaration : [+Default] `function` `*` `(` FormalParameters `)` `{` GeneratorBody `}`
    // GeneratorExpression : `function` `*` BindingIdentifier? `(` FormalParameters `)` `{` GeneratorBody `}`
    pub fn generator(
        &self,
        name: Option<arena::Box<'alloc, BindingIdentifier<'alloc>>>,
        params: arena::Box<'alloc, FormalParameters<'alloc>>,
        body: arena::Box<'alloc, FunctionBody<'alloc>>,
    ) -> Function<'alloc> {
        Function {
            name: name.map(|b| b.unbox()),
            is_async: false,
            is_generator: true,
            params: params.unbox(),
            body: body.unbox(),
        }
    }

    // AsyncGeneratorDeclaration : `async` `function` `*` BindingIdentifier `(` FormalParameters `)` `{` AsyncGeneratorBody `}`
    // AsyncGeneratorDeclaration : [+Default] `async` `function` `*` `(` FormalParameters `)` `{` AsyncGeneratorBody `}`
    // AsyncGeneratorExpression : `async` `function` `*` BindingIdentifier? `(` FormalParameters `)` `{` AsyncGeneratorBody `}`
    pub fn async_generator(
        &self,
        name: Option<arena::Box<'alloc, BindingIdentifier<'alloc>>>,
        params: arena::Box<'alloc, FormalParameters<'alloc>>,
        body: arena::Box<'alloc, FunctionBody<'alloc>>,
    ) -> Function<'alloc> {
        Function {
            name: name.map(|b| b.unbox()),
            is_async: true,
            is_generator: true,
            params: params.unbox(),
            body: body.unbox(),
        }
    }

    // UniqueFormalParameters : FormalParameters
    pub fn unique_formal_parameters(
        &self,
        parameters: arena::Box<'alloc, FormalParameters<'alloc>>,
    ) -> arena::Box<'alloc, FormalParameters<'alloc>> {
        // TODO
        parameters
    }

    // FormalParameters : [empty]
    pub fn empty_formal_parameters(&self) -> arena::Box<'alloc, FormalParameters<'alloc>> {
        self.alloc(FormalParameters {
            items: self.new_vec(),
            rest: None,
        })
    }

    // FormalParameters : FunctionRestParameter
    // FormalParameters : FormalParameterList `,` FunctionRestParameter
    pub fn with_rest_parameter(
        &self,
        mut params: arena::Box<'alloc, FormalParameters<'alloc>>,
        rest: arena::Box<'alloc, Binding<'alloc>>,
    ) -> arena::Box<'alloc, FormalParameters<'alloc>> {
        params.rest = Some(rest.unbox());
        params
    }

    // FormalParameterList : FormalParameter
    pub fn formal_parameter_list_single(
        &self,
        parameter: arena::Box<'alloc, Parameter<'alloc>>,
    ) -> arena::Box<'alloc, FormalParameters<'alloc>> {
        self.alloc(FormalParameters {
            items: self.new_vec_single(parameter.unbox()),
            rest: None,
        })
    }

    // FormalParameterList : FormalParameterList "," FormalParameter
    pub fn formal_parameter_list_append(
        &self,
        mut params: arena::Box<'alloc, FormalParameters<'alloc>>,
        next_param: arena::Box<'alloc, Parameter<'alloc>>,
    ) -> arena::Box<'alloc, FormalParameters<'alloc>> {
        self.push(&mut params.items, next_param.unbox());
        params
    }

    // FunctionBody : FunctionStatementList
    pub fn function_body(
        &self,
        statements: arena::Box<'alloc, arena::Vec<'alloc, Statement<'alloc>>>,
    ) -> arena::Box<'alloc, FunctionBody<'alloc>> {
        // TODO: Directives
        self.alloc(FunctionBody {
            directives: self.new_vec(),
            statements: statements.unbox(),
        })
    }

    // FunctionStatementList : StatementList?
    pub fn function_statement_list(
        &self,
        statements: Option<arena::Box<'alloc, arena::Vec<'alloc, Statement<'alloc>>>>,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, Statement<'alloc>>> {
        match statements {
            Some(statements) => statements,
            None => self.alloc(self.new_vec()),
        }
    }

    // ArrowFunction : ArrowParameters `=>` ConciseBody
    pub fn arrow_function(
        &self,
        params: arena::Box<'alloc, FormalParameters<'alloc>>,
        body: arena::Box<'alloc, ArrowExpressionBody<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::ArrowExpression(ArrowExpression {
            is_async: false,
            params: params.unbox(),
            body: body.unbox(),
        }))
    }

    // ArrowParameters : BindingIdentifier
    pub fn arrow_parameters_bare(
        &self,
        identifier: arena::Box<'alloc, BindingIdentifier<'alloc>>,
    ) -> arena::Box<'alloc, FormalParameters<'alloc>> {
        self.alloc(FormalParameters {
            items: self.new_vec_single(Parameter::Binding(Binding::BindingIdentifier(
                identifier.unbox(),
            ))),
            rest: None,
        })
    }

    // ArrowParameters : CoverParenthesizedExpressionAndArrowParameterList
    pub fn uncover_arrow_parameters(
        &self,
        covered: arena::Box<'alloc, CoverParenthesized<'alloc>>,
    ) -> Result<'alloc, arena::Box<'alloc, FormalParameters<'alloc>>> {
        Ok(match covered.unbox() {
            CoverParenthesized::Expression(expression) => self.alloc(FormalParameters {
                items: self.expression_to_parameter_list(expression)?,
                rest: None,
            }),
            CoverParenthesized::Parameters(parameters) => parameters,
        })
    }

    // ConciseBody : [lookahead != `{`] AssignmentExpression
    pub fn concise_body_expression(
        &self,
        expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, ArrowExpressionBody<'alloc>> {
        self.alloc(ArrowExpressionBody::Expression(expression))
    }

    // ConciseBody : `{` FunctionBody `}`
    pub fn concise_body_block(
        &self,
        body: arena::Box<'alloc, FunctionBody<'alloc>>,
    ) -> arena::Box<'alloc, ArrowExpressionBody<'alloc>> {
        self.alloc(ArrowExpressionBody::FunctionBody(body.unbox()))
    }

    // MethodDefinition : PropertyName `(` UniqueFormalParameters `)` `{` FunctionBody `}`
    pub fn method_definition(
        &self,
        name: arena::Box<'alloc, PropertyName<'alloc>>,
        params: arena::Box<'alloc, FormalParameters<'alloc>>,
        body: arena::Box<'alloc, FunctionBody<'alloc>>,
    ) -> arena::Box<'alloc, MethodDefinition<'alloc>> {
        self.alloc(MethodDefinition::Method(Method {
            name: name.unbox(),
            is_async: false,
            is_generator: false,
            params: params.unbox(),
            body: body.unbox(),
        }))
    }

    // MethodDefinition : `get` PropertyName `(` `)` `{` FunctionBody `}`
    pub fn getter(
        &self,
        name: arena::Box<'alloc, PropertyName<'alloc>>,
        body: arena::Box<'alloc, FunctionBody<'alloc>>,
    ) -> arena::Box<'alloc, MethodDefinition<'alloc>> {
        self.alloc(MethodDefinition::Getter(Getter {
            property_name: name.unbox(),
            body: body.unbox(),
        }))
    }

    // MethodDefinition : `set` PropertyName `(` PropertySetParameterList `)` `{` FunctionBody `}`
    pub fn setter(
        &self,
        name: arena::Box<'alloc, PropertyName<'alloc>>,
        parameter: arena::Box<'alloc, Parameter<'alloc>>,
        body: arena::Box<'alloc, FunctionBody<'alloc>>,
    ) -> arena::Box<'alloc, MethodDefinition<'alloc>> {
        self.alloc(MethodDefinition::Setter(Setter {
            property_name: name.unbox(),
            param: parameter.unbox(),
            body: body.unbox(),
        }))
    }

    // GeneratorMethod : `*` PropertyName `(` UniqueFormalParameters `)` `{` GeneratorBody `}`
    pub fn generator_method(
        &self,
        name: arena::Box<'alloc, PropertyName<'alloc>>,
        params: arena::Box<'alloc, FormalParameters<'alloc>>,
        body: arena::Box<'alloc, FunctionBody<'alloc>>,
    ) -> arena::Box<'alloc, MethodDefinition<'alloc>> {
        self.alloc(MethodDefinition::Method(Method {
            name: name.unbox(),
            is_async: false,
            is_generator: true,
            params: params.unbox(),
            body: body.unbox(),
        }))
    }

    // YieldExpression : `yield`
    // YieldExpression : `yield` AssignmentExpression
    pub fn yield_expr(
        &self,
        operand: Option<arena::Box<'alloc, Expression<'alloc>>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::YieldExpression(YieldExpression {
            expression: operand,
        }))
    }

    // YieldExpression : `yield` `*` AssignmentExpression
    pub fn yield_star_expr(
        &self,
        operand: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::YieldGeneratorExpression(
            YieldGeneratorExpression {
                expression: operand,
            },
        ))
    }

    // AsyncGeneratorMethod ::= "async" "*" PropertyName "(" UniqueFormalParameters ")" "{" AsyncGeneratorBody "}" => AsyncGeneratorMethod($0, $1, $2, $3, $4, $5, $6, $7, $8)
    pub fn async_generator_method(
        &self,
        name: arena::Box<'alloc, PropertyName<'alloc>>,
        params: arena::Box<'alloc, FormalParameters<'alloc>>,
        body: arena::Box<'alloc, FunctionBody<'alloc>>,
    ) -> arena::Box<'alloc, MethodDefinition<'alloc>> {
        self.alloc(MethodDefinition::Method(Method {
            name: name.unbox(),
            is_async: true,
            is_generator: true,
            params: params.unbox(),
            body: body.unbox(),
        }))
    }

    // ClassDeclaration : `class` BindingIdentifier ClassTail
    // ClassDeclaration : `class` ClassTail
    pub fn class_declaration(
        &self,
        name: Option<arena::Box<'alloc, BindingIdentifier<'alloc>>>,
        tail: arena::Box<'alloc, ClassExpression<'alloc>>,
    ) -> arena::Box<'alloc, Statement<'alloc>> {
        let tail = tail.unbox();
        self.alloc(Statement::ClassDeclaration(ClassDeclaration {
            name: match name {
                None => BindingIdentifier {
                    name: Identifier { value: "default" },
                },
                Some(bi) => bi.unbox(),
            },
            super_: tail.super_,
            elements: tail.elements,
        }))
    }

    // ClassExpression : `class` BindingIdentifier? ClassTail
    pub fn class_expression(
        &self,
        name: Option<arena::Box<'alloc, BindingIdentifier<'alloc>>>,
        mut tail: arena::Box<'alloc, ClassExpression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        tail.name = name.map(|boxed| boxed.unbox());
        self.alloc(Expression::ClassExpression(tail.unbox()))
    }

    // ClassTail : ClassHeritage? `{` ClassBody? `}`
    pub fn class_tail(
        &self,
        heritage: Option<arena::Box<'alloc, Expression<'alloc>>>,
        body: Option<arena::Box<'alloc, arena::Vec<'alloc, ClassElement<'alloc>>>>,
    ) -> arena::Box<'alloc, ClassExpression<'alloc>> {
        self.alloc(ClassExpression {
            name: None,
            super_: heritage,
            elements: match body {
                None => self.new_vec(),
                Some(boxed) => boxed.unbox(),
            },
        })
    }

    // ClassElementList : ClassElementList ClassElement
    pub fn class_element_list_append(
        &self,
        mut list: arena::Box<'alloc, arena::Vec<'alloc, ClassElement<'alloc>>>,
        mut element: arena::Box<'alloc, arena::Vec<'alloc, ClassElement<'alloc>>>,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, ClassElement<'alloc>>> {
        self.append(&mut list, &mut element);
        list
    }

    // ClassElement : MethodDefinition
    pub fn class_element(
        &self,
        method: arena::Box<'alloc, MethodDefinition<'alloc>>,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, ClassElement<'alloc>>> {
        self.alloc(self.new_vec_single(ClassElement {
            is_static: false,
            method: method.unbox(),
        }))
    }

    // ClassElement : `static` MethodDefinition
    pub fn class_element_static(
        &self,
        method: arena::Box<'alloc, MethodDefinition<'alloc>>,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, ClassElement<'alloc>>> {
        self.alloc(self.new_vec_single(ClassElement {
            is_static: true,
            method: method.unbox(),
        }))
    }

    // ClassElement : `;`
    pub fn class_element_empty(
        &self,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, ClassElement<'alloc>>> {
        self.alloc(self.new_vec())
    }

    // AsyncMethod : `async` PropertyName `(` UniqueFormalParameters `)` `{` AsyncFunctionBody `}`
    pub fn async_method(
        &self,
        name: arena::Box<'alloc, PropertyName<'alloc>>,
        params: arena::Box<'alloc, FormalParameters<'alloc>>,
        body: arena::Box<'alloc, FunctionBody<'alloc>>,
    ) -> arena::Box<'alloc, MethodDefinition<'alloc>> {
        self.alloc(MethodDefinition::Method(Method {
            name: name.unbox(),
            is_async: true,
            is_generator: false,
            params: params.unbox(),
            body: body.unbox(),
        }))
    }

    // AwaitExpression : `await` UnaryExpression
    pub fn await_expr(
        &self,
        operand: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::AwaitExpression(AwaitExpression {
            expression: operand,
        }))
    }

    // AsyncArrowFunction : `async` AsyncArrowBindingIdentifier `=>` AsyncConciseBody
    // AsyncArrowFunction : CoverCallExpressionAndAsyncArrowHead `=>` AsyncConciseBody
    pub fn async_arrow_function(
        &self,
        params: arena::Box<'alloc, FormalParameters<'alloc>>,
        body: arena::Box<'alloc, ArrowExpressionBody<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        self.alloc(Expression::ArrowExpression(ArrowExpression {
            is_async: true,
            params: params.unbox(),
            body: body.unbox(),
        }))
    }

    // AsyncArrowFunction : CoverCallExpressionAndAsyncArrowHead `=>` AsyncConciseBody
    //
    // This is used to convert the Expression that is produced by parsing a CoverCallExpressionAndAsyncArrowHead
    pub fn async_arrow_parameters(
        &self,
        call_expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> Result<'alloc, arena::Box<'alloc, FormalParameters<'alloc>>> {
        match call_expression.unbox() {
            Expression::CallExpression(CallExpression {
                callee: ce,
                arguments,
            }) => {
                // Check that `callee` is `async`.
                match ce {
                    ExpressionOrSuper::Expression(callee) => match callee.unbox() {
                        Expression::IdentifierExpression(IdentifierExpression { name }) => {
                            if name.value != "async" {
                                // `foo(a, b) => {}`
                                return Err(ParseError::ArrowHeadInvalid);
                            }
                        }
                        _ => {
                            // `obj.async() => {}`
                            return Err(ParseError::ArrowHeadInvalid);
                        }
                    },

                    ExpressionOrSuper::Super => {
                        // Can't happen: `super()` doesn't match
                        // CoverCallExpressionAndAsyncArrowHead.
                        return Err(ParseError::ArrowHeadInvalid);
                    }
                }

                self.arguments_to_parameter_list(arguments.args)
            }
            _ => {
                // The grammar ensures that the parser always passes
                // a valid CallExpression to this function.
                panic!("invalid argument");
            }
        }
    }

    // CoverCallExpressionAndAsyncArrowHead : MemberExpression Arguments
    pub fn cover_call_expression_and_async_arrow_head(
        &self,
        callee: arena::Box<'alloc, Expression<'alloc>>,
        arguments: arena::Box<'alloc, Arguments<'alloc>>,
    ) -> arena::Box<'alloc, Expression<'alloc>> {
        // TODO
        self.alloc(Expression::CallExpression(CallExpression {
            callee: ExpressionOrSuper::Expression(self.alloc(callee.unbox())),
            arguments: arguments.unbox(),
        }))
    }

    // Script : ScriptBody?
    pub fn script(
        &self,
        script: Option<arena::Box<'alloc, Script<'alloc>>>,
    ) -> arena::Box<'alloc, Script<'alloc>> {
        match script {
            Some(script) => script,
            None => self.alloc(Script {
                directives: self.new_vec(),
                statements: self.new_vec(),
            }),
        }
    }

    // ScriptBody : StatementList
    pub fn script_body(
        &self,
        statements: arena::Box<'alloc, arena::Vec<'alloc, Statement<'alloc>>>,
    ) -> arena::Box<'alloc, Script<'alloc>> {
        // TODO: directives
        self.alloc(Script {
            directives: self.new_vec(),
            statements: statements.unbox(),
        })
    }

    // Module : ModuleBody?
    pub fn module(
        &self,
        body: Option<arena::Box<'alloc, arena::Vec<'alloc, Statement<'alloc>>>>,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, Statement<'alloc>>> {
        body.unwrap_or_else(|| self.alloc(self.new_vec()))
    }

    // ModuleItemList : ModuleItem
    pub fn module_item_list_single(
        &self,
        item: arena::Box<'alloc, Statement<'alloc>>,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, Statement<'alloc>>> {
        self.alloc(self.new_vec_single(item.unbox()))
    }

    // ModuleItemList : ModuleItemList ModuleItem
    pub fn module_item_list_append(
        &self,
        mut list: arena::Box<'alloc, arena::Vec<'alloc, Statement<'alloc>>>,
        item: arena::Box<'alloc, Statement<'alloc>>,
    ) -> arena::Box<'alloc, arena::Vec<'alloc, Statement<'alloc>>> {
        self.push(&mut list, item.unbox());
        list
    }

    // ImportDeclaration : `import` ImportClause FromClause `;`
    // ImportDeclaration : `import` ModuleSpecifier `;`
    pub fn import_declaration(
        &self,
        _import_clause: Option<arena::Box<'alloc, Void>>,
        _module_specifier: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // ImportClause : ImportedDefaultBinding
    // ImportClause : NameSpaceImport
    // ImportClause : NamedImports
    // ImportClause : ImportedDefaultBinding `,` NameSpaceImport
    // ImportClause : ImportedDefaultBinding `,` NamedImports
    pub fn import_clause(
        &self,
        _default_binding: Option<arena::Box<'alloc, BindingIdentifier<'alloc>>>,
        _name_space_import: Option<arena::Box<'alloc, Void>>,
        _named_imports: Option<arena::Box<'alloc, Void>>,
    ) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // NameSpaceImport : `*` `as` ImportedBinding
    pub fn name_space_import(
        &self,
        _name: arena::Box<'alloc, BindingIdentifier<'alloc>>,
    ) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // NamedImports : `{` `}`
    pub fn imports_list_empty(&self) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // ImportsList : ImportSpecifier
    // ImportsList : ImportsList `,` ImportSpecifier
    pub fn imports_list_append(
        &self,
        _list: arena::Box<'alloc, Void>,
        _item: arena::Box<'alloc, Void>,
    ) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // ImportSpecifier : ImportedBinding
    pub fn import_specifier(
        &self,
        _name: arena::Box<'alloc, BindingIdentifier<'alloc>>,
    ) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // ImportSpecifier : IdentifierName `as` ImportedBinding
    pub fn import_specifier_renaming(
        &self,
        _original_name: arena::Box<'alloc, Token<'alloc>>,
        _local_name: arena::Box<'alloc, BindingIdentifier<'alloc>>,
    ) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // ModuleSpecifier : StringLiteral
    pub fn module_specifier(
        &self,
        _token: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, Token<'alloc>> {
        unimplemented!();
    }

    // ExportDeclaration : `export` `*` FromClause `;`
    pub fn export_all_from(
        &self,
        _module_specifier: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // ExportDeclaration : `export` ExportClause FromClause `;`
    pub fn export_set_from(
        &self,
        _export_clause: arena::Box<'alloc, Void>,
        _module_specifier: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // ExportDeclaration : `export` ExportClause `;`
    pub fn export_set(&self, _export_clause: arena::Box<'alloc, Void>) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // ExportDeclaration : `export` VariableStatement
    pub fn export_vars(
        &self,
        _statement: arena::Box<'alloc, Statement<'alloc>>,
    ) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // ExportDeclaration : `export` Declaration
    pub fn export_declaration(
        &self,
        _declaration: arena::Box<'alloc, Statement<'alloc>>,
    ) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // ExportDeclaration : `export` `default` HoistableDeclaration
    pub fn export_default_hoistable(
        &self,
        _declaration: arena::Box<'alloc, Statement<'alloc>>,
    ) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // ExportDeclaration : `export` `default` ClassDeclaration
    pub fn export_default_class(
        &self,
        _class_declaration: arena::Box<'alloc, Statement<'alloc>>,
    ) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // ExportDeclaration : `export` `default` [lookahead <! {`function`, `async`, `class`}] AssignmentExpression `;`
    pub fn export_default_value(
        &self,
        _expression: arena::Box<'alloc, Expression<'alloc>>,
    ) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // ExportClause : `{` `}`
    pub fn exports_list_empty(&self) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // ExportsList : ExportSpecifier
    // ExportsList : ExportsList `,` ExportSpecifier
    pub fn exports_list_append(
        &self,
        _list: arena::Box<'alloc, Void>,
        _export_specifier: arena::Box<'alloc, Void>,
    ) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // ExportSpecifier : IdentifierName
    pub fn export_specifier(
        &self,
        _identifier: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }

    // ExportSpecifier : IdentifierName `as` IdentifierName
    pub fn export_specifier_renaming(
        &self,
        _local_name: arena::Box<'alloc, Token<'alloc>>,
        _exported_name: arena::Box<'alloc, Token<'alloc>>,
    ) -> arena::Box<'alloc, Void> {
        unimplemented!();
    }
}
