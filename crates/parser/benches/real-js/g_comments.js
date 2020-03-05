var g_id_product;
var g_n_open_blocks = 1;
var g_time_interval = 0;


function focusTextAreaComment() {
    //remove the info text on focus
    if ($(this).is('[first]')) {
        $(this).removeAttr('first');
        $(this).val('');
    }
}

function validateComment($el) {
    if ($el.is('[first]') || $el.val() == "") {
        //it's no allowed to submit the information comment
        return false;
    }

    if (!$el.val().replace(/\s/g, '').length) {
        // string only contained whitespace (ie. spaces, tabs or line breaks)
        return false;
    }

    if ($el.val().length > g_MAX_COMMENT_SIZE) {

        var $notpanel = $el.closest(".co_new_comment").find(".g_msg_limit_size");
        $notpanel.fadeIn(300);
        setTimeout(function() {
            $notpanel.fadeOut(300);
        }, 5000);
        return false;
    }
    return true;
}

function pad(num, size) {
    var s = num + "";
    while (s.length < size)
        s = "0" + s;
    return s;
}

$(document).ready(function() {
    // widget tab control 
    $('#ctrl_reviews').click(function() {
        $('#comments_widget').hide();
        $('#widget-product-reviews').show();

        $(this).addClass("sel");
        $('#ctrl_comments').removeClass("sel");
        $('.g_follow').hide();

    });

    $('#ctrl_comments').click(function() {
        $('#comments_widget').show();
        $('#widget-product-reviews').hide();

        $(this).addClass("sel")
        $('#ctrl_reviews').removeClass("sel");
        $('.g_follow').show();
    });

    $('textarea#new_comment').focus(focusTextAreaComment);

    // add the comment
    var uploading = false;
    $('#add_new_comment').click(function(event) {
        event.preventDefault();
        if (uploading) {
            return;
        }

        if (!validateComment($('textarea#new_comment')))
            return;

        uploading = true;

        var $button = $(this).hide();
        var $sn = $(this).siblings(".g_sending_note").show();


        $.ajax({
            url: 'includes/g_comments_widget/php/g_perform_comment.php',
            type: 'POST',
            data: {
                'comment': $('textarea#new_comment').val(),
                'id_product': g_id_product
            },
            datatype: 'JSON',
            success: function(json) {
                //refresh comment page
                g_list_refresh(json.id_comment);
                uploading = false;
                $button.show();
                $sn.hide();
                $('textarea#new_comment').val("");
            }
        })
    });

    $("#g_subscribe").change(function() {
        var st = $(this).attr("checked");
        st = !st ? 0 : 1;
        $.ajax({
            url: 'includes/g_comments_widget/php/g_comment_subscription.php',
            type: 'POST',
            data: 'st=' + st + '&products_id=' + g_id_product
        });
    });

    listHooks();

    g_time_interval = setInterval(function() {
        $(".g_time").each(function() {
            var n = $(this).attr("n_seconds") - 1;
            if (n <= 0) {
                $(this).siblings(".edit_ctrl,.delete_ctrl").remove();
                $(this).remove();
                return;
            }
            $(this).attr("n_seconds", n);
            $(this).html(pad(Math.floor(n / 60), 2) + ":" + pad(n % 60, 2));
        });
    }, 1000);

    $("a#goToAnchor").click(function(e) {
        /*e.preventDefault();
         $('html, body').animate({
         scrollTop: $("#anchor_widgets").offset().top
         }, 500);
         return false;*/

        e.preventDefault();
        if (!$('#widget-product-reviews').is(":visible")) {
            $('#ctrl_reviews').trigger("click");
        }
        var target = this.hash,
                $target = $(target);
        $('html, body').stop().animate({
            'scrollTop': $target.offset().top
        }, 900, 'swing', function() {
            window.location.hash = target;
        });
    });
});

