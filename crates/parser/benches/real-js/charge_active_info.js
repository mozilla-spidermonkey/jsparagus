define( function () {
    var $plus = $( '.plus-js' );
    var $minus = $( '.minus-js' );
    var $num_g = $( '.num' );
    var $count = $( '.count-js' );

    function init () {
        total();
        bindEvent();
    }

    function bindEvent () {
        $plus.bind( 'click', function () {
            plus( this );
            return false;
        } );

        $minus.bind( 'click', function () {
            minus( this );
            return false;
        } );

        $num_g.bind( 'blur', function () {
            changeNum( this );
        } );
    }

    // 加操作
    function plus ( _this ) {
        var $this = $( _this );
        var conf = getPriceAndNum( $this );
        var val = conf.$num.val();
        var num = parseFloat( val );

        if ( num == 0 )
        {
            conf.$num.val( conf.$min.val() );
            total();
        }
        else if ( num < conf.$max.val() )
        {
            num += 1;
            conf.$num.val( isNaN( num ) ? 0 : num );
            conf.$num_hide.val( isNaN( num ) ? 0 : num );
            total();

        }
        else
        {
            alert( '最大值为'+conf.$num.val() );

            return false;
        }
    }

    // 减操作
    function minus ( _this ) {
        var $this = $( _this );
        var conf = getPriceAndNum( $this );
        var num = parseFloat( conf.$num.val() );
        var min = parseFloat( conf.$min.val() );

        if ( num > ( min - 1 ) )
        {
            if(num == min){
                num -= min;
            }else{
                num -= 1;
            }
            conf.$num.val( isNaN( num ) ? 0 : num );
            conf.$num_hide.val( isNaN( num ) ? 0 : num );
            total();

        }
        else
        {
            //alert( '最小值为'+ min );
            return false;
        }
    }

    // 手工修改数量
    function changeNum ( _this ) {
        var $this = $( _this );
        var num = parseFloat( $this.val() );
        var conf = getPriceAndNum( $this );

        //手动输入0保留原值不做处理
        if(num==0){
            //conf.$num_hide.val( conf.$min.val() );
            //$this.val( conf.$min.val() );
            //alert( '最小值为'+ conf.$min.val());
            //return false;
        }else if ( num > conf.$max.val() )
        {
            conf.$num_hide.val( conf.$max.val() );
            $this.val( conf.$max.val() );
            alert( '最大值为'+conf.$max.val() );
        }
        else if ( num < conf.$min.val() )
        {
            conf.$num_hide.val( conf.$min.val() );
            $this.val( conf.$min.val() );
            alert( '最小值为'+ conf.$min.val());
        }
        else if ( isNaN( num ) )
        {
            alert( '请输入正确的票数！' );
            $this.focus();

            return false;
        }
        else
        {
            conf.$num_hide.val( num );
        }

        total();
    }

    // 更改submit状态
    function change_submit(tip){
        //符合提交条件
        if(tip==1){
            $('#product_goodslist_submit').removeAttr('disabled');
            $('#product_goodslist_submit').attr({style:'background:none repeat scroll 0 0 #DC3C00'});
        }else{
            $('#product_goodslist_submit').attr({disabled:'disabled',style:'background:none repeat scroll 0 0 #C1C1C1'});

        }


    }

    // 总计
    function total ( isLoad ) {
        var $trs = $( '.active-info tbody' ).find( 'tr' );
        var num = 0.00, $cost, cost, $num, num_temp, $min, $tr;

        if ( isLoad )
        {
            $num_g.each( function ( i, item ) {
                /*
                 var  $item = $( item );
                 var $tr = $item.parents( 'tr' );
                 var $min = $tr.find( '.min_buy_num' );
                 */

                $( item ).val( 0 );
            } );
        }
        else
        {
            $trs.each( function ( i, tr ) {
                $tr = $( tr );
                $cost = $tr.find( '.cost' ).length > 0
                    ? $tr.find( '.cost' )
                    : $tr.find( '.price' );
                $min = $tr.find( '.min_buy_num' );
                $num = $tr.find( '.num' );
                cost = $cost.html();
                cost = parseFloat( cost );
                num_temp = parseFloat( $num.val() );
                num_temp = isNaN( num_temp ) ? 0 : num_temp;

                num += cost * num_temp;
            } );
        }

        $count.html( new Number( num ).toFixed( 2 ) );
    }

    // 返回“原价”和“数量”的内容
    function getPriceAndNum ( $this ) {
        var $tr = $this.parents( 'tr' );
        var $num = $tr.find( '.num' );
        var $num_hide = $tr.find( '.num-hide' );
        var $max = $tr.find( '.max_buy_num' );
        var $min = $tr.find( '.min_buy_num' );
        var $price = $tr.find( '.cost' ).length > 0
            ? $tr.find( '.cost' )
            : $tr.find( '.price' );

        return {
            price: parseFloat( $price.html() ),
            $num_hide: $num_hide,
            $max: $max,
            $min: $min,
            $num: $num
        }
    }

    return {
        init: init
    }
} );