$(document).ready(function () {
    $("[data-js-dialog-hide]").click(function () {       
        stopYouTubevideo();
        stopHTML5video();
    });

    $(document).keydown(function (e) {
        var key = Cortex.Utilities.getKeyCode(e);

        if (key == $.ui.keyCode.ESCAPE) {
            stopYouTubevideo();
            stopHTML5video();
        }
    });

    $("[data-js-dialog-hide]").keydown(function (e) {
        var key = Cortex.Utilities.getKeyCode(e)

        if (key == $.ui.keyCode.ENTER || key == $.ui.keyCode.SPACE || key == $.ui.keyCode.ESCAPE || key == undefined) {
            stopYouTubevideo();
            stopHTML5video();
        }
    });

    $('a[data-js-dialog-show][href="#"]').on("click", function (e) {
        e.preventDefault();
    });
});



function stopYouTubevideo() {
    $(".youtube:visible").each(function () {      
        this.contentWindow.postMessage('{"event":"command","func":"' + 'stopVideo' + '","args":""}', '*')
    });
}

function stopHTML5video() {
    $("[role='dialog']").each(function () {
        if ($(this).find("video").length > 0) {
            $(this).find("video")[0].pause();
        }        
    });
}