function showCommentEditionBlock(edit) {
    var id_comment = $(this).attr("id_comment");
    //check if therre is other reply opened
    var $prev = $(".co_new_comment.reply[id_comment=" + id_comment + "]");
    if ($prev.length)
    {
        //remove the previous opened
        $prev.remove();
    }
    edit = edit ? 1 : 0;
    var $toEL = $(this).closest(".g_comment");
    $.ajax({
        url: "includes/g_comments_widget/php/g_ajax_edit_block.php?edit=" + edit + "&id=" + id_comment,
        success: function(html) {
            $toEL.closest(".g_comment").after(html);

            var $ta = $("textarea[id_comment=" + id_comment + "]");
            $ta.focus(focusTextAreaComment);

            // save the reply
            var uploading = false;
            $("#g_comment_button_" + id_comment).click(function(event) {
                event.preventDefault();
                if (uploading) {
                    return;
                }

                if (!validateComment($ta))
                    return;

                uploading = true;

                $(this).hide();
                $(this).siblings(".g_cancel_button").hide()
                $(this).siblings(".g_sending_note").show();

                $.ajax({
                    url: 'includes/g_comments_widget/php/g_perform_comment.php',
                    type: 'POST',
                    data: {
                        'comment': $ta.val(),
                        'id_product': g_id_product,
                        'reply_of': id_comment,
                        'id_comment': !edit ? "" : id_comment
                    },
                    dataType: 'json',
                    success: function(json) {

                        //refresh comment page
                        g_list_refresh(json.id_comment);
                        uploading = false;
                        $ta.val("");

                    }
                });
            });

            $("#g_cancel_button_" + id_comment).click(function() {
                $(this).closest(".co_new_comment").remove();
            });

            if (edit) {
                $.ajax({
                    url: 'includes/g_comments_widget/php/g_comment_body_json.php?id_comment=' + id_comment,
                    type: 'GET',
                    dataType: 'json',
                    success: function(body) {
                        $ta.removeAttr("first");
                        $ta.val(body);
                    }
                });
            }
        }
    });

}

function listHooks() {
    $('.g_list_see_more').click(function() {
        $(this).remove();
        $.ajax({
            url: 'includes/g_comments_widget/php/g_load_block.php',
            type: 'GET',
            data: 'id_product=' + g_id_product + '&block=' + g_n_open_blocks,
            success: function(html) {
                $("#list_comments").append(html);
                listHooks();
                g_n_open_blocks++;
            }
        });
    });

    $(".comment_body .see_more").click(function() {
        $(this).parent().append($(this).find(".text").html());
        $(this).remove();
    });

    $(".comment_reply").click(function() {
        showCommentEditionBlock.apply(this, [false]);
    });

    $("a.fancy_comment").fancybox({
        'transitionIn': 'elastic',
        'transitionOut': 'elastic',
        'speedIn': 600,
        'speedOut': 200,
        'overlayShow': false,
        onComplete: function() {

            $(".g_comment_button.grey.cancel").click(function() {
                $.fancybox.close();
            });

            $(".g_comment_button.grey.accept").click(function() {
                $.ajax({
                    url: "includes/g_comments_widget/php/g_delete_comment.php?id_comment=" + $(this).attr("id_comment"),
                    type: "GET",
                    success: function() {
                        $.fancybox.close();
                        g_list_refresh();
                    }
                });
            });
        }
    });

    $(".inappropriate").click(function() {
        var $el = $(this);
        var id_comment = $(this).attr("id_comment");
        $.ajax({
            url: 'includes/g_comments_widget/php/g_mark_inappropriate.php?id_comment=' + id_comment,
            type: 'GET',
            success: function() {
                g_list_refresh();

            }
        })
    });

    $(".edit_ctrl").click(function() {
        showCommentEditionBlock.apply(this, [true]);
    });

    hookWorldTimeFormat();
}

function g_list_refresh(id_comment) {
    // refresh counters
    $("#n_comments").load("includes/g_comments_widget/php/g_refresh_n_comments.php?id_product=" + g_id_product);

    // refresh list
    $("#list_comments").load('includes/g_comments_widget/php/g_refresh_comments.php?id_product=' + g_id_product + "&n_open_blocks=" + g_n_open_blocks, function() {
        listHooks();
        if (id_comment) {
            goToComment(id_comment);
        }
    });
}

function goToComment(id_comment) {
    var $el = $("div[scroll_id_comment='" + id_comment + "']");
    $('html, body').animate({
        scrollTop: $el.offset().top
    }, 1000);
    $el.addClass("enhaced_comment");
    setTimeout(function() {
        $el.removeClass("enhaced_comment");
    }, 3000);
}