$(function () {
    var n = $(location).attr("href"),
        t = $(document).attr("title"),
        i = $("#cnxSocialWidget-root-subject").val(),
        r = $("#cnxSocialWidget-root-body").val();
    $(".cnxSocialWidget-root-facebook").attr("href", "http://www.facebook.com/sharer.php?u=" + encodeURIComponent(n)), $(".cnxSocialWidget-root-LinkedIn").attr("href", "http://www.linkedin.com/shareArticle?mini=true&url=" + encodeURIComponent(n) + "&title=" + encodeURIComponent(t)), $(".cnxSocialWidget-root-twitter").attr("href", "http://twitter.com/share?text=" + encodeURIComponent(n) + "&url=" + encodeURIComponent(t)), $(".cnxSocialWidget-root-email").attr("href", "mailto:?subject=" + i + "&body=" + r)
});

$(document).ready(function () {

    $('#headerArea .m-skip-to-main').on('click', function (e) {
        var anchor = $(e.currentTarget).attr('href');
        setTimeout(function () {
            $(anchor).find('*:first').focus();
        });
    });    

});