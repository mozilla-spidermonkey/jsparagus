$(document).ready(function() {
    // $("#suggestionFrame").addClass('footer_suggestionFrame');
    // $("#suggestionBox").addClass('footer_suggestionBox');
    $("#suggestionBox").attr('placeholder', obs.config.suggestions.default);

    $('#submit.suggestion-button').on('click', function() {
        var error = getSuggestionFormError('#suggestionBox');

        if (error != '') {
            alert(error);
            return;
        }

        if (checkSuggestionForm('#suggestionBox')) {
            $.ajax(
                {
                    datatype: "json",
                    url: "/includes/ajax/suggestion_box_ajax.php",
                    method: "POST",
                    data: {
                        contactme: $('#contactme').is(':checked'),
                        email: $('#email').val(),
                        suggestion: $('#suggestionBox').val(),
                        origin_href: $('#origin_href').val()
                    }
                }
            ).done(function(data) {
                var result = jQuery.parseJSON(data);
                if (result.email_result) {
                    $("#suggestionFrame").html(result.newform);
                }
            }).fail(function(xhr) {
                fail_callback(xhr);
            });
        }
    });

    $('#suggestionBox').on('focus', function() {
        if (this.style.color=='grey') {
            $('#extraInfo').slideDown('fast');
            this.style.color = 'black';
            this.value = '';
            this.rows=3;
        }
    });
    $('#suggestionBox').on('blur', function() {
        if (this.value == '') {
            this.style.color = 'grey';
            this.value = obs.config.suggestions.default;
            $('#extraInfo').slideUp('fast');
            this.rows=1;
        }
    });
});

function showContactNote() {
    checkEmail() ? $('#email').attr("placeholder", obs.config.text.optional) : $('#email').attr("placeholder", obs.config.text.simpleRequired.toUpperCase());
}

function checkEmail() {
    if($('#contactme').is(':checked') && $('#email').val() == '') { return false } else { return true; }
}

// Returns error message(s) for suggestion form, or a blank string if form is ok and ready to submit
function getSuggestionFormError(text_box) {
    if (!$(text_box).length) {
        return 'JavaScript Error: No suggestion box found';
    }

    if ($(text_box).val().length == 0 || $(text_box).val() == obs.config.suggestions.default) {
        return obs.config.suggestions.enterSomething;
    }

    if (!checkEmail()) {
        return obs.config.suggestions.enterEmail;
    }

    return '';
}

// Makes sure user has entered something before allowing them to continue.
function checkSuggestionForm(text_box) {
    if(getSuggestionFormError(text_box) == '') { return true } else { return false; }
}

