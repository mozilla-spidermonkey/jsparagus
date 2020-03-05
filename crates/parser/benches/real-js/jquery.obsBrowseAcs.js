obs.browse = (function ($) {

    var config = {
        
    };

    var _initialize = function (options) {
        if (options) {
            obs.utils.merge(config, options);
        }
    };

    var _addSearchOption = function (label, allValuesList, newValuesList) {
        return $.ajax({
            type: 'GET',
            url: "/api/browse/acs/" + label + "/" + allValuesList + "/" + newValuesList,
            dataType: "html",
            error: function (XMLHttpRequest, textStatus, errorThrown) {
                //$.uiDialog(errorThrown, 'Error', 'Warning');
            },
            success: function(data) {
            	replaceSelects(label, data);
            }
        });
    };
	
	var replaceSelects = function (label, contents) {
		var containingTh = document.getElementById(label);
		//$(containingTh).fadeOut(0);
		containingTh.innerHTML = contents;
		$(containingTh).fadeIn();
	};
    
	var addSearchOption = function (label, value) {
		//update select set flag
		var acsSelectSetFlagInput = document.getElementById('acs_select_set');
		newAcsSelectSetFlag = parseInt(acsSelectSetFlagInput.value) + 1;
		acsSelectSetFlagInput.value = newAcsSelectSetFlag.toString();

		valuesListId = 'selected_' + label + '_list';
		valuesListInput = document.getElementById(valuesListId);
		valuesList = valuesListInput.value;
		if (valuesList == '' || valuesList == '0') {
			newValuesList = value;
		} else {
			newValuesList = valuesList + '_' + value;
		}
		
		allValuesListId = label + '_list_all';
		allValuesListInput = document.getElementById(allValuesListId);
		allValuesList = allValuesListInput.value;
				
		valuesListInput.value = newValuesList;
		
        var xhr = _addSearchOption(label, allValuesList, newValuesList);
        
    };
    
	var removeSearchOption = function (label, value) {
		//update select set flag
		var acsSelectSetFlagInput = document.getElementById('acs_select_set');
		newAcsSelectSetFlag = parseInt(acsSelectSetFlagInput.value) - 1;
		if (newAcsSelectSetFlag < 0) {
			newAcsSelectSetFlag = 0;
		}
		acsSelectSetFlagInput.value = newAcsSelectSetFlag.toString();
	
		allValuesListId = label + '_list_all';
		allValuesListInput = document.getElementById(allValuesListId);
		allValuesList = allValuesListInput.value;

		valuesListId = 'selected_' + label + '_list';
		valuesListInput = document.getElementById(valuesListId);
		valuesList = valuesListInput.value;
		
		if (valuesList.search('_' + value + '_') > -1) {
			valuesList = valuesList.replace('_' + value + '_', '_');
		} else if (valuesList.search('_' + value) > -1) {
			valuesList = valuesList.replace('_' + value, '');
		} else if (valuesList.search(value + '_') > -1) {
			valuesList = valuesList.replace(value + '_', '');
		} else if (valuesList.search(value) > -1) {
			valuesList = '0';
		}
	
		newValuesList = valuesList;
		valuesListInput.value = newValuesList;
										
        var xhr = _addSearchOption(label, allValuesList, newValuesList);
        
    };

	var startOver = function (location) {
		var url = document.URL;
		var acsSelectSetFlagInput = document.getElementById('acs_select_set');
		var acsSelectSetFlag = acsSelectSetFlagInput.value;
		
		if (parseInt(acsSelectSetFlag) > 0) {
		
			$("input[id^='selected_acs_']").each(function() {
				$(this).val(0);
				label = $(this).attr("name");
				allValuesListId = label + '_list_all';
				allValuesListInput = document.getElementById(allValuesListId);
				allValuesList = allValuesListInput.value;
				
				var xhr = _addSearchOption(label, allValuesList, '0');
				
			});

			//update select set flag
			acsSelectSetFlagInput.value = '0';

			$('#acs_modify_search').fadeOut();
			$('#search_bar_advanced_controls:hidden').slideToggle();
			$('#hottest_slider_row:hidden').slideToggle();
			$('#newest_slider_row:hidden').slideToggle();
			
		} else {
			window.location.href = location;
		}

    };

	var resetExtraRows = function () {
		$(".selected_acs_extra_row_list").each(function() {
			$(this).val(0);
			label = $(this).attr("name");
			allValuesListId = label + '_list_all';
			allValuesListInput = document.getElementById(allValuesListId);
			allValuesList = allValuesListInput.value;
		
			var xhr = _addSearchOption(label, allValuesList, '0');
		});
    };
    
	var acsToggleExtras = function() {
		if ($("#acs_other_search_options").text() == 'Show all card search options') {
		
			$("tr[id^='acs_extra_row_']").each(function() {
				$(this).fadeIn();
			});
			$("#acs_show_extra_rows").val('1');
			$("#acs_other_search_options").text('Hide extra search options');
			
		} else {
		
			$("tr[id^='acs_extra_row_']").each(function() {
				$(this).fadeOut();
				//resetExtraRows();
			});
			$("#acs_show_extra_rows").val('0');
			$("#acs_other_search_options").text('Show all card search options');

		}
	};

	var replaceProductListing = function (label, contents) {
		var containingTd = document.getElementById(label);
		$(containingTd).fadeIn();
		containingTd.innerHTML = contents;
	};

	var loadCard = function (productId, manuId, productPrice, cardCount) {
		var containingDiv = '#acs_cf_' + productId;
		var title = $(containingDiv).text();
		var discussLink = '/product/' + productId + '#ctrl_comments';
		$('#acs_discuss').attr("href", discussLink);
		$('#acs_center_content').fadeTo(400, 0.1);
		$('#acs_image').fadeTo(400, 0.1);
		$('#acs_card_count').val(cardCount);
		var cPath = $('input[name=cPath]').val();

		if ($('#flag_link').attr('href').search('login.php') == -1) {
			$('#acs_flag_submit_button').attr('onclick', "obs.browse.acsFlagSubmit('" + productId + "');");
		}

		if ($('input[name=redirect_url]').val().search('acs_cc') != -1) {

			redirectValue = $('input[name=redirect_url]').val();
			redirectValue = redirectValue.replace(/acs_cc=[0-9]+/, 'acs_cc=' + cardCount);
			$('input[name=redirect_url]').val(redirectValue);

		} else {
			redirectValue = $('input[name=redirect_url]').val() + '&acs_cc=' + cardCount;
			$('input[name=redirect_url]').val(redirectValue);
		}

		$("div[id^='acs_cf']").each(function() {
			$(this).css({"background-color": "#ffffff", "font-weight": "normal"});
		});
		$(containingDiv).css({"background-color": "#f9f2d6", "font-weight": "bold"});
		$('#acs_image').attr('src', '../images/' + manuId + '/' + productId + '-thumb350.jpg').attr('alt', title);
		$('#acs_image').on("load", function() {
			$(this).fadeTo(400, 1);
		});

		return $.ajax({
			type: 'GET',
			url: "/api/browse/acs_load/" + productId + "/" + productPrice + "/" + cPath,
			dataType: "json",
			error: function (XMLHttpRequest, textStatus, errorThrown) {
				//$.uiDialog(errorThrown, 'Error', 'Warning');
			},
			success: function(data) {
				replaceCard(data);
			}
		});
	};
	
	var replaceCard = function (data) {
		var containingTd = '#acs_center_content';
		var linkContainingTd = '#flag_link_td';

		$(linkContainingTd).html(data[0]);
		$(containingTd).html(data[1]);
		$(containingTd).fadeTo(400, 1);
	};

	var acsSearch = function (categoryHead,cardCounter,cPath) {
		var overallSelectedValuesList = '';
		var labelsList = '';
		var hrefLink = window.location.pathname;
		var showExtraRows = $('#acs_show_extra_rows').val();

		if (showExtraRows == '1') {
			var showExtraRowsParam = '&show_acs=1';
		} else {
			var showExtraRowsParam = '';
		}

		$("input[id^='selected_acs_']").each(function() {
			if ($(this).val() != '0') {
				overallSelectedValuesList = overallSelectedValuesList + 'X' + $(this).val();
				labelsList = labelsList + $(this).attr('name').replace('acs_','QQ');
			}			
		});
		labelsList = labelsList.replace('QQ','');
		
		$("tr[id^='acs_slider']").each(function() {
			$(this).remove();
		});
		
		$('#acs_product_listing').fadeOut();
		
		var overallSelectedValuesList = overallSelectedValuesList.replace('X','');

		if (overallSelectedValuesList != '') {

			$('input[name=redirect_url]').val(hrefLink + '?acs_vl=' + overallSelectedValuesList + '&acs_ll=' + labelsList + showExtraRowsParam + '&acs_cc=' + cardCounter + '&cPath=' + cPath);
			return $.ajax({
				type: 'GET',
				url: "/api/browse/acs_search/" + categoryHead + "/" + overallSelectedValuesList + "/" + labelsList + "/" + cardCounter + "/" + cPath,
				dataType: "html",
				error: function (XMLHttpRequest, textStatus, errorThrown) {
					//$.uiDialog(errorThrown, 'Error', 'Warning');
				},
				success: function(data) {
					replaceProductListing('acs_product_listing', data);
				}
			});
			
		} else {
			location.reload();
		}
    };
    
	var feedback = function (productId) {
		alert('Thank you for your feedback.');
		window.location.href = '/contact_us.php';
	};

	var acsFlagSubmit = function (productId) {
		if ($('input[name="flag_type"]:checked').length) {
			checkedValue = $('input[name="flag_type"]:checked').val();
			if (checkedValue == '3') {
				otherText = $('#flag_other_text').val();
				otherText = encodeURIComponent(otherText.replace("+", "%2b"));
			} else {
				otherText = 'none';
			}

			$('#flag_form').fadeToggle();
			$('#flag_form_wrapper').fadeToggle();
			$('#flag_link').text('Thanks for the feedback');
			$("#flag_link").attr('onclick', "alert('You have already flagged this card. Thanks for the feedback.'); return false;");
			return $.ajax({
				type: 'POST',
				url: '/api/browse/acs_flag/' + productId + '/' + checkedValue,
				dataType: "html",
				data: {other_text: otherText},
				error: function (XMLHttpRequest, textStatus, errorThrown) {
						//$.uiDialog(errorThrown, 'Error', 'Warning');
				},
				success: function(data) {
					//alert(data);
				}
			});
		} else {
			$("#flag_select_warning").slideDown();
		}
	};

    return {
        initialize: _initialize,
        addSearchOption: addSearchOption,
        removeSearchOption: removeSearchOption,
        acsToggleExtras: acsToggleExtras,
        resetExtraRows: resetExtraRows,
        replaceProductListing: replaceProductListing,
        acsSearch: acsSearch,
        loadCard: loadCard,
        replaceCard: replaceCard,
        feedback: feedback,
        acsFlagSubmit: acsFlagSubmit,
        startOver: startOver
    };

})(jQuery);