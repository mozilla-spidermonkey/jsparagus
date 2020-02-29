// Set Global YAHOO
if (typeof YAHOO === 'undefined') {
    /**
     * @namespace
     */
    YAHOO = {};
}
if (typeof YAHOO.JP === 'undefined') {
    /**
     * @namespace
     */
    YAHOO.JP = {};
}
if (typeof YAHOO.JP.jnb === 'undefined') {
    /**
     * @namespace
     */
    YAHOO.JP.jnb = {};
}




/**
*  モーダルウィンドウ
*
* @class modalWindow
* @namespace YAHOO.JP.jnb
* @static
*/
YAHOO.JP.jnb.modalWindow  = (function(){

    return{

        /**
        *　初期化・イベント設定
        *
        * @method init
        * @private
        */
        _CUSTOM : {},
        init : function (setting) {

          if(!setting || !setting.modalBtn){
            var setting = {
            "modalBtn":".js-modalBtn", //モーダルのトリガーボタン
            "modalMask":".js-modalMask", //モーダルの下の半透明のマスク
            "modalWindow":".js-modalWindow", //モーダルウィンドウ
            "closeBtn":".js-modalCloseBtn"//モーダルを閉じるボタン
            }
          }

        var self = this; //thisはYAHOO.JP.jnbを指す
        self._CUSTOM[setting.modalBtn] = setting ;



            $(self._CUSTOM[setting.modalBtn].modalBtn).on("click", function(e){

                //以下URL作成に移行
                // //Yahoo! JAPAN IDの登録情報を利用しないで口座開設ボタンの場合
                // //autofillのon/off
                // //ネットキャッシュ、外貨ではautofillは常にon
                // if($(this).hasClass("js-YidOnly")){
                //     $("#JNBlink").attr("href" , $("#JNBlink").attr("href").replace("autofill=on","autofill=off").replace("autofill%3Don","autofill%3Doff"))
                // }else{
                //     $("#JNBlink").attr("href" , $("#JNBlink").attr("href").replace("autofill=off","autofill=on").replace("autofill%3Doff","autofill%3Don"))
                // }
                self.view(setting);
            });
            $(self._CUSTOM[setting.modalBtn].closeBtn).on("click", function(){
                self.hide(setting);
            });

        }, //init


        /**
        *　モーダルを表示
        *
        * @method scrollAnchor
        * @private
        */
       view : function (setting) {
           var self = this;

            self.followScroll(setting);
            $(self._CUSTOM[setting.modalBtn].modalMask).removeClass("is-hide");
            $(self._CUSTOM[setting.modalBtn].modalWindow).removeClass("is-hide");


       }, //view
       /**
        *　モーダルを隠す
        *
        * @method scrollAnchor
        * @private
        */
       hide : function (setting) {
          var self = this;
          if(!self._CUSTOM[setting.modalBtn]){
           self._CUSTOM[setting.modalBtn] = setting ;
           }
            $(self._CUSTOM[setting.modalBtn].modalMask).addClass("is-hide");
            $(self._CUSTOM[setting.modalBtn].modalWindow).addClass("is-hide").attr("aria-hidden","true");
            // $(self._CUSTOM[setting.modalBtn].modalBtn).focus();
            $("body").off("keydown.dialog");
            self.resumeScroll();

            //フォーカスをモーダル開くボタンに戻す
            // $(self._CUSTOM[setting.modalBtn].modalBtn).focus();

       }, //scrollAnchor
       /**
        *　画面のスクロールを止める
        *
        * @method stopScroll
        * @private
        */
        stopScroll : function() {
            var scrollTop  = 0,
            scrollLeft = 0;

            scrollTop  = $(window).scrollTop();

                // $('html,body').css({
                //     'position': 'fixed',
                //     'width':'100%',
                //     'top': (-scrollTop) + 'px'
                // });
            //androidでうまく動かないためsettimeoutで少し遅らせている
            setTimeout(function(){$('html,body').css({'position': 'fixed','width':'100%','top': '0px'})},0)

        },
       /**
        *　画面のスクロールを止めるのを解除する
        *
        * @method resumeScroll
        * @private
        */
        resumeScroll : function() {
                 $('html,body').css({
                    'position': 'static'
                });

        },
        /**
        *　スクロールに合わせて真ん中に来るようにする（PC対応）
        *　settingの値によって表示場所を変える
        *
        * @method followScroll
        * @private
        */
        followScroll : function(setting) {
            // console.log(scroll)
            var self = this;
            var scrollTop  = 0,
            scrollLeft = 0,
            // modalHeight = scroll,
            windowHeight = window.innerHeight,
            scrollTop  = $(window).scrollTop();
            // if((parseInt(windowHeight)-parseInt(modalHeight))<0){
                var modalpPositon = parseInt(scrollTop) +parseInt(150);
            // }else{
            //     var modalpPositon = scrollTop +(parseInt(windowHeight)-parseInt(modalHeight))/2
            // }

            $(self._CUSTOM[setting.modalBtn].modalWindow).css({
                //'position': 'fixed',
                'top': (modalpPositon) + 'px'
            });

        }
    }

})();




