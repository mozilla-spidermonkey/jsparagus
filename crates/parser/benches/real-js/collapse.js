(function ($) {
	/**
	 * Add show/hide HTML so it can be referenced.
	 */
	OBS_Form.settings.showText = '<span class="fieldset-show">' + OBS_Form.t('(Show)') + '</span>';
	OBS_Form.settings.hideText = '<span class="fieldset-hide">' + OBS_Form.t('(Hide)') + '</span>';

	/**
	 * Toggle the visibility of a fieldset using smooth animations.
	 */
	OBS_Form.toggleFieldset = function (fieldset) {

		var $fieldset = $(fieldset);
		if ($fieldset.is('.form-collapsed')) {
			var $content = $('> .fieldset-wrapper', fieldset).hide();
			$fieldset
				.removeClass('form-collapsed')
				.trigger({ type: 'form-collapsed', value: false })
				.find('> legend span.fieldset-legend-prefix').html(OBS_Form.settings.hideText);
			$content.slideDown({
				duration: 'fast',
				easing: 'linear',
				complete: function () {
					OBS_Form.collapseScrollIntoView(fieldset);
					fieldset.animating = false;
				},
				step: function () {
					// Scroll the fieldset into view.
					OBS_Form.collapseScrollIntoView(fieldset);
				}
			});
		}
		else {
			$fieldset.trigger({ type: 'collapsed', value: true });
			$('> .fieldset-wrapper', fieldset).slideUp('fast', function () {
				$fieldset
					.addClass('form-collapsed')
					.find('> legend span.fieldset-legend-prefix').html(OBS_Form.settings.showText);
				fieldset.animating = false;
			});
		}
	};

	/**
	 * Scroll a given fieldset into view as much as possible.
	 */
	OBS_Form.collapseScrollIntoView = function (node) {
		var h = document.documentElement.clientHeight || document.body.clientHeight || 0;
		var offset = document.documentElement.scrollTop || document.body.scrollTop || 0;
		var posY = $(node).offset().top;
		var fudge = 55;
		if (posY + node.offsetHeight + fudge > h + offset) {
			if (node.offsetHeight > h) {
				window.scrollTo(0, posY);
			}
			else {
				window.scrollTo(0, posY + node.offsetHeight - h + fudge);
			}
		}
	};

	OBS_Form.behaviors.collapse = {
		attach: function (context, settings) {
			$('fieldset.form-collapsible', context).once('collapse', function () {
				var $fieldset = $(this);
				// Expand fieldset if there are errors inside, or if it contains an
				// element that is targeted by the URI fragment identifier.
				var anchor = location.hash && location.hash != '#' ? ', ' + location.hash : '';
				if ($fieldset.find('.error' + anchor).length) {
					$fieldset.removeClass('form-collapsed');
				}
				// Turn the legend into a clickable link, but retain span.fieldset-legend for CSS positioning.
				var $legend = $('> legend.fieldset-legend', this);
				$('<span class="fieldset-legend-prefix element-invisible"></span>')
					.append($fieldset.hasClass('form-collapsed') ? OBS_Form.settings.showText : OBS_Form.settings.hideText)
					.prependTo($legend)
					.after(' ');
				// .wrapInner() does not retain bound events.
				var $link = $('<a class="fieldset-title" href="#"></a>')
					.prepend($legend.contents())
					.appendTo($legend)
					.click(function () {
						var fieldset = $fieldset.get(0);
						// Don't animate multiple times.
						if (!fieldset.animating) {
							fieldset.animating = true;
							OBS_Form.toggleFieldset(fieldset);
						}
						return false;
					});
				// Lastly, anything that's collapsed on load - collapse it.
				$('fieldset.form-collapsed > .fieldset-wrapper').click();
			});
		}
	};

})(jQuery);
