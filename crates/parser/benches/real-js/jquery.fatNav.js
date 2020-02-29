(function($, window, document) {
    
    var pluginName = 'fatNav',
    defaults = {};
    
    function Plugin(options) {
        this.settings = $.extend({}, defaults, options);
        this._defaults = defaults;
        this._name = pluginName;
        this.init();
    }
    function mobileLogin(){
        var userAgentInfo = navigator.userAgent;
        var Agents = ["Android", "iPhone",
            "SymbianOS", "Windows Phone"
        ];
        var flag = false;
        for (var v = 0; v < Agents.length; v++) {
            if (userAgentInfo.indexOf(Agents[v]) > 0) {
                flag = true;
                break;
            }
        }
        if (flag) {
            window.location.href = '/gitchat/mobileLogin';
            return true;
        } else {
            return false;
        }
    }
    $.extend(Plugin.prototype, {
        
        init: function() {

            var self = this;
            var $nav = this.$nav = $('.fat-nav');
            var $hamburger = this.$hamburger = $('<a style="display: none;" id="hamburger" href="javascript:void(0)" class="hamburger"><div class="hamburger__icon"></div></a>');
            
            this._bodyOverflow = $('body').css('overflow');

            // Hack to prevent mobile safari scrolling the whole body when nav is open
            if (navigator.userAgent.match(/(iPad|iPhone|iPod)/g)) {
                
                $nav.children().css({
                    'height': '110%',
                    'transform': 'translateY(-5%)'
                });
                
            }
            
            $('body').append($hamburger);

            $('#loginOKBtn').on('click', function(e) {
                $('#hamburger').hide();
                self.toggleNav();
            });

            $('#loginCancelBtn').on('click', function(e) {
                $('#hamburger').hide();
                self.toggleNav();
            });

            $('#hamburger').on('click', function(e) {
                $('#hamburger').hide();
                self.toggleNav();
            });

            $('#loginBtn').on('click', function(e) {
                if(mobileLogin()){
                    return;
                }
                if((/MicroMessenger/i).test(window.navigator.userAgent)){
                    var state = encodeURI(window.location.href);
                    var redirect_url = encodeURI('https://gitbook.cn/weixin/m/callback/login');
                    var redirect_href = "https://open.weixin.qq.com/connect/oauth2/authorize?appid=wx7ba47eadabd9ddde&redirect_uri="+redirect_url+"&response_type=code&scope=snsapi_userinfo&state="+state+"#wechat_redirect";
                    window.location.href= redirect_href
                } else {
                    $('#hamburger').show();
                    $('#loginInputDiv').show();
                    $('#loginSubmittedDiv').hide();
                    $('#mailAddressInvalidDiv').hide();
                    self.toggleNav();
                }
            });
            $('#newChatBtn').on('click', function(e) {
                $('#hamburger').show();
                $('#loginInputDiv').show();
                $('#loginSubmittedDiv').hide();
                $('#mailAddressInvalidDiv').hide();
                self.toggleNav();
            });
            $('#newChatBtn2').on('click', function(e) {
                $('#hamburger').show();
                $('#loginInputDiv').show();
                $('#loginSubmittedDiv').hide();
                $('#mailAddressInvalidDiv').hide();
                self.toggleNav();
            });
            $('#newChatBtn3').on('click', function(e) {
                $('#hamburger').show();
                $('#loginInputDiv').show();
                $('#loginSubmittedDiv').hide();
                $('#mailAddressInvalidDiv').hide();
                self.toggleNav();
            });
            $('#writeBtn').on('click', function(e) {
                $('#hamburger').show();
                $('#loginInputDiv').show();
                $('#loginSubmittedDiv').hide();
                $('#mailAddressInvalidDiv').hide();
                self.toggleNav();
            });
            $('#loginBtn2').on('click', function (e) {
                if ((/MicroMessenger/i).test(window.navigator.userAgent)) {
                    var state = encodeURI(window.location.href);
                    var redirect_url = encodeURI('http://www.gitbook.cn/weixin/m/callback/login');
                    window.location.href = "https://open.weixin.qq.com/connect/oauth2/authorize?appid=wx7ba47eadabd9ddde&redirect_uri=" + redirect_url + "&response_type=code&scope=snsapi_userinfo&state=" + state + "#wechat_redirect";
                } else {
                    $('#hamburger').show();
                    $('#loginInputDiv').show();
                    $('#loginSubmittedDiv').hide();
                    $('#mailAddressInvalidDiv').hide();
                    self.toggleNav();
                }
            });

            $('#scanQRLoginBtn').on('click', function (e) {
                if ((/MicroMessenger/i).test(window.navigator.userAgent)) {
                    var state = encodeURI(window.location.href);
                    var redirect_url = encodeURI('http://www.gitbook.cn/weixin/m/callback/login');
                    window.location.href = "https://open.weixin.qq.com/connect/oauth2/authorize?appid=wx7ba47eadabd9ddde&redirect_uri=" + redirect_url + "&response_type=code&scope=snsapi_userinfo&state=" + state + "#wechat_redirect";
                } else {
                    $('#hamburger').show();
                    $('#loginInputDiv').show();
                    $('#loginSubmittedDiv').hide();
                    $('#mailAddressInvalidDiv').hide();
                    self.toggleNav();
                }
            });

            $('#commentLoginBtn').on('click', function (e) {
                if (mobileLogin()) {
                    return;
                }
                if ((/MicroMessenger/i).test(window.navigator.userAgent)) {
                    var state = encodeURI(window.location.href);
                    var redirect_url = encodeURI('http://www.gitbook.cn/weixin/m/callback/login');
                    window.location.href = "https://open.weixin.qq.com/connect/oauth2/authorize?appid=wx7ba47eadabd9ddde&redirect_uri=" + redirect_url + "&response_type=code&scope=snsapi_userinfo&state=" + state + "#wechat_redirect";
                } else {
                    $('#hamburger').show();
                    $('#loginInputDiv').show();
                    $('#loginSubmittedDiv').hide();
                    $('#mailAddressInvalidDiv').hide();
                    self.toggleNav();
                }
            });

            $('#writeBtn2').on('click', function(e) {
                $('#hamburger').show();
                $('#loginInputDiv').show();
                $('#loginSubmittedDiv').hide();
                $('#mailAddressInvalidDiv').hide();
                self.toggleNav();
            });

            $('#initActivity2').on('click', function(e) {
                $('#hamburger').show();
                $('#loginInputDiv').show();
                $('#loginSubmittedDiv').hide();
                $('#mailAddressInvalidDiv').hide();
                self.toggleNav();
            });
            $('#initActivity').on('click', function(e) {
                $('#hamburger').show();
                $('#loginInputDiv').show();
                $('#loginSubmittedDiv').hide();
                $('#mailAddressInvalidDiv').hide();
                self.toggleNav();
            });
            $('#tryRead2Login,#tryRead2Login_cata').on('click', function(e) {
                if (mobileLogin()) {
                    return;
                }
                $('#hamburger').show();
                $('#loginInputDiv').show();
                $('#loginSubmittedDiv').hide();
                $('#mailAddressInvalidDiv').hide();
                self.toggleNav();
            });
            $('#tryBuy2Login,#tryBuy2Login_cata').on('click', function(e) {
                if (mobileLogin()) {
                    return;
                }
                $('#hamburger').show();
                $('#loginInputDiv').show();
                $('#loginSubmittedDiv').hide();
                $('#mailAddressInvalidDiv').hide();
                self.toggleNav();
            });
            $('#tryRead2Login2,#tryRead2Login2_cata').on('click', function(e) {
                if (mobileLogin()) {
                    return;
                }
                $('#hamburger').show();
                $('#loginInputDiv').show();
                $('#loginSubmittedDiv').hide();
                $('#mailAddressInvalidDiv').hide();
                self.toggleNav();
            });
            
        },
        
        toggleNav: function() {

            var self = this;

            this.$nav.fadeToggle(400);
			
			self.toggleBodyOverflow();

            $().add(this.$hamburger).add(this.$nav).toggleClass('active');

        },
        
        toggleBodyOverflow: function() {
            
            var self = this;
			
			var $body = $('body');
            
            $body.toggleClass('no-scroll');

			var isNavOpen = $body.hasClass('no-scroll');
			
//			$body.width($body.width());
            $body.css('overflow', isNavOpen ? 'hidden' : self._bodyOverflow);
			
        }
        
    });
    
    if (typeof $[pluginName] === 'undefined') {
        
        $[pluginName] = function(options) {
            return new Plugin(this, options);
        };
        
    }

}(jQuery, window, document));
var fatNav = null;
(function() {

    fatNav = $.fatNav();

}());