/**
*  URL作成
*
* @class modalWindow
* @namespace YAHOO.JP.jnb
* @static
*/
YAHOO.JP.jnb.urlMaking  = (function(){

    return{

        /**
        *　初期化・イベント設定
        *
        * @method init
        * @private
        */
        _CUSTOM : {},
        init : function (setting) {

            if(!setting){
                var setting = {
                    "login":"false",
                    "page_type":"normal",
                    "url" : 'https://bank.yahoo.co.jp/autofill/apply.html?pid=1&autofill=on&lptype=a&crumb=3fd0b0c4f7a6cc2b9f7b16cfcd421455&t=1552987496&lpid=2015000060&aatc=ot1410yj01a&param1=&param2=&account_type=normal',
                    "url2": 'https://login.japannetbank.co.jp/wctx/AF.do?SikibetuId=2018000037&param1='
                }
            }
            var self = this; //thisはYAHOO.JP.jnbを指す
            var jsMainBtn = $("#JNBlink");//ログイン


            $(".js-modalBtn").on("click",function(e){
                if(setting.page_type == "normal" || !setting.page_type ){
                    //デフォルトの時
                    jsMainBtn.attr('href', self.normal(setting , $(this)));
                }else if(setting.page_type == "cashing"){
                    //キャッシングの時
                    if($(this).hasClass("js-cashingOnly")){
                        //右ボタン
                        jsMainBtn.attr('href', setting.url2);
                    }else{
                        //左ボタン
                        jsMainBtn.attr('href', self.loginCheck(setting.url,setting.login));
                    }

                }
            })


        },


        //通常時のURL作成
        normal : function(setting,link){
            var self = this; //thisはYAHOO.JP.jnbを指す
            var url = setting.url;

            if(link.hasClass("js-YidOnly")){
                //Yahoo! JAPAN IDの登録情報を利用しないで口座開設（こちらは口座開設のみです。）リンク
                //オートフィルOFFに 
                url = url.replace("autofill=on" , "autofill=off");
            }else{
                //通常ボタン
                //オートフィルをonに
                url = url.replace("autofill=off" , "autofill=on");
                //ネットキャッシュチェックの有無
                if($(".netcash-check").length && $(".netcash-check").is(':checked')){
                    // console.log("チェックあり")
                    //ネットキャッシュ化
                    url = url.replace("account_type=normal" , "account_type=netcash");
                }
                url =  this.loginCheck(url,setting.login);
            }
            return url;

        },


        //ログインチェック
        loginCheck : function(url,login){
            if(login =="false"){
                url = "https://login.yahoo.co.jp/config/login?.lg=jp&.intl=jp&.src=payment&.done="+encodeURIComponent(url)
            }
            return url;
        }
    }

})();







/**
*  URL作成
*
* @class modalWindow
* @namespace YAHOO.JP.jnb
* @static
*/
YAHOO.JP.jnb.netcashRapid  = (function(){

    return{

        /**
        *　初期化・イベント設定
        *
        * @method init
        * @private
        */
        _CUSTOM : {},
        init : function (setting) {



            $(".netcash-check").change(function() {
            var thisRapidP = $(this).attr("data-rapid_p");

            var thisLinkData = [
              {
                sec: "registration",
                _links: [
                  {
                    slk: "chkmrk",
                    _p: thisRapidP
                  }
                ]
              }
            ];

            if ($(this).is(':checked')) {
                $(".netcash-check").prop("checked",true);
                // チェックが入ったときだけクリックログ送信

                // クリックログを可視化するためにまずLinkViewsを送信
                ins.beaconLinkViews(thisLinkData, false);

                // クリックログの送信
                ins.beaconClick("registration","chkmrk",thisRapidP);

            } else {
            $(".netcash-check").prop("checked",false);
            }


            });

        }
    }

})();