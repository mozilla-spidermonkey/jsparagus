$(function () {
    var _activity_goods_list = {};
    _activity_goods_list._submit_button = $("#goods_submit_button");
    _activity_goods_list._item_js = $(".item-js");
    _activity_goods_list.goods_num = $(".goods_num");
    _activity_goods_list.list = $('.active-info tbody').find('tr');
    _activity_goods_list.count_js = $(".count-js");
    _activity_goods_list.project_id = $("#project_id").val();
    _activity_goods_list.use_coupon_code = $("#use_coupon_code");
    _activity_goods_list.use_coupon_code_button = $("#use_coupon_code_button");
    _activity_goods_list.change_coupon_code_button = $("#change_coupon_code_button");
    _activity_goods_list.coupon_code_div = $("#coupon_code_div");
    // 父页面的iframe对象
    _activity_goods_list.parent_frame = parent.document.getElementById("ticket_list");

    // 没有票, 不显示购票列表
    if(_activity_goods_list.list.size() == 0){
        _activity_goods_list.parent_frame.remove();
        return false;
    }

    // 高度发生变化时,修改iframe高度
    _activity_goods_list.IFrameResize = function(){
        var iframeWin = _activity_goods_list.parent_frame.contentWindow || _activity_goods_list.parent_frame.contentDocument.parentWindow;
        if (iframeWin.document.body) {
            var iframe_new_height = $(iframeWin.document.body).height();
            // console.log(iframe_new_height);
            _activity_goods_list.parent_frame.style.height = iframe_new_height + "px";  //调整父页面中IFrame的高度为此页面的高度
        }
    };
    //票务的描述
    _activity_goods_list.InvoiceDesc = function(){
        // 判断是否为移动端运行环境
        if(/AppleWebKit.*Mobile/i.test(navigator.userAgent) || (/MIDP|SymbianOS|NOKIA|SAMSUNG|LG|NEC|TCL|Alcatel|BIRD|DBTEL|Dopod|PHILIPS|HAIER|LENOVO|MOT-|Nokia|SonyEricsson|SIE-|Amoi|ZTE/.test(navigator.userAgent))){
            if(window.location.href.indexOf("?mobile")<0){
                try{
                    if(/Android|webOS|iPhone|iPod|BlackBerry/i.test(navigator.userAgent)){
                        // 判断访问环境是 Android|webOS|iPhone|iPod|BlackBerry 则加载以下
                        $('.ticketName_prompt').on('singleTap',function(){
                            $(this).find('.arrowD').toggle();
                            $(this).find('.arrowUp').toggle();
                            $(this).find('.prompt_btn').toggle();
                            $(this).find('.prompt_con').toggle();
                            _activity_goods_list.IFrameResize();
                        });
                    }
                    else if(/iPad/i.test(navigator.userAgent)){
                        // 判断访问环境是 iPad 则加载以下
                        $('.ticketName_prompt').on('singleTap',function(){
                            $(this).find('.arrowD').toggle();
                            $(this).find('.arrowUp').toggle();
                            $(this).find('.prompt_btn').toggle();
                            $(this).find('.prompt_con').toggle();
                            _activity_goods_list.IFrameResize();
                        });
                    }
                    else{
                        // 判断访问环境是 其他移动设备 则加载以下
                        $('.ticketName_prompt').on('singleTap',function(){
                            $(this).find('.arrowD').toggle();
                            $(this).find('.arrowUp').toggle();
                            $(this).find('.prompt_btn').toggle();
                            $(this).find('.prompt_con').toggle();
                            _activity_goods_list.IFrameResize();
                        });
                    }

                }
                catch(e){}
            }
        }else{
            // 如果以上都不是，则加载以下(PC      //跟随鼠标显示：
            $('.ticketName_td span').mousemove(function() {

                $(this).siblings('.ticketName_proCon').show();
            })
            $('.ticketName_td span').mouseout(function() {
                $(this).siblings('.ticketName_proCon').hide();
            });
        }
    };
    _activity_goods_list.IFrameResize();
   //票务描述
    _activity_goods_list.InvoiceDesc();
    // 尺寸发生变化时,调整高度
    $(window).resize(function() {
        _activity_goods_list.IFrameResize();
    });

    // 显示优惠码
    _activity_goods_list.show_use_coupon = function () {
        // 隐藏使用优惠码按钮
        _activity_goods_list.use_coupon_code_button.hide();
        // 隐藏立即购买按钮
        _activity_goods_list._submit_button.hide();
        // 隐藏换一个优惠码div
        _activity_goods_list.coupon_code_div.hide();
        // 显示使用优惠码组件
        _activity_goods_list.use_coupon_code.show();

        $("#coupon_code_input").focus();

        _activity_goods_list.total_price();
        _activity_goods_list.IFrameResize();
    };

    // 隐藏优惠码
    _activity_goods_list.hide_use_coupon = function () {
        // 显示立即购买按钮
        _activity_goods_list._submit_button.show();
        // 隐藏使用优惠码组件
        _activity_goods_list.use_coupon_code.hide();

        if(_activity_goods_list.coupon_code_div.find(".coupon_code_text").text() == ""){
            _activity_goods_list.use_coupon_code_button.show();
        }else{
            _activity_goods_list.coupon_code_div.show();
        }

        _activity_goods_list.total_price();
        _activity_goods_list.IFrameResize();
    };

    // 使用优惠码取消按钮
    _activity_goods_list.use_coupon_code.find(".cancel").on("click", function () {
        _activity_goods_list.hide_use_coupon();
    });

    // 校验优惠码失败
    _activity_goods_list.check_coupon_error = function(msg){
        _activity_goods_list.use_coupon_code.find(".confirm").hide();
        _activity_goods_list.use_coupon_code.find(".cancel").hide();
        _activity_goods_list.use_coupon_code.find(".check").text(msg).show();
        _activity_goods_list.coupon_code_div.hide().find(".coupon_code_price").empty();
    };

    // 使用优惠码确定按钮
    _activity_goods_list.use_coupon_code.find(".confirm").on("click", function () {
        // console.log("aaa");
        var _this = $(this);

        var _coupon_code_input = $("#coupon_code_input")
            ,_coupon_code = _coupon_code_input.val() || false;
        if(_coupon_code == "" || _coupon_code == "输入优惠码" || !_coupon_code){
            _activity_goods_list.coupon_code_div.find(".coupon_code_text").empty();
            _activity_goods_list.coupon_code_div.find(".coupon_code_price").empty();
            _activity_goods_list.hide_use_coupon();
            return false;
        }

        var goods_num = $(".goods_num");
        var _project_id = parseInt($("#project_id").val());
        var _goods_list = [];

        goods_num.each(function(){
            var _this_goods = $(this)
                ,_num = parseInt(_this_goods.val());

            if(_num && _num > 0){
                _goods_list.push({
                    goods_id: parseInt(_this_goods.siblings(".goods_id").val()),
                    goods_price: parseFloat(_this_goods.parents("td").siblings(".cost").text()),
                    num: _num
                });
            }
        });

        if(_goods_list){
            _this.hide().siblings(".cancel").hide().siblings(".check").show();

            $.ajax({
                type: "post",
                url: "/api/activity_api/check_coupon_code",
                timeout: 5000,
                data: {
                    coupon_code: _coupon_code,
                    project_id: _project_id,
                    goods_list: _goods_list
                },
                dataType: "json",
                success: function(o){
                    if(o.status == 1){
                        // alert("success");
                        _activity_goods_list.coupon_code_div.find(".coupon_code_text").text(_coupon_code);
                        _activity_goods_list.coupon_code_div.find(".coupon_code_price").text(parseFloat(o.price).toFixed(2));
                        _activity_goods_list.coupon_code_div.show();

                        _activity_goods_list.hide_use_coupon();

                        _activity_goods_list.IFrameResize();
                    }else{
                        // alert(o.msg);
                        _activity_goods_list.check_coupon_error(o.msg);
                    }
                },
                error: function(){
                    // alert("服务端错误");
                    _activity_goods_list.check_coupon_error("服务端错误");
                },
                complete: function (XMLHttpRequest, status) {
                    if (status == 'timeout') {
                        // alert("请求超时");
                        _activity_goods_list.check_coupon_error("请求超时");
                    }

                    // 指定时间后自动恢复确认和取消按钮
                    setTimeout(function(){
                        _this.show().siblings(".cancel").show().siblings(".check").hide().text("正在验证");
                    },2000);
                }
            });
        }
    });

    // 使用优惠码按钮
    _activity_goods_list.use_coupon_code_button.on("click", function () {
        _activity_goods_list.show_use_coupon();
    });

    // 换一个优惠码
    _activity_goods_list.change_coupon_code_button.on("click",function(){
        _activity_goods_list.show_use_coupon();
    });

    // 立即购买 报错
    _activity_goods_list.show_error = function (msg) {
        console.log(msg);
    };

    // 立即购买 请求超时
    _activity_goods_list.timeout = function () {
        console.log("请求超时");
    };

    // 验证是否是pc访问
    _activity_goods_list.IsPC = function(){
        /*var ua = window.navigator.userAgent.toLowerCase();
        if(ua.match(/MicroMessenger/i) == 'micromessenger'){
            return true;
        }else{
            return false;
        }*/
            var userAgentInfo = navigator.userAgent;
            var Agents = ["Android", "iPhone",
                "SymbianOS", "Windows Phone",
                "iPad", "iPod"];
            var flag = true;
            for (var v = 0; v < Agents.length; v++) {
                if (userAgentInfo.indexOf(Agents[v]) > 0) {
                    flag = false;
                    break;
                }
            }
            return flag;

    }

    _activity_goods_list._submit_button.on("click", function () {
        var goods_num = $(".goods_num");
        if (goods_num.size() > 0) {
            var num_count = 0;
            goods_num.each(function () {
                num_count = num_count + parseInt($(this).val());
            });

            if (num_count == 0) {
                alert('商品选择数量为空');
                return false;
            } else {
                var currentName = csdn.getCookie('UserName');
                if (currentName == null || currentName == '' || !currentName) {
                    // 判断是否为微信浏览器
                    /*if(!_activity_goods_list.IsPC()){
                        parent.location.href = "https://passport.csdn.net/account/login?from=" + encodeURI(parent.location.href);
                    }else{
                        parent.csdn.showLogin(function () {
                            $("#goods_submit_button").trigger('click');
                        });
                    }
                    return false;*/
                    parent.location.href = "https://passport.csdn.net/account/login?from=" + encodeURI(parent.location.href);
                } else {
                    var flag = 4;
                    var num_item, project_id_array = [], goods_id_array = [], buy_num_array = [];
                    _activity_goods_list.list.each(function () {
                        num_item = $(this).find(".goods_num").val();
                        if (num_item > 0) {
                            project_id_array.push(_activity_goods_list.project_id);
                            goods_id_array.push($(this).find(".goods_id").val());
                            buy_num_array.push($(this).find(".goods_num").val());
                        }
                    });

                    cart.buy_now(project_id_array.toString(), goods_id_array.toString(), buy_num_array.toString(), flag, _activity_goods_list.show_error, _activity_goods_list.timeout, {"coupon_code": _activity_goods_list.coupon_code_div.find(".coupon_code_text").text()});
                }
            }
        }
    });

    _activity_goods_list.total_price = function(){
        var num = 0, num_item, cost_num, _coupon_code_price;
        _activity_goods_list.list.each(function () {
            cost_num = ($(this).find(".cost").size() > 0 && parseFloat($(this).find(".cost").text()) > 0) ? $(this).find(".cost") : $(this).find(".price");
            num_item = $(this).find(".goods_num").val();
            num = parseFloat(num) + parseFloat(cost_num.text()) * num_item;
        });

        num = num ? num : 0;
        _coupon_code_price = parseFloat(_activity_goods_list.coupon_code_div.find(".coupon_code_price").text()) || 0;

        num = _coupon_code_price > 0 ? ((num - _coupon_code_price) < 0 ? 0 : (num - _coupon_code_price)) : num;

        num = num.toFixed(2);
        _activity_goods_list.count_js.text(num);
    };

    _activity_goods_list.set_num = function (current_input, val) {
        current_input.val(val);
        _activity_goods_list.total_price();
    };

    _activity_goods_list.check_num = function (current_input, new_val, min_val, max_val) {
        // 不是数字
        if (new_val == null || !new_val) {
            new_val = 0;
        }

        // 小于0 或者小于最小购买数量
        if (new_val < 0 || (new_val > 0 && new_val < min_val)) {
            _activity_goods_list.set_num(current_input, min_val);
            console.warn("最小值为：" + min_val);
            return false;
        }

        // 大于最大购买数量
        if (new_val > max_val) {
            _activity_goods_list.set_num(current_input, max_val);
            console.warn("最大值为：" + max_val);
            return false;
        }

        _activity_goods_list.set_num(current_input, new_val);
    };

    _activity_goods_list._item_js.on("click", function () {
        var _this = $(this), _type = _this.attr("data-type"),
            _current_input = _this.siblings(".goods_num"),
            _current_old_val = parseInt(_current_input.val()),
            _current_min_val = parseInt(_this.siblings(".min_buy_num").val()),
            _current_max_val = parseInt(_this.siblings(".max_buy_num").val()),
            _current_new_val;

        if (_type == "minus") {
            _current_new_val = _current_old_val - 1;
            if (_current_new_val < _current_min_val) {
                _current_new_val = 0;
            }
        }

        if (_type == "plus") {
            _current_new_val = _current_old_val + 1;
            if (_current_new_val < _current_min_val) {
                _current_new_val = _current_min_val;
            }
        }

        _activity_goods_list.check_num(_current_input, _current_new_val, _current_min_val, _current_max_val);
    });

    _activity_goods_list.goods_num.on("blur", function () {
        var _current_input = $(this),
            _current_min_val = parseInt(_current_input.siblings(".min_buy_num").val()),
            _current_max_val = parseInt(_current_input.siblings(".max_buy_num").val()),
            _current_new_val = parseInt(_current_input.val());

        _activity_goods_list.check_num(_current_input, _current_new_val, _current_min_val, _current_max_val);
    });

    // 显示ifrmae
    _activity_goods_list.parent_frame.style.display = "block";
    _activity_goods_list.IFrameResize();




});