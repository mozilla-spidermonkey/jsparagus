$(document).ready(function() {
	var activeUrl = location.pathname.split("/")[1];
	var saveUse = $('.cmnMdGloNavi li div.typeTxt4');
	var top = $('.cmnMdGloNavi li div.typeTxt1');
	
	$('.cmnMdGloNavi li').each(function() {
		
		var navLink = $("a",this).attr("href").split("/")[3];

		if( navLink == activeUrl ) {
			$(this).addClass('current');
			$("a", this).click(function(e){
				e.preventDefault();
			});
		} else if( activeUrl == 'alliance' ){
			saveUse.parent('li').addClass('current');
			saveUse.children('a').click(function(e){
				e.preventDefault();
			});
		} else if( activeUrl == '' ){
			top.parent('li').addClass('current');
			top.children('a').click(function(e){
				e.preventDefault();
			});
		}
    });
});
