$(function(){
    try {
        // イベントマップ
        var EVENT_MAP = {
            top : {
                url_reg : new RegExp('^https?://[^\/]*/$')
                ,trgt : [
                    {
                      elm : '[id^=stampPattern]',
                      efuc : function(e) {setData('stamprally_moduleclick', 'event294,event394', '', e)}
                    }
                ]
            }
            ,shisetsu_list : {
                url_reg : new RegExp('^.*/dhotel/search/.*')
                ,trgt : [
                    {
                      elm : '[id^=stampPattern]',
                      efuc : function(e) {setData('stamprally_moduleclick', 'event294,event394', '', e)}
                    }
                ]
            }
            ,shisetsu_info : {
                url_reg : new RegExp('^.*/dhotel/shisetsu/HT\\d{8}/information.*')
                ,trgt    : [
                    { elm  : '#favorite_button'
                        ,efuc : function(e){setData('regist_favorite', 'scAdd,event95', '', e)} }
                    ,{ elm  : '#trvcp'
                        ,efuc : function(e){setData('see_5percent_over_plan', 'event16,event116', '', e)} }
                ]
            }
            ,shisetsu_photo : {
                url_reg : new RegExp('^.*/dhotel/shisetsu/HT\\d{8}/photos.*')
                ,trgt    : [
                    { elm  : '#favorite_button'
                        ,efuc : function(e){setData('regist_favorite', 'scAdd,event95', '', e)} }
                    ,{ elm  : '#trvcp'
                        ,efuc : function(e){setData('see_5percent_over_plan', 'event16,event116', '', e)} }
                ]
            }
            ,shisetsu_review : {
                url_reg : new RegExp('^.*/dhotel/shisetsu/HT\\d{8}/review.*')
                ,trgt    : [
                    { elm  : '#favorite_button'
                        ,efuc : function(e){setData('regist_favorite', 'scAdd,event95', '', e)} }
                    ,{ elm  : '#trvcp'
                        ,efuc : function(e){setData('see_5percent_over_plan', 'event16,event116', '', e)} }
                ]
            }
            ,plan_list : {
                url_reg : new RegExp('^.*/dhotel/shisetsu/HT\\d{8}/?[\?].*')
                ,trgt    : [
                    { elm  : '#favorite_button'
                        ,efuc : function(e){setData('regist_favorite', 'scAdd,event95', '', e)} }
                    ,{ elm  : '#htlapbtn a'
                        ,efuc : function(e){setData('see_other_plan', 'event15,event115', '', e)} }
                    ,{
                        elm : '[id^=stampPattern]',
                        efuc : function(e) {setData('stamprally_moduleclick', 'event294,event394', '', e)}
                    }
                ]
            }
            ,plan_detail : {
                url_reg : new RegExp('^.*/dhotel/shisetsu/HT\\d{8}/(JTB|IKYU|BR|YAHOO)/(?!.*ci=).+$')
                ,trgt    : [
                    { elm  : '#favorite_button'
                        ,efuc : function(e){setData('regist_favorite', 'scAdd,event95', '', e)} }
                    ,{ elm  : '.jumpDirect .directLink'
                        ,efuc : function(e){setData('click_vacant_status', 'event10,event110', '', e);} }
                    ,{ elm  : '#an_pbtn1 a, #an_pbtn2 a'
                        ,efuc : function(e){setData('see_other_plan', 'event15,event115', '', e)} }
                ]
            }
            ,plan_detail_date_y : {
                url_reg : new RegExp('^.*/dhotel/shisetsu/HT\\d{8}/YAHOO/.*ci=.*')
                ,trgt    : [
                    { elm  : '#favorite_button'
                        ,efuc : function(e){setData('regist_favorite', 'scAdd,event95', '', e)} }
                    ,{ elm  : '#directLinkCheckd, #floatingRbtnGet'
                        ,efuc : setCartButton}
                    ,{ elm  : '#an_pbtn1 a, #an_pbtn2 a'
                        ,efuc : function(e){setData('see_other_plan', 'event15,event115', '', e)} }
                ]
            }
            ,plan_detail_date_cp : {
                url_reg : new RegExp('^.*/dhotel/shisetsu/HT\\d{8}/(JTB|IKYU|BR|OS)/.*ci=.*')
                ,trgt    : [
                    { elm  : '#favorite_button'
                        ,efuc : function(e){setData('regist_favorite', 'scAdd,event95', '', e)} }
                    ,{ elm  : '#directSolidGet, #onlineSolidGet, #floatingCbtnGet, #floatingObtnGet, #directSolidPost, #directLinkCheckd, #plandescRbtnGet span, #floatingRbtnGet, #plandescGbtnGet, #directLinkCheckdGuest, #floatingGbtnGet' 
                        ,efuc : setCartButton}
                    ,{ elm  : '#an_pbtn1 a, #an_pbtn2 a'
                        ,efuc : function(e){setData('see_other_plan', 'event15,event115', '', e)} }
                ]
            }
            ,favorite : {
                url_reg : new RegExp('^https://my.travel.yahoo.co.jp/favorite/dom/.*')
                ,trgt    : [
                    { elm  : '.mdMyFavHotelList .delete a'
                        ,efuc : setRemoveFav }
                ]
            }
            ,review_entry : {
                url_reg : new RegExp('^https://my.travel.yahoo.co.jp/review/entry/')
                ,trgt    : [
                    { elm  : '#value_total'
                        ,efuc : function(e){setData('review_total', 'event82,event182', '', e, 'eVar5')} }
                    ,{ elm  : '#comment_id'
                        ,efuc : function(e){setData('review_text', 'event83,event183', '', e, 'eVar5')} }
                    ,{ elm  : '#decide_id'
                        ,efuc : function(e){setData('review_confirm', 'event85,event185', '', e, 'eVar5')} }
                    ,{ elm  : '.yjmthproplogoarea a'
                        ,efuc : function(e){setData('review_top link', 'event88,event188', '', e, 'eVar5')} }
                ]
            }
            ,review_completion : {
                url_reg : new RegExp('^https://my.travel.yahoo.co.jp/review/completion/')
                ,trgt    : [
                    { elm  : '.yjmthproplogoarea a'
                        ,efuc : function(e){setData('review_top link', 'event88,event188', '', e, 'eVar5')} }
                ]
            }
            ,trstlst : {
                url_reg : new RegExp('^.*/biz/area/pref/.*')
                ,trgt    : [
                     { elm  : '.line' ,efuc : setLineCode}
                ]
            }
        };

        // ちょっと初期処理
        if (!window.s) return;
        window.s.tlData = {};

        // URLからイベントマップを切り出し、再格納
        for (var i in EVENT_MAP) {
            if ( ! EVENT_MAP[i].url_reg.test(location.href) ) {
                continue;
            }
            EVENT_MAP = EVENT_MAP[i].trgt;
            break;
        }

        // イベントマップ適用
        for (var i=0, l=EVENT_MAP.length; i<l; i++) {
            var n = EVENT_MAP[i]
                ,t = $(n.elm)
                ;
            // 要素が無ければスキップ
            if (t.length == 0) continue;
            // イベントバインド
            t.on('click', n.efuc)
        }

        // データ発行
        function setData(idname, events, event_inc, e_obj, characteristic, tldata_params) {
            if (!idname) return;
            // events
            window.s.tlData = {events : events};
            // products
            if (window.analyticsData && window.analyticsData.products) {
                var n = window.analyticsData.products.split(';');
                n[4] = event_inc;
                window.s.tlData['products'] = n.join(';');
            }

            // 固有パラメータ
            if(characteristic){
                // 値はプロセッシングルールのため、s_code.jsで補完
                window.s.tlData[characteristic] = '';
            }

            // 固有パラメータ（値設定有）
            if (tldata_params) {
                for (var key in tldata_params) {
                    window.s.tlData[key] = tldata_params[key];
               }
            }

            // ドーン
            window.s.tl(true, 'o', idname);
        }

        /*
         * 以降カスタム
         */

        // お気に入り削除
        function setRemoveFav(e) {
            // 施設ID取得
            window.s.tlData.products = $(this).attr('href').split('=').pop();
            setData('remove_favorite', 'scRemove,event96', '', e);
        }

        // 予約ボタン
        function setCartButton(e) {
            // 押下ボタンの判定(aタグのみのボタンがあるため、spanタグのtextがない場合は自分自身のtextを取得)
            var b_txt  = $(this).find('span').text() || $(this).text()
                ,events = ''
                ,idname = ''
                ,event_inc = ''
                ,params = null
            ;
            
            // ikyuフラグ
            var ikyu_reg = new RegExp('^.*/dhotel/shisetsu/HT\\d{8}/IKYU/.*ci=.*');
            var is_ikyu = ikyu_reg.test(location.href);
            
            switch (b_txt) {
                case '現地支払い' :
                    events = 'event12,event36,event37,event38,event39,event40,event41,event42,event43,event112,event136';
                    idname = 'local_payment';
                    break;
                case 'オンライン決済' :
                    events = 'event13,event36,event37,event38,event39,event40,event41,event42,event43,event113,event136';
                    idname = 'online_settlement';
                    break;
                case '予約へ進む' :
                    if (is_ikyu) {
                        events = 'event36,event37,event38,event39,event40,event41,event42,event43,event136,event70,event170';
                        idname = 'click_agreement';
                        if (window.analyticsData && window.analyticsData.products && window.analyticsData.products.match(/eVar35=([0-9A-Za\z\_]+)/)) {
                            params = {};
                            params['prop37'] = RegExp.$1;
                        }
                    } else {
                        events = 'event36,event37,event38,event39,event40,event41,event42,event43,event136';
                        idname = 'click_external_link';
                    }
                    break;
                case 'ログインせず予約へ進む' :
                    var ikyu_reg = new RegExp('^.*/dhotel/shisetsu/HT\\d{8}/IKYU/.*ci=.*');
                    if( ikyu_reg.test(location.href)) {
                        events = 'event36,event37,event38,event39,event40,event41,event42,event43,event99,event136,event199';
                        idname = 'reserve_without_login';
                        if (window.analyticsData && window.analyticsData.products && window.analyticsData.products.match(/eVar35=([0-9A-Za\z\_]+)/)) {
                            params = {};
                            params['prop37'] = RegExp.$1;
                        }
                    }
                    break;
                case '上記に同意して予約へ進む' :
                case '確認事項に同意して予約へ進む' :
                    var ikyu_reg = new RegExp('^.*/dhotel/shisetsu/HT\\d{8}/IKYU/.*ci=.*');
                    if( ikyu_reg.test(location.href)) {
                        events = 'event36,event37,event38,event39,event40,event41,event42,event43,event136,event70,event170';
                        idname = 'click_agreement';
                        if (window.analyticsData && window.analyticsData.products && window.analyticsData.products.match(/eVar35=([0-9A-Za\z\_]+)/)) {
                            params = {};
                            params['prop37'] = RegExp.$1;
                        }
                        break;
                    }
                default:
                    return;
            }
            // s_products の載せ換え(ん？これはs_productsにのせんの？)
            try {
                event_inc = getPlanDetailEvent(events);
            } catch (e){}

            setData(idname, events, event_inc, e, null, params);
        }

        // プラン明細のイベントデータ生成
        function getPlanDetailEvent(events) {
            // 準備
            var param  = location.search.substring(1).split('&')
                ,sumNum = function(d){
                    var s = 0;
                    for(var i=0,l=d.length;i<l; i++) s = s + d[i] * 1;
                    return s;
                }
                ,pobj = {}
                ,event_org = {}
                ,event_str = []
                ;

            // パラメタ分解
            for (var i=0,pr; param[i]; i++) {
                pr = param[i].split('=');
                pobj[pr[0]] = pr[1];
            }

            // 人数,泊数計算
            var adlt   = pobj.adlt  ? sumNum(pobj.adlt.split('%2C'))  : 0
                ,chlda  = pobj.chlda ? sumNum(pobj.chlda.split('%2C')) : 0
                ,chldb  = pobj.chldb ? sumNum(pobj.chldb.split('%2C')) : 0
                ,chldc  = pobj.chldc ? sumNum(pobj.chldc.split('%2C')) : 0
                ,chldd  = pobj.chldd ? sumNum(pobj.chldd.split('%2C')) : 0
                ,chldo  = pobj.chldo ? sumNum(pobj.chldo.split('%2C')) : 0
                ,chld_all = chlda + chldb + chldc + chldd + chldo
                ;

            if (pobj.ci && pobj.co) {
                var ci     = pobj.ci.replace(/^(\d{4})(\d{2})(\d{2})$/g, function(){return RegExp.$1 + '-' + RegExp.$2 + '-' + RegExp.$3 ;})
                    ,co     = pobj.co.replace(/^(\d{4})(\d{2})(\d{2})$/g, function(){return RegExp.$1 + '-' + RegExp.$2 + '-' + RegExp.$3 ;})
                    ,diff_d = ((new Date(co)) - (new Date(ci))) / (3600000*24)
                    ;
            }

            // バリバリ入れてく(event_inc)
            event_org = {
                event37 : $('.totalPrice .price, .totalPriceBalloon .price').text().split('円')[0].split(',').join('')
                ,event38 : adlt + chld_all
                ,event40 : adlt
                ,event41 : chld_all
                ,event42 : diff_d || ''
                ,event43 : $(':hidden[name=rm]').val()
            }

            // 整形
            for (var i in event_org) {
                event_str.push(i+'='+event_org[i]);
            }
            return event_str.join('|');
        }

        // 路線名クリックデータ生成
        function setLineCode(e) {
            setData('bh_modal_stationlist', 'event93,event193', '', e, '', { eVar55 : $(this).data('line_code') });
        }

    } catch (e){}
});
