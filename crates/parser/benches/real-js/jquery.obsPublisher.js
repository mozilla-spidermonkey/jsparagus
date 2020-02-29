obs.publisher = (function ($) {

    var config = {};

    var _initialize = function (options) {
        if (options) {
            obs.utils.merge(config, options);
        }
    };

    var replaceTuckboxSelect = function (divId, data) {

        $(divId).html(data);

    };

    var updateTuckboxForm = function () {

        var productId = $('input[name=tuckbox_products_id]').val();
        var cardType = $('input[name=tuckbox_card_type]').val();
        var manufacturersId = $('input[name=tuckbox_manufacturers_id]').val();

        return $.ajax({
            type: 'GET',
            url: "/api/publisher/tuckbox_select/" + manufacturersId + "/" + productId + "/" + cardType,
            dataType: "json",
            error: function (XMLHttpRequest, textStatus, errorThrown) {
                //$.uiDialog(errorThrown, 'Error', 'Warning');
            },
            success: function (data) {
                replaceTuckboxSelect('#tuckbox_select', data.message);
                if (data.status == 'hide_select') {
                    $('#tuckbox_form').slideUp();
                } else if (data.status == 'show_select_hide_uploader') {
                    $('#tuckbox_div').slideDown();
                    $('#tuckbox_form').slideUp();
                } else {
                    $('#tuckbox_div').slideDown();
                    $('#tuckbox_form').slideDown();
                }
            }
        });

    };

    var updateTuckboxSize = function (tuckboxSize) {

        var productId = $('input[name=tuckbox_products_id]').val();
        var manufacturersId = $('input[name=tuckbox_manufacturers_id]').val();

        return $.ajax({
            type: 'GET',
            url: "/api/publisher/update_tuckbox_size/" + manufacturersId + "/" + productId + "/" + tuckboxSize,
            dataType: "json",
            error: function (XMLHttpRequest, textStatus, errorThrown) {
                //$.uiDialog(errorThrown, 'Error', 'Warning');
            },
            success: function (data) {
                if (data.status == 'disable_continue_card') {

                    $('#continue_button').css('opacity', 0.5);
                    $('#continue_button').css('cursor', 'default');
                    $('#continue_button').attr("onclick","$('return false;");
                    $('span#status_messages').append('You must upload a card print file before you can continue.<br /><br />');

                } else if (data.status == 'disable_continue_both') {

                    $('#continue_button').css('opacity', 0.5);
                    $('#continue_button').css('cursor', 'default');
                    $('#continue_button').attr("onclick","$('return false;");
                    $('span#status_messages').append('You must upload a card print file and a tuckbox print file before you can continue.<br /><br />');

                } else if (data.status == 'enable_continue') {

                    $('#continue_button').css('opacity', 1);
                    $('#continue_button').css('cursor', 'pointer');
                    $('#continue_button').attr("onclick","$('#other_options_form').submit();");

                }
            }
        });

    };

    var newTuckboxSizeSelected = function () {

        var tuckboxSize = $('select[name="tuckbox_size"]').val();
        obs.publisher.updateTuckboxSize(tuckboxSize);

        if (tuckboxSize == "0") {
            $('#tuckbox_form').slideUp();
        } else {
            $('#tuckbox_form').slideDown();
        }

    };

    var podFileCheck = function () {

        var productId = $('input[name=products_id]').val();
        var productOptionsValuesId = $('input[name=products_options_values_id]').val();

        return $.ajax({
            type: 'GET',
            url: "/api/publisher/pub_check_for_files/" + productId + "/" + productOptionsValuesId,
            dataType: "json",
            success: function (data) {

                if (data['status'] == 'success') {
                    // call the bleed check function
                    publisherGetBleedMessage();
                } else {
                    $('#upload_status_messages').html('<font color="red"><b>' + data['message'] + '</b>');
                    $('#upload_status_message_table').fadeIn();
                }

            }
        });

    };

    var publisherApprovedBleed = function () {

        var productId = $('input[name=products_id]').val();
        var productOptionsValuesId = $('input[name=products_options_values_id]').val();

        return $.ajax({
            type: 'GET',
            url: "/api/publisher/pub_approved_bleed/" + productId + "/" + productOptionsValuesId,
            dataType: "json",
            success: function (data) {

                if (data['status'] == 'success') {
                    // Submit the form
                    $('#other_options_form').submit();
                } else {

                    $('#upload_status_messages').html('<font color="red"><b>' + data['message'] + '</b>');
                    $('#bleed_message_wrapper').fadeToggle();
                    $('#bleed_message_box').fadeToggle();
                    $('#upload_status_message_table').fadeIn();

                }

            }
        });

    };

    var publisherWillReviseBleed = function () {

        $('#bleed_message_wrapper').fadeToggle();
        $('#bleed_message_box').fadeToggle();
        $('span#status_messages').append('<span style="color:red;font-weight:bold;">After revising the bleed in your file, reload this page and try your upload again.<br />Until then, no previously uploaded file will be replaced.</span><br /><br />');

    };

    var publisherGetBleedMessage = function () {

        var productId = $('input[name=products_id]').val();
        var productOptionsValuesId = $('input[name=products_options_values_id]').val();
        var lsiId = $('#trim_size').val();

        return $.ajax({
            type: 'GET',
            url: "/api/publisher/pub_get_bleed_message/" + productId + "/" + productOptionsValuesId + "/" + lsiId,
            dataType: "json",
            success: function (data) {

                if (data['status'] == 'success') {

                    // put the message in the box and show it
                    $('#bleed_message_box_content').html('');
                    $('#bleed_message_box_content').html(data['message']);
                    $('#bleed_message_wrapper').fadeToggle();
                    $('#bleed_message_box').fadeToggle();

                } else {
                    // remove message content, do not show box, submit the form
                    $('#bleed_message_box_content').html('');
                    $('#other_options_form').submit();
                }

            }
        });

    };

    // For print card upload page
    var uploadSuccess_card = function(file, serverData) {

        // if the tuckbox form exists, redraw tuckbox select (retains current size setting unless new card count is too high)
        if ($('#tuckbox_form').length > 0) {
            obs.publisher.updateTuckboxForm();
        }

        var productId = $('input[name=products_id]').val();

        // show spinner
        $('#continueSpinner').show();

        $("body").css("cursor", "default");

        // get new continue button status via AJAX call -- checks for existence of card file and tuckbox file if needed
        // AJAX returns 'enable' or 'disable' as appropriate.
        return $.ajax({
            type: 'GET',
            url: "/api/publisher/continue_button_status_card/" + productId,
            dataType: "json",
            success: function (data) {

                if (data['status'] == 'enable') {

                    if ($('#card_type').val() != '') {

                        $('#continue_button').css('opacity', 1);
                        $('#continue_button').css('cursor', 'pointer');
                        $('#continue_button').attr("onclick","$('#other_options_form').submit();");

                    } else {

                        $('#continue_button').css('opacity', 0.7);
                        $('#continue_button').css('cursor', 'default');
                        $('#continue_button').attr("onclick","$('span#status_messages').append('You must upload file(s) before continuing.<br /><br />'); return false;");

                    }

                } else if (data['status'] == 'disable') {

                    $('#continue_button').css('opacity', 0.7);
                    $('#continue_button').css('cursor', 'default');
                    $('#continue_button').attr("onclick","$('span#status_messages').append('You must upload file(s) before continuing.<br /><br />'); return false;");

                }

                // hide spinner
                $('#continueSpinner').hide();

            }
        });
    };

    // For print card upload page
    var uploadSuccess_tuckbox = function() {

        var productId = $('input[name=products_id]').val();
        console.log('tuck show spinner');
        // show spinner
        $('#continueSpinner').show();

        $("body").css("cursor", "default");

        // get new continue button status via AJAX call -- checks for existence of card file and tuckbox file if needed
        // AJAX returns 'enable' or 'disable' as appropriate.
        return $.ajax({
            type: 'GET',
            url: "/api/publisher/continue_button_status_card/" + productId,
            dataType: "json",
            success: function (data) {

                if (data['status'] == 'enable') {

                    $('#continue_button').css('opacity', 1);
                    $('#continue_button').css('cursor', 'pointer');
                    $('#continue_button').attr("onclick","$('#other_options_form').submit();");

                } else if (data['status'] == 'disable') {

                    $('#continue_button').css('opacity', 0.7);
                    $('#continue_button').css('cursor', 'default');
                    $('#continue_button').attr("onclick","$('span#status_messages').append('You must upload file(s) before continuing.<br /><br />'); return false;");

                }

                // hide spinner
                $('#continueSpinner').hide();

            }

        });
    };

    // For print book upload page
    var uploadSuccess_cover = function() {

        var productId = $('input[name=products_id]').val();
        var productOptionsValuesId = $('input[name=products_options_values_id]').val();
        console.log('cover show spinner');
        // show spinner
        $('#continueSpinner').show();

        $("body").css("cursor", "default");

        // get new continue button status via AJAX call -- checks for existence of card file and tuckbox file if needed
        // AJAX returns 'enable' or 'disable' as appropriate.
        return $.ajax({
            type: 'GET',
            url: "/api/publisher/continue_button_status_book/" + productId + "/" + productOptionsValuesId,
            dataType: "json",
            success: function (data) {

                if (data['status'] == 'enable') {

                    $('#continue_button').css('opacity', 1);
                    $('#continue_button').css('cursor', 'pointer');
                    $('#continue_button').attr("onclick", "$('#other_options_form').submit();");

                } else if (data['status'] == 'disable') {

                    $('#continue_button').css('opacity', 0.7);
                    $('#continue_button').css('cursor', 'default');
                    $('#continue_button').attr("onclick","$('span#status_messages').append('You must upload a cover and a contents file before continuing.<br /><br />'); return false;");

                }

                // hide spinner
                $('#continueSpinner').hide();

            }

        });
    };

    // For print book upload page
    var uploadSuccess_contents = function(show_bleed_message) {

        var productId = $('input[name=products_id]').val();
        var productOptionsValuesId = $('input[name=products_options_values_id]').val();
        console.log('cover show spinner');
        // show spinner
        $('#continueSpinner').show();

        $("body").css("cursor", "default");

        // get new continue button status via AJAX call -- checks for existence of card file and tuckbox file if needed
        // AJAX returns 'enable' or 'disable' as appropriate.
        return $.ajax({
            type: 'GET',
            url: "/api/publisher/continue_button_status_book/" + productId + "/" + productOptionsValuesId,
            dataType: "json",
            success: function (data) {

                if (data['status'] == 'enable') {

                    $('#continue_button').css('opacity', 1);
                    $('#continue_button').css('cursor', 'pointer');

                    if (show_bleed_message == 1) {
                        $('#continue_button').attr("onclick", "obs.publisher.publisherGetBleedMessage(); return false;");
                    } else {
                        $('#continue_button').attr("onclick", "$('#other_options_form').submit();");
                    }

                } else if (data['status'] == 'disable') {

                    $('#continue_button').css('opacity', 0.7);
                    $('#continue_button').css('cursor', 'default');
                    $('#continue_button').attr("onclick","$('span#status_messages').append('You must upload a cover and a contents file before continuing.<br /><br />'); return false;");

                }

                // hide spinner
                $('#continueSpinner').hide();

            }

        });
    };

    return {
    	initialize: _initialize,
		replaceTuckboxSelect: replaceTuckboxSelect,
		updateTuckboxSize: updateTuckboxSize,
		newTuckboxSizeSelected: newTuckboxSizeSelected,
        updateTuckboxForm: updateTuckboxForm,
        podFileCheck: podFileCheck,
		publisherApprovedBleed: publisherApprovedBleed,
        publisherWillReviseBleed: publisherWillReviseBleed,
		publisherGetBleedMessage: publisherGetBleedMessage,
        uploadSuccess_card: uploadSuccess_card,
        uploadSuccess_tuckbox: uploadSuccess_tuckbox,
        uploadSuccess_cover: uploadSuccess_cover,
        uploadSuccess_contents: uploadSuccess_contents
    };

})(jQuery);