var AnimatedAddToCart = {
	adding_to_cart: false,
	ANIMATED_ADD_TO_CART_SELECTOR: 'animated-add-to-cart',
	animate_elements_to_cart: function (element) {
		var html_to_restore = $(element).html();

		if (AnimatedAddToCart.adding_to_cart) {
			return false;
		}

		AnimatedAddToCart.show_spinner($(element));
		AnimatedAddToCart.adding_to_cart = true;

		var self = element;
		var products_id = QueryStringToJSON($(element).attr('href')).products_id;
		var cart = $('#header-cart-widget');
		var final_top_px = cart.css('top');
		if (cart.css('display') == 'none') {
			cart.show('fast', 'swing');
		}

		cart.animate({top: $('body').scrollTop() + 40 + 'px'}, 'fast', 'swing', function () {
			var addToCartLink = $(self).attr('href');
			addToCartLink = addToCartLink + '&mode=json';
			$.post(addToCartLink, "", function (data) {
				var imgtofly = AnimatedAddToCart.get_img_to_fly(self);
				var imgclone = AnimatedAddToCart.get_animated_clone(imgtofly, cart);

				imgclone.animate({'width': 0, 'height': 0}, function () {
					$(this).detach();
					set_quantity_hidden_increase(products_id, 0, function () {
						$('#header-cart-widget').animate({top: final_top_px }, 'slow', 'swing', function () {
							$(this).css('position', '');
							AnimatedAddToCart.adding_to_cart = false;
							$(self).removeAttr('style');
							$(self).html(html_to_restore);
						});
					});
				});
			});
			return false;
		});
	},
	show_spinner: function (jquery_elem) {
		jquery_elem.html("<img src='m/img/spinner.gif'>");
	},
	get_img_to_fly: function (jquery_elem) {
		var imgtoflyselector = $(jquery_elem).attr('img-to-fly');
		if (typeof imgtoflyselector !== typeof undefined && imgtoflyselector !== false) {
			return $(imgtoflyselector);
		}

		// default: this is the product listing img
		return $(jquery_elem).closest('.dtrpgListing-row').find('img').first();
	},
	get_animated_clone: function (imgtofly, cart) {
		var imgclone = imgtofly.clone().offset({top: imgtofly.offset().top, left: imgtofly.offset().left})
										.css({'opacity': '0.7', 'position': 'absolute', 'height': '208px', 'width': '132px', 'z-index': '1000'})
										.appendTo($('body'))
										.animate({
											'top': cart.offset().top + 10,
											'left': cart.offset().left + 30,
											'width': 55,
											'height': 55
										}, 1250, 'swing');

		return imgclone;
	}
};

$(document).ready(function () {
	var selector = '.' + AnimatedAddToCart.ANIMATED_ADD_TO_CART_SELECTOR;
	$('body').on('click', selector, function (e) {
		e.preventDefault();
		AnimatedAddToCart.animate_elements_to_cart(this);
	});
});