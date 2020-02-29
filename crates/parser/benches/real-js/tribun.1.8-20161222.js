var $ = jQuery.noConflict();
$(document).ready(function() { 
	$(document).on('mouseover','.art-list',function(){
		var t = $(this).find(".sharecot");
		var ctop = $(this).find(".foot").position().top;
		var cwleft = $(this).find(".foot").position().left;
		var cwidth = $(this).find(".foot").width();
		var wpfoot = $(this).find(".foot").parent().width();
		if (t.is(':empty')) {
			var url = t.attr("data-href");
			var title = t.attr("data-title");
			var via = t.attr("data-via");
			if(via == "" || via == null ) via = "tribunnews";
			html = '<div class="fl mr10 pos_rel sncount"><a href="javascript:void(0)" onclick="window.open(\'https://www.facebook.com/sharer/sharer.php?u='+url+'&t='+title+'\',\'popUpWindow\',\'height=350,width=550,resizable=no,scrollbars=no,toolbar=no,menubar=no,location=no,directories=no,status=no\')"><span class="fbshare"><i class="fa fa-facebook-official"></i>Share</span></a></div><div class="fl pos_rel sncount"><a href="javascript:void(0)" onclick="window.open(\'https://twitter.com/intent/tweet?original_referer='+url+'&text='+title+'&tw_p=tweetbutton&url='+url+'&via='+via+'\',\'popUpWindow\',\'height=350,width=550,resizable=no,scrollbars=no,toolbar=no,menubar=no,location=no,directories=no,status=no\')"><span class="twshare"><i class="fa fa-twitter"></i>Tweet</span></a></div>';
			t.html(html);
		} else {
			t.show();
		}
		t.css("top",ctop);
		if(wpfoot > 300) {
			$(this).find(".foot").parent().children().show();
			t.css("left",cwidth + cwleft  + 15);
		} else {
			$(this).find(".foot").parent().children().hide();
			$(this).find(".foot").parent().css({display:'block',height:'18px'});
			t.css("height", $(this).find(".foot").parent().height());
			t.css("left", 0);
		}
	}).on('mouseout','.art-list',function(){
		$(this).find(".foot").parent().children().show();
		$(this).find(".sharecot").hide();
	});


	var ontop = function(){
		var st = $(window).scrollTop();		
		if(st < $(window).height())	
			$(".ontop").hide();
		else
			$(".ontop").show();
	}
	$(window).scroll(ontop);
	$(".ontop").click(function(){
      $('html, body').animate({scrollTop:0}, 'normal');
        return false;
	});

	$("#qsearch").submit(function(){
		if($("#searchbox").val() == $("#searchbox").attr("placeholder")) {
			$("#searchbox").val("");
			$("#searchbox").focus();
			return false;		
		}
	})
	$(".plh").focus(function(){
		if($(this).val() == $(this).attr("placeholder")) {
			$(this).val("");
		}

	});
	$(".plh").keypress(function(){
		if($(this).val() == $(this).attr("placeholder")) {
			$(this).val("");
		}
	
	});


	$(".dialogbox").fancybox({
		maxWidth	: 800,
		maxHeight	: 600,
		    helpers:  {
		        title : {
		            type : 'inside'
		        },
		        overlay : {
		            showEarly : false
		        }
		    },
		padding:10

	});

	$('#fpbtn').click(function() {
		$('#fpresultpar').show();
		$('#fpresultpar').html("mengirim...");
		$.getJSON("http://www.tribunnews.com/auth/forgot_password_json?callback=?", $("#fpform").serialize(), function(data) {
			$("#fpresultpar").html(data.status);
		});
		return false;
	});
	
	$('#logform').submit(function() {
		$('#logresultpar').show().attr("class","message").html("mengirim...");
		$.getJSON("http://www.tribunnews.com/auth/login_json?callback=?", $("#logform").serialize(), function(data) {
			if (data.status == 'login') {
				$.cookie("auth",null,{	path: '/',domain: '.tribunnews.com'});
				location.reload();
			}
			else
			{
				$("#logresultpar").html(data.status).attr("class","error");
				if(data.status == "attempts_exceeded") {
					$("#captchalogin").show();
					$("#captchaloginimg").html(data.captcha_html);				
					$("#logresultpar").html("Anda sudah mencoba login beberapa kali, masukan kode sekuriti untuk keamanan login");
				}
				else if(data.status == "Kode Angka Salah") {
					$("#captchalogin").show();
					$("#captchaloginimg").html(data.captcha_html);				
					$("#logresultpar").html("Anda sudah mencoba login beberapa kali, masukan kode sekuriti untuk keamanan login");
				}
			}
			
		});	
		return false;
	});

	$('#rform').submit(function() {
		$('#regresultpar').show().attr("class","message").html("mengirim data...");
		$.getJSON("http://www.tribunnews.com/auth/register_json?callback=?", $("#rform").serialize(), function(data) {
			if (data.connect_sl == 'success') {
				$.cookie("auth",null,{	path: '/',domain: '.tribunnews.com'});
				location.reload();
			}
			else
			{
				if(data.error) 
				{
					$("#regresultpar").html(data.error);
					$("#regresultpar").attr("class","error");
				}
				else 
				{
					$("#regresultpar").html(data.status);
					$("#regresultpar").attr("class","error");
					if(data.register == "success") {
						$("#rform").hide();
						$("#regresultpar").attr("class","message");
					}
				}

				$("#captcha").html(data.captcha_html);
				$("#captchadiv input").val("");
			}
			
		});	
		return false;
	});
});
	
	function ajaxload(url,result,loaded,cache)
	{
		var url = url.replace('/#','/');
		var url = url.replace('#','/');
		if (!re) var re = false;
		if (loaded) $(loaded).show('slow');
		$.ajax({
			url : url,
			cache: cache,
			dataType: "html",
			success: function(data) {
				$(result).html(data);
				if (loaded) $(loaded).hide('slow');
			}
		});
	}

	function ajaxload2(url,result)
	{
		$(result).show();
		$(result).load(url);
		return false;
	}

	function fbox(url,tit,tipe)
	{
		if(tipe == "") tipe = "ajax";
		$.fancybox.open({
			href:url,
			title:tit,
			padding:10,
			fitToView:false,
autoCenter:false,
		    helpers:  {
		        title : {
		            type : 'inside'
		        },
		        overlay : {
		            showEarly : false
		        }
		    },
			type:tipe,	
			afterClose:function() { $.cookie("fconnect","false",{	path: '/',domain: '.tribunnews.com'});$.cookie("auth",null,{	path: '/',domain: '.tribunnews.com'});}
		});
	};

	function fbox_close()
	{
		$.fancybox.close();
	};

	function login(type,title)
	{
		$('#logresultpar').html("Loading").hide();
		if(type == 'login'){ 
			$(".fotofb").hide();
			$("#sl_btn_log").show();
			$("#logindesc").html('Belum punya akun? <br />silahkan <a class="fbo" onclick="register(\'register\',\'Daftar Baru\')" href="javascript:">Mendaftar</a>');
		} else {
			$(".fotofb").show();
			$("#sl_btn_log").hide();
			$("#logindesc").html('<a class="" onclick="register(\''+type+'\',\''+title+'\')" href="javascript:">atau daftar baru</a>');
		}
		fbox('#logresult',title,'inline');

	};
	function register(type,title)
	{
		$.getJSON("http://www.tribunnews.com/auth/register_json?callback=?", $("#rform").serialize(), function(data) {
			$("#captcha").html(data.captcha_html);
		});
		$('#regresultpar').html("Loading").hide();
		if(type == 'register'){
			$('#captchadiv').show();
			$('#username').val("");	
		    $('#name').val(""); 
			$('#email').val("");
			$('.sl_type').val("");
			$('.sl_userid').val("");
			$(".fotofb").hide();
			$("#sl_btn_reg").show();
			$("#registerdesc").html('Sudah punya akun, silahkan <a class="fbo" onclick="login(\'login\',\'Masuk\')"  href="javascript:" >Masuk</a>');
		} else {
			$('#captchadiv').hide();
			$(".fotofb").show();
			$("#sl_btn_reg").hide();
			$("#registerdesc").html('<a class="" onclick="login(\''+type+'\',\''+title+'\')" href="javascript:">atau hubungkan dengan akun saya</a>');
		}
		fbox('#regresult',title,'inline');
	}

jQuery.cookie=function(name,value,options){if(typeof value!='undefined'){options=options||{};if(value===null){value='';options=$.extend({},options);options.expires=-1;}var expires='';if(options.expires&&(typeof options.expires=='number'||options.expires.toUTCString)){var date;if(typeof options.expires=='number'){date=new Date();date.setTime(date.getTime()+(options.expires*24*60*60*1000));}else{date=options.expires;}expires='; expires='+date.toUTCString();}var path=options.path?'; path='+(options.path):'';var domain=options.domain?'; domain='+(options.domain):'';var secure=options.secure?'; secure':'';document.cookie=[name,'=',encodeURIComponent(value),expires,path,domain,secure].join('');}else{var cookieValue=null;if(document.cookie&&document.cookie!=''){var cookies=document.cookie.split(';');for(var i=0;i<cookies.length;i++){var cookie=jQuery.trim(cookies[i]);if(cookie.substring(0,name.length+1)==(name+'=')){cookieValue=decodeURIComponent(cookie.substring(name.length+1));break;}}}return cookieValue;}};