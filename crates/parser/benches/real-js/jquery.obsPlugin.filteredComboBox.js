/**
 * Created by Doug on 11/9/2014.
 *
 * This plugin adds a new jQuery command obs_filteredComboBox which can be used on any valid
 * jquery node (it is intended to be used with a text input).  It takes the text input and
 * transforms it into one of our site searchable/filterable dropdown boxes (such as the searchable
 * publisher drop-down selector).
 */
/*!
 * jQuery lightweight plugin boilerplate
 * Original author: @ajpiano
 * Further changes, comments: @addyosmani
 * Licensed under the MIT license
 */

// the semi-colon before the function invocation is a safety
// net against concatenated scripts and/or other plugins
// that are not closed properly.
;(function ( $, window, document, undefined ) {

	if (!$.obs) {
		$.obs = {};
	};

	$.obs.filteredComboBox = function ( el, options ) {
		// To avoid scope issues, use 'base' instead of 'this'
		// to reference this class from internal events and functions.
		var base = this;

		// Create the container element to hold the widget
		base.$widget = $('<span></span>');
		base.$widget.addClass('obs-filtered-combobox');
		// Access to jQuery and DOM versions of element
		base.$input = $(el).replaceWith(base.$widget);
		base.input  = el;

		base.$input.attr('autocomplete','off');
		base.$input.addClass('nav-counter-select');
		base.$input.appendTo(base.$widget);

		//Create the Spin.JS spinner which we will use as our loading graphic
		base.$spinner = $('<span></span>');
		base.$spinner.css("position","relative");
		base.$spinner.css("bottom","18px");
		base.$spinner.css("left","45px");
		base.$spinner.spin('tiny','grey');
		base.$spinner.hide();
		base.$spinner.appendTo(base.$widget);

		//Create the button element which serves to "show all"
		base.$button = $('<button></button>');
		base.$button.attr('role','button');
		base.$button.attr('type','button');
		base.$button.attr('tabindex','-1');
		base.$button.addClass('ui-button ui-widget ui-state-default ui-button-icon-only ui-corner-right ui-button-icon');
		base.$button.addClass('nav-combobox-button icon-ui-dd-btn');

		//Hover-in / Hover-out visual state change
		base.$button.on('hover', function() {
			var el = $(this);
			el.toggleClass('ui-state-hover');
			el.toggleClass('ui-state-default');
		});
		//Iconography for the drop down button
		var span = $('<span></span>');
		span.addClass('ui-button-icon-primary ui-icon ui-icon-triangle-1-s');

		span.appendTo(base.$button);

		base.$button.appendTo(base.$widget);

		// Add a reverse reference to the DOM object
		base.$widget.data( "obs.filteredComboBox" , base );

		/*
		* This is our init function which sets up the functionality of the widget after it has been
		* built.
		 */
		base.init = function () {

			//Search cache, to increase performance and reduce server hits
			var cache = {};

			base.options = $.extend({},
				$.obs.filteredComboBox.defaultOptions, options);

			//Set the placeholder text and on-hover title attributes
			base.$input.attr('placeholder',base.options.placeholder_text);
			base.$button.attr('title',"Show all "+base.options.item_title)

			//Init our jqueryUI provided autocomplete/autosuggest drop down
			base.autocomplete = base.$input.autocomplete(
				{
					source: function (request, response) {
						var term = request.term;
						base.options.ajax.rq_data["term"] = term;

						if (term in cache) {
							response(cache[term]);
							base.$spinner.hide();
							return;
						}
						$.ajax({
							url: base.options.ajax.url,
							dataType: "json",
							data: base.options.ajax.rq_data,
							success: function (data) {
								cache[term] = data;
								base.$spinner.hide();
								response(data);
							}
						});
					},
					delay: base.options.ajax.delay,
					minLength: base.options.ajax.minLength,
					html: 'html',
					//On open of the autocomplete we add a class which prevent it from over-running the
					//screen if we have a large number of elements (such as is the case with publishers)
					open: function () {
						base.$input.data("autocomplete").menu.element.addClass("ui-ac-combobox");
					},
					//On hover we replace the text in the input with whatever is being hovered over
					focus: function (event, ui) {
						base.$input.val(ui.item.name);
					},
					//If something is selected, browse to that location
					select: function (event, ui) {
						window.location.replace(ui.item.link);

						return false;
					},
					//On close we remove the ON class from the button, so the spinner functionality
					//will also behave properly (our 'when to spin' logic looks for this).
					close: function (event, ui) {
						base.$button.removeClass('btn-on');
					}
				}
			);

			//The drop button to show all choices
			base.$button.on('click',function() {
				var el = $(this);
				el.toggleClass('btn-on');
				if(el.hasClass('btn-on')) {
					//First click, show the spinner and search for all results
					base.$spinner.show();
					base.$spinner.spin('tiny','grey');
					base.autocomplete.autocomplete("search","get_all");
				} else{
					//We're done doing our work, if they click again close the drop down
					base.autocomplete.autocomplete("close");
				}
				//Leave focus on the input
				base.autocomplete.focus();
			});

			//Return the widget, for chaining purposes
			return base.$widget;
		};

		// Sample Function, Uncomment to use
		// base.functionName = function( paramaters ){
		//
		// };
		// Run initializer
		base.init();
	};

	$.obs.filteredComboBox.defaultOptions = {
		ajax : {
			delay: 100,//the amount of time to wait before searching
			minLength: 1,//the mininum length of text to accept before starting a search
			rq_data : {
				//term is the reserved spot for whatever data entered into the text input to be sent
				term: ''//Any additional DATA to be sent to the AJAX endpoint should be included in here
			},
			url: ''//the endpoint to hit for AJAX results
		},
		item_title : 'Publishers',//The on-hover title to be applied
		placeholder_text : 'Search...'//the placeholder text to use in the text input
	};

	$.fn.obs_filteredComboBox = function
		( options ) {
		return this.each(function () {
			(new $.obs.filteredComboBox(this, options));
		});
	};

})( jQuery, window, document );