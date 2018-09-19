<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_Implementation Toolkit</name>
   <tag></tag>
   <elementGuidId>d78175ff-ad31-41fd-98df-74deb26e1796</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value> js flexbox flexboxlegacy canvas canvastext webgl no-touch geolocation postmessage websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms no-csstransforms3d csstransitions fontface no-generatedcontent video audio localstorage sessionstorage webworkers applicationcache svg inlinesvg smil svgclippaths</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
    
    Implementation Toolkit

   
    
    
    
    
    
    
    
    
    
    


    



        nav.navbar.navbar-inverse {
    background: linear-gradient(90deg,#7c2a55, #e26632,#f6aa00,#e26632, #7c2a55);
}

        body{
            font-size:17px !important;
             font-family: Tahoma,Verdana,Segoe,sans-serif !important;
        }

        /*NEW nav BAR*/
        .navbar-inverse {
            background-color: #7c2a55;
            border-color: #ffffff;
        }
        /*END NEW nav BAR*/



        li {
            margin-left: 24px;
        }

        section.menu-section {
            height: 50px;
        }

        /*For Logout button*/
        li.logout {
            margin-top: 11px;
        }

        li.user {
            margin-top: 11px;
        }


        li.Home {
            margin-top: 11px;
        }
        /*END For Logout button*/


        .dropbtn {
            background-color: transparent;
            color: white;
            padding: 15px;
            font-size: 16px;
            border: none;
            cursor: pointer;
        }

        .dropdown {
            position: relative;
            display: inline-block;
        }

        .dropdown-content {
            display: none;
            position: absolute;
            right: 0;
            background-color: #d65f35;
            min-width: 160px;
            box-shadow: 0px 8px 16px 0px rgba(0,0,0,0.2);
            z-index: 9999;
        }

            .dropdown-content a {
                color: #ffffff;
                padding: 12px 16px;
                text-decoration: none;
                display: block;
            }

                .dropdown-content a:hover {
                    background-color: #a25a179c;
                }

        .dropdown:hover .dropdown-content {
            display: block;
        }

        .dropdown:hover .dropbtn {
            background-color: #d65f35;
        }

        .divfix {
            /* bottom: 0;*/
            right: 10px;
            position: absolute;
            z-index: 3000;
        }


        .mybutton {
            position: fixed;
            /* margin-top:250px;
            margin-left:850px;
           bottom: -1000px;
            right: 5px; */
            background-color: #f3f3f0;
            display: none;
            z-index: 9999;
            overflow: hidden;
            cursor: auto;
            bottom: 0px;
            right: 0px;
        }

        .chatbotDiv {
            bottom: 0px;
            right: 0px;
            position: fixed;
            z-index: 3000;
            cursor: auto;
        }

        .closebtn {
            color: red;
        }

        .fixHead {
            position: fixed;
            right: 0;
            bottom: 0;
            display: none;
        }

        .divfix .tooltiptext {
            visibility: hidden;
            width: 120px;
            background-color: black;
            color: #fff;
            text-align: center;
            border-radius: 6px;
            padding: 5px 0;
            position: absolute;
            z-index: 1;
            /*bottom: 100%;*/
            left: 50%;
            margin-left: -110px;
            /* Fade in tooltip - takes 1 second to go from 0% to 100% opac: */
            opacity: 0;
            transition: opacity 1s;
        }

        .divfix:hover .tooltiptext {
            visibility: visible;
            opacity: 1;
        }


        .fixHead .tooltiptext {
            visibility: hidden;
            width: 120px;
            background-color: black;
            color: #fff;
            text-align: center;
            border-radius: 6px;
            padding: 5px 0;
            position: absolute;
            z-index: 1;
            bottom: 100%;
            left: 50%;
            margin-left: -60px;
            /* Fade in tooltip - takes 1 second to go from 0% to 100% opac: */
            opacity: 0;
            transition: opacity 1s;
        }

        .fixHead:hover .tooltiptext {
            visibility: visible;
            opacity: 1;
        }

        body {
            padding-top: 25px;
            padding-bottom: 20px;
        }

        
        img {
            height: 40px;
        }

        /*HARRI*/
        img.sdbot {
            height: 120px;
            position: absolute;
            margin-top: -22px;
        }

        .dropdown-submenu {
            position: relative;
        }

        .dropdown-submenu .dropdown-menu {
            top: 0;
            left: 100%;
            margin-top: -1px;
        }
    


    
    
    
    
    

    
    
    
    .notifyjs-bootstrap-base {
	font-weight: bold;
	padding: 8px 15px 8px 14px;
	text-shadow: 0 1px 0 rgba(255, 255, 255, 0.5);
	background-color: #fcf8e3;
	border: 1px solid #fbeed5;
	-webkit-border-radius: 4px;
	-moz-border-radius: 4px;
	border-radius: 4px;
	white-space: nowrap;
	padding-left: 25px;
	background-repeat: no-repeat;
	background-position: 3px 7px;
}
.notifyjs-bootstrap-error {
	color: #B94A48;
	background-color: #F2DEDE;
	border-color: #EED3D7;
	background-image: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABQAAAAUCAYAAACNiR0NAAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAAtRJREFUeNqkVc1u00AQHq+dOD+0poIQfkIjalW0SEGqRMuRnHos3DjwAH0ArlyQeANOOSMeAA5VjyBxKBQhgSpVUKKQNGloFdw4cWw2jtfMOna6JOUArDTazXi/b3dm55socPqQhFka++aHBsI8GsopRJERNFlY88FCEk9Yiwf8RhgRyaHFQpPHCDmZG5oX2ui2yilkcTT1AcDsbYC1NMAyOi7zTX2Agx7A9luAl88BauiiQ/cJaZQfIpAlngDcvZZMrl8vFPK5+XktrWlx3/ehZ5r9+t6e+WVnp1pxnNIjgBe4/6dAysQc8dsmHwPcW9C0h3fW1hans1ltwJhy0GxK7XZbUlMp5Ww2eyan6+ft/f2FAqXGK4CvQk5HueFz7D6GOZtIrK+srupdx1GRBBqNBtzc2AiMr7nPplRdKhb1q6q6zjFhrklEFOUutoQ50xcX86ZlqaZpQrfbBdu2R6/G19zX6XSgh6RX5ubyHCM8nqSID6ICrGiZjGYYxojEsiw4PDwMSL5VKsC8Yf4VRYFzMzMaxwjlJSlCyAQ9l0CW44PBADzXhe7xMdi9HtTrdYjFYkDQL0cn4Xdq2/EAE+InCnvADTf2eah4Sx9vExQjkqXT6aAERICMewd/UAp/IeYANM2joxt+q5VI+ieq2i0Wg3l6DNzHwTERPgo1ko7XBXj3vdlsT2F+UuhIhYkp7u7CarkcrFOCtR3H5JiwbAIeImjT/YQKKBtGjRFCU5IUgFRe7fF4cCNVIPMYo3VKqxwjyNAXNepuopyqnld602qVsfRpEkkz+GFL1wPj6ySXBpJtWVa5xlhpcyhBNwpZHmtX8AGgfIExo0ZpzkWVTBGiXCSEaHh62/PoR0p/vHaczxXGnj4bSo+G78lELU80h1uogBwWLf5YlsPmgDEd4M236xjm+8nm4IuE/9u+/PH2JXZfbwz4zw1WbO+SQPpXfwG/BBgAhCNZiSb/pOQAAAAASUVORK5CYII=);
}
.notifyjs-bootstrap-success {
	color: #468847;
	background-color: #DFF0D8;
	border-color: #D6E9C6;
	background-image: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABQAAAAUCAYAAACNiR0NAAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAAutJREFUeNq0lctPE0Ecx38zu/RFS1EryqtgJFA08YCiMZIAQQ4eRG8eDGdPJiYeTIwHTfwPiAcvXIwXLwoXPaDxkWgQ6islKlJLSQWLUraPLTv7Gme32zoF9KSTfLO7v53vZ3d/M7/fIth+IO6INt2jjoA7bjHCJoAlzCRw59YwHYjBnfMPqAKWQYKjGkfCJqAF0xwZjipQtA3MxeSG87VhOOYegVrUCy7UZM9S6TLIdAamySTclZdYhFhRHloGYg7mgZv1Zzztvgud7V1tbQ2twYA34LJmF4p5dXF1KTufnE+SxeJtuCZNsLDCQU0+RyKTF27Unw101l8e6hns3u0PBalORVVVkcaEKBJDgV3+cGM4tKKmI+ohlIGnygKX00rSBfszz/n2uXv81wd6+rt1orsZCHRdr1Imk2F2Kob3hutSxW8thsd8AXNaln9D7CTfA6O+0UgkMuwVvEFFUbbAcrkcTA8+AtOk8E6KiQiDmMFSDqZItAzEVQviRkdDdaFgPp8HSZKAEAL5Qh7Sq2lIJBJwv2scUqkUnKoZgNhcDKhKg5aH+1IkcouCAdFGAQsuWZYhOjwFHQ96oagWgRoUov1T9kRBEODAwxM2QtEUl+Wp+Ln9VRo6BcMw4ErHRYjH4/B26AlQoQQTRdHWwcd9AH57+UAXddvDD37DmrBBV34WfqiXPl61g+vr6xA9zsGeM9gOdsNXkgpEtTwVvwOklXLKm6+/p5ezwk4B+j6droBs2CsGa/gNs6RIxazl4Tc25mpTgw/apPR1LYlNRFAzgsOxkyXYLIM1V8NMwyAkJSctD1eGVKiq5wWjSPdjmeTkiKvVW4f2YPHWl3GAVq6ymcyCTgovM3FzyRiDe2TaKcEKsLpJvNHjZgPNqEtyi6mZIm4SRFyLMUsONSSdkPeFtY1n0mczoY3BHTLhwPRy9/lzcziCw9ACI+yql0VLzcGAZbYSM5CCSZg1/9oc/nn7+i8N9p/8An4JMADxhH+xHfuiKwAAAABJRU5ErkJggg==);
}
.notifyjs-bootstrap-info {
	color: #3A87AD;
	background-color: #D9EDF7;
	border-color: #BCE8F1;
	background-image: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABQAAAAUCAYAAACNiR0NAAAABmJLR0QA/wD/AP+gvaeTAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAB3RJTUUH3QYFAhkSsdes/QAAA8dJREFUOMvVlGtMW2UYx//POaWHXg6lLaW0ypAtw1UCgbniNOLcVOLmAjHZolOYlxmTGXVZdAnRfXQm+7SoU4mXaOaiZsEpC9FkiQs6Z6bdCnNYruM6KNBw6YWewzl9z+sHImEWv+vz7XmT95f/+3/+7wP814v+efDOV3/SoX3lHAA+6ODeUFfMfjOWMADgdk+eEKz0pF7aQdMAcOKLLjrcVMVX3xdWN29/GhYP7SvnP0cWfS8caSkfHZsPE9Fgnt02JNutQ0QYHB2dDz9/pKX8QjjuO9xUxd/66HdxTeCHZ3rojQObGQBcuNjfplkD3b19Y/6MrimSaKgSMmpGU5WevmE/swa6Oy73tQHA0Rdr2Mmv/6A1n9w9suQ7097Z9lM4FlTgTDrzZTu4StXVfpiI48rVcUDM5cmEksrFnHxfpTtU/3BFQzCQF/2bYVoNbH7zmItbSoMj40JSzmMyX5qDvriA7QdrIIpA+3cdsMpu0nXI8cV0MtKXCPZev+gCEM1S2NHPvWfP/hL+7FSr3+0p5RBEyhEN5JCKYr8XnASMT0xBNyzQGQeI8fjsGD39RMPk7se2bd5ZtTyoFYXftF6y37gx7NeUtJJOTFlAHDZLDuILU3j3+H5oOrD3yWbIztugaAzgnBKJuBLpGfQrS8wO4FZgV+c1IxaLgWVU0tMLEETCos4xMzEIv9cJXQcyagIwigDGwJgOAtHAwAhisQUjy0ORGERiELgG4iakkzo4MYAxcM5hAMi1WWG1yYCJIcMUaBkVRLdGeSU2995TLWzcUAzONJ7J6FBVBYIggMzmFbvdBV44Corg8vjhzC+EJEl8U1kJtgYrhCzgc/vvTwXKSib1paRFVRVORDAJAsw5FuTaJEhWM2SHB3mOAlhkNxwuLzeJsGwqWzf5TFNdKgtY5qHp6ZFf67Y/sAVadCaVY5YACDDb3Oi4NIjLnWMw2QthCBIsVhsUTU9tvXsjeq9+X1d75/KEs4LNOfcdf/+HthMnvwxOD0wmHaXr7ZItn2wuH2SnBzbZAbPJwpPx+VQuzcm7dgRCB57a1uBzUDRL4bfnI0RE0eaXd9W89mpjqHZnUI5Hh2l2dkZZUhOqpi2qSmpOmZ64Tuu9qlz/SEXo6MEHa3wOip46F1n7633eekV8ds8Wxjn37Wl63VVa+ej5oeEZ/82ZBETJjpJ1Rbij2D3Z/1trXUvLsblCK0XfOx0SX2kMsn9dX+d+7Kf6h8o4AIykuffjT8L20LU+w4AZd5VvEPY+XpWqLV327HR7DzXuDnD8r+ovkBehJ8i+y8YAAAAASUVORK5CYII=);
}
.notifyjs-bootstrap-warn {
	color: #C09853;
	background-color: #FCF8E3;
	border-color: #FBEED5;
	background-image: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABQAAAAUCAMAAAC6V+0/AAABJlBMVEXr6eb/2oD/wi7/xjr/0mP/ykf/tQD/vBj/3o7/uQ//vyL/twebhgD/4pzX1K3z8e349vK6tHCilCWbiQymn0jGworr6dXQza3HxcKkn1vWvV/5uRfk4dXZ1bD18+/52YebiAmyr5S9mhCzrWq5t6ufjRH54aLs0oS+qD751XqPhAybhwXsujG3sm+Zk0PTwG6Shg+PhhObhwOPgQL4zV2nlyrf27uLfgCPhRHu7OmLgAafkyiWkD3l49ibiAfTs0C+lgCniwD4sgDJxqOilzDWowWFfAH08uebig6qpFHBvH/aw26FfQTQzsvy8OyEfz20r3jAvaKbhgG9q0nc2LbZxXanoUu/u5WSggCtp1anpJKdmFz/zlX/1nGJiYmuq5Dx7+sAAADoPUZSAAAAAXRSTlMAQObYZgAAAAFiS0dEAIgFHUgAAAAJcEhZcwAACxMAAAsTAQCanBgAAAAHdElNRQfdBgUBGhh4aah5AAAAlklEQVQY02NgoBIIE8EUcwn1FkIXM1Tj5dDUQhPU502Mi7XXQxGz5uVIjGOJUUUW81HnYEyMi2HVcUOICQZzMMYmxrEyMylJwgUt5BljWRLjmJm4pI1hYp5SQLGYxDgmLnZOVxuooClIDKgXKMbN5ggV1ACLJcaBxNgcoiGCBiZwdWxOETBDrTyEFey0jYJ4eHjMGWgEAIpRFRCUt08qAAAAAElFTkSuQmCC);
}

    
    
    
    
       
    
    
    
        $(document).ready(function(){
            $('.dropdown-submenu a.navmenu').on(&quot;click&quot;, function(e){
                $(this).next('ul').toggle();
                e.stopPropagation();
                e.preventDefault();
            });
        });
    

.notifyjs-bootstrap-base {
	font-weight: bold;
	padding: 8px 15px 8px 14px;
	text-shadow: 0 1px 0 rgba(255, 255, 255, 0.5);
	background-color: #fcf8e3;
	border: 1px solid #fbeed5;
	-webkit-border-radius: 4px;
	-moz-border-radius: 4px;
	border-radius: 4px;
	white-space: nowrap;
	padding-left: 25px;
	background-repeat: no-repeat;
	background-position: 3px 7px;
}
.notifyjs-bootstrap-error {
	color: #B94A48;
	background-color: #F2DEDE;
	border-color: #EED3D7;
	background-image: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABQAAAAUCAYAAACNiR0NAAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAAtRJREFUeNqkVc1u00AQHq+dOD+0poIQfkIjalW0SEGqRMuRnHos3DjwAH0ArlyQeANOOSMeAA5VjyBxKBQhgSpVUKKQNGloFdw4cWw2jtfMOna6JOUArDTazXi/b3dm55socPqQhFka++aHBsI8GsopRJERNFlY88FCEk9Yiwf8RhgRyaHFQpPHCDmZG5oX2ui2yilkcTT1AcDsbYC1NMAyOi7zTX2Agx7A9luAl88BauiiQ/cJaZQfIpAlngDcvZZMrl8vFPK5+XktrWlx3/ehZ5r9+t6e+WVnp1pxnNIjgBe4/6dAysQc8dsmHwPcW9C0h3fW1hans1ltwJhy0GxK7XZbUlMp5Ww2eyan6+ft/f2FAqXGK4CvQk5HueFz7D6GOZtIrK+srupdx1GRBBqNBtzc2AiMr7nPplRdKhb1q6q6zjFhrklEFOUutoQ50xcX86ZlqaZpQrfbBdu2R6/G19zX6XSgh6RX5ubyHCM8nqSID6ICrGiZjGYYxojEsiw4PDwMSL5VKsC8Yf4VRYFzMzMaxwjlJSlCyAQ9l0CW44PBADzXhe7xMdi9HtTrdYjFYkDQL0cn4Xdq2/EAE+InCnvADTf2eah4Sx9vExQjkqXT6aAERICMewd/UAp/IeYANM2joxt+q5VI+ieq2i0Wg3l6DNzHwTERPgo1ko7XBXj3vdlsT2F+UuhIhYkp7u7CarkcrFOCtR3H5JiwbAIeImjT/YQKKBtGjRFCU5IUgFRe7fF4cCNVIPMYo3VKqxwjyNAXNepuopyqnld602qVsfRpEkkz+GFL1wPj6ySXBpJtWVa5xlhpcyhBNwpZHmtX8AGgfIExo0ZpzkWVTBGiXCSEaHh62/PoR0p/vHaczxXGnj4bSo+G78lELU80h1uogBwWLf5YlsPmgDEd4M236xjm+8nm4IuE/9u+/PH2JXZfbwz4zw1WbO+SQPpXfwG/BBgAhCNZiSb/pOQAAAAASUVORK5CYII=);
}
.notifyjs-bootstrap-success {
	color: #468847;
	background-color: #DFF0D8;
	border-color: #D6E9C6;
	background-image: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABQAAAAUCAYAAACNiR0NAAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAAutJREFUeNq0lctPE0Ecx38zu/RFS1EryqtgJFA08YCiMZIAQQ4eRG8eDGdPJiYeTIwHTfwPiAcvXIwXLwoXPaDxkWgQ6islKlJLSQWLUraPLTv7Gme32zoF9KSTfLO7v53vZ3d/M7/fIth+IO6INt2jjoA7bjHCJoAlzCRw59YwHYjBnfMPqAKWQYKjGkfCJqAF0xwZjipQtA3MxeSG87VhOOYegVrUCy7UZM9S6TLIdAamySTclZdYhFhRHloGYg7mgZv1Zzztvgud7V1tbQ2twYA34LJmF4p5dXF1KTufnE+SxeJtuCZNsLDCQU0+RyKTF27Unw101l8e6hns3u0PBalORVVVkcaEKBJDgV3+cGM4tKKmI+ohlIGnygKX00rSBfszz/n2uXv81wd6+rt1orsZCHRdr1Imk2F2Kob3hutSxW8thsd8AXNaln9D7CTfA6O+0UgkMuwVvEFFUbbAcrkcTA8+AtOk8E6KiQiDmMFSDqZItAzEVQviRkdDdaFgPp8HSZKAEAL5Qh7Sq2lIJBJwv2scUqkUnKoZgNhcDKhKg5aH+1IkcouCAdFGAQsuWZYhOjwFHQ96oagWgRoUov1T9kRBEODAwxM2QtEUl+Wp+Ln9VRo6BcMw4ErHRYjH4/B26AlQoQQTRdHWwcd9AH57+UAXddvDD37DmrBBV34WfqiXPl61g+vr6xA9zsGeM9gOdsNXkgpEtTwVvwOklXLKm6+/p5ezwk4B+j6droBs2CsGa/gNs6RIxazl4Tc25mpTgw/apPR1LYlNRFAzgsOxkyXYLIM1V8NMwyAkJSctD1eGVKiq5wWjSPdjmeTkiKvVW4f2YPHWl3GAVq6ymcyCTgovM3FzyRiDe2TaKcEKsLpJvNHjZgPNqEtyi6mZIm4SRFyLMUsONSSdkPeFtY1n0mczoY3BHTLhwPRy9/lzcziCw9ACI+yql0VLzcGAZbYSM5CCSZg1/9oc/nn7+i8N9p/8An4JMADxhH+xHfuiKwAAAABJRU5ErkJggg==);
}
.notifyjs-bootstrap-info {
	color: #3A87AD;
	background-color: #D9EDF7;
	border-color: #BCE8F1;
	background-image: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABQAAAAUCAYAAACNiR0NAAAABmJLR0QA/wD/AP+gvaeTAAAACXBIWXMAAAsTAAALEwEAmpwYAAAAB3RJTUUH3QYFAhkSsdes/QAAA8dJREFUOMvVlGtMW2UYx//POaWHXg6lLaW0ypAtw1UCgbniNOLcVOLmAjHZolOYlxmTGXVZdAnRfXQm+7SoU4mXaOaiZsEpC9FkiQs6Z6bdCnNYruM6KNBw6YWewzl9z+sHImEWv+vz7XmT95f/+3/+7wP814v+efDOV3/SoX3lHAA+6ODeUFfMfjOWMADgdk+eEKz0pF7aQdMAcOKLLjrcVMVX3xdWN29/GhYP7SvnP0cWfS8caSkfHZsPE9Fgnt02JNutQ0QYHB2dDz9/pKX8QjjuO9xUxd/66HdxTeCHZ3rojQObGQBcuNjfplkD3b19Y/6MrimSaKgSMmpGU5WevmE/swa6Oy73tQHA0Rdr2Mmv/6A1n9w9suQ7097Z9lM4FlTgTDrzZTu4StXVfpiI48rVcUDM5cmEksrFnHxfpTtU/3BFQzCQF/2bYVoNbH7zmItbSoMj40JSzmMyX5qDvriA7QdrIIpA+3cdsMpu0nXI8cV0MtKXCPZev+gCEM1S2NHPvWfP/hL+7FSr3+0p5RBEyhEN5JCKYr8XnASMT0xBNyzQGQeI8fjsGD39RMPk7se2bd5ZtTyoFYXftF6y37gx7NeUtJJOTFlAHDZLDuILU3j3+H5oOrD3yWbIztugaAzgnBKJuBLpGfQrS8wO4FZgV+c1IxaLgWVU0tMLEETCos4xMzEIv9cJXQcyagIwigDGwJgOAtHAwAhisQUjy0ORGERiELgG4iakkzo4MYAxcM5hAMi1WWG1yYCJIcMUaBkVRLdGeSU2995TLWzcUAzONJ7J6FBVBYIggMzmFbvdBV44Corg8vjhzC+EJEl8U1kJtgYrhCzgc/vvTwXKSib1paRFVRVORDAJAsw5FuTaJEhWM2SHB3mOAlhkNxwuLzeJsGwqWzf5TFNdKgtY5qHp6ZFf67Y/sAVadCaVY5YACDDb3Oi4NIjLnWMw2QthCBIsVhsUTU9tvXsjeq9+X1d75/KEs4LNOfcdf/+HthMnvwxOD0wmHaXr7ZItn2wuH2SnBzbZAbPJwpPx+VQuzcm7dgRCB57a1uBzUDRL4bfnI0RE0eaXd9W89mpjqHZnUI5Hh2l2dkZZUhOqpi2qSmpOmZ64Tuu9qlz/SEXo6MEHa3wOip46F1n7633eekV8ds8Wxjn37Wl63VVa+ej5oeEZ/82ZBETJjpJ1Rbij2D3Z/1trXUvLsblCK0XfOx0SX2kMsn9dX+d+7Kf6h8o4AIykuffjT8L20LU+w4AZd5VvEPY+XpWqLV327HR7DzXuDnD8r+ovkBehJ8i+y8YAAAAASUVORK5CYII=);
}
.notifyjs-bootstrap-warn {
	color: #C09853;
	background-color: #FCF8E3;
	border-color: #FBEED5;
	background-image: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABQAAAAUCAMAAAC6V+0/AAABJlBMVEXr6eb/2oD/wi7/xjr/0mP/ykf/tQD/vBj/3o7/uQ//vyL/twebhgD/4pzX1K3z8e349vK6tHCilCWbiQymn0jGworr6dXQza3HxcKkn1vWvV/5uRfk4dXZ1bD18+/52YebiAmyr5S9mhCzrWq5t6ufjRH54aLs0oS+qD751XqPhAybhwXsujG3sm+Zk0PTwG6Shg+PhhObhwOPgQL4zV2nlyrf27uLfgCPhRHu7OmLgAafkyiWkD3l49ibiAfTs0C+lgCniwD4sgDJxqOilzDWowWFfAH08uebig6qpFHBvH/aw26FfQTQzsvy8OyEfz20r3jAvaKbhgG9q0nc2LbZxXanoUu/u5WSggCtp1anpJKdmFz/zlX/1nGJiYmuq5Dx7+sAAADoPUZSAAAAAXRSTlMAQObYZgAAAAFiS0dEAIgFHUgAAAAJcEhZcwAACxMAAAsTAQCanBgAAAAHdElNRQfdBgUBGhh4aah5AAAAlklEQVQY02NgoBIIE8EUcwn1FkIXM1Tj5dDUQhPU502Mi7XXQxGz5uVIjGOJUUUW81HnYEyMi2HVcUOICQZzMMYmxrEyMylJwgUt5BljWRLjmJm4pI1hYp5SQLGYxDgmLnZOVxuooClIDKgXKMbN5ggV1ACLJcaBxNgcoiGCBiZwdWxOETBDrTyEFey0jYJ4eHjMGWgEAIpRFRCUt08qAAAAAElFTkSuQmCC);
}
.notifyjs-corner {
	position: fixed;
	margin: 5px;
	z-index: 1050;
}

.notifyjs-corner .notifyjs-wrapper,
.notifyjs-corner .notifyjs-container {
	position: relative;
	display: block;
	height: inherit;
	width: inherit;
	margin: 3px;
}

.notifyjs-wrapper {
	z-index: 1;
	position: absolute;
	display: inline-block;
	height: 0;
	width: 0;
}

.notifyjs-container {
	display: none;
	z-index: 1;
	position: absolute;
}

.notifyjs-hidable {
	cursor: pointer;
}

[data-notify-text],[data-notify-html] {
	position: relative;
}

.notifyjs-arrow {
	position: absolute;
	z-index: 2;
	width: 0;
	height: 0;
}.notifyjs-corner {
	position: fixed;
	margin: 5px;
	z-index: 1050;
}

.notifyjs-corner .notifyjs-wrapper,
.notifyjs-corner .notifyjs-container {
	position: relative;
	display: block;
	height: inherit;
	width: inherit;
	margin: 3px;
}

.notifyjs-wrapper {
	z-index: 1;
	position: absolute;
	display: inline-block;
	height: 0;
	width: 0;
}

.notifyjs-container {
	display: none;
	z-index: 1;
	position: absolute;
}

.notifyjs-hidable {
	cursor: pointer;
}

[data-notify-text],[data-notify-html] {
	position: relative;
}

.notifyjs-arrow {
	position: absolute;
	z-index: 2;
	width: 0;
	height: 0;
}#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-elementInfoDiv {color: lightblue; padding: 5px}


    
        
            
                
                    
                        
                    
                   
                    

                          
                        Hi  Sanjeev
                    


                    
                    


                    
                

            
        
    


    
        


                
                    Administrator
                
                

                       Home

                    
                        
                             Your Setup 
                            
                                
                                    Your Payroll 
                                    
                                        Payroll Basic Details
                                        Tax Groups
                                        Pay Groups
                                    
                                
                                
                                    Your Pay &amp; Benefits 
                                    
                                        Allowances
                                        Deduction
                                        Pension
                                        PensionRules
                                        Company Pay Rates
                                        MinimumRates
                                        
                                        
                                            Lookups 
                                            
                                                SalaryChangeReason
                                                Currencies
                                                Bank Indemnity
                                            
                                        
                                    
                                
                            
                        
                    


                    
                        
                              Data Migration
                            
                            
                        
                    

                    
                        
                              Reconciliation
                            
                            
                        
                    

                           Admin

                
                    

                    
                    Logout
                
        
    



    
        


 
table#example {
  width: 100%;
}


table#Payroll {
  width: 100%;
}

tr#Payroll{

width: 100px;

}

  .container {
  padding-right: 15px;
  padding-left: 15px;
  margin-right: 15px;
  margin-left: 15px;
}

    @media (min-width: 1200px){
.container {
    width: 98%;
}}

    



    
        Client Setup
        Payroll Setup

    

    
        
            
                





$(document).ready(function() {
    $('#msgClient').fadeOut(10000);
});




 

 Add Customer















Show 102550100 entriesSearch:
    
        ActionRight Now ID *Customer Name *Bundle
    
    
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
    
                
                     Edit
                     Delete
                     Save
                     Cancel 
                

                
                    111112
                    
                    
                

                
                    SDWorx-Ouma
                    
                    
                

                 
                    Payroll
                   
                     Please Select
Payroll
Pay &amp; HR

                    
                


            
                
                     Edit
                     Delete
                     Save
                     Cancel 
                

                
                    123456
                    
                    
                

                
                    Sdworx-Ouma2
                    
                    
                

                 
                    Payroll
                   
                     Please Select
Payroll
Pay &amp; HR

                    
                


            
                
                     Edit
                     Delete
                     Save
                     Cancel 
                

                
                    111113
                    
                    
                

                
                    SDWorx-Sanjeev
                    
                    
                

                 
                    Payroll
                   
                     Please Select
Payroll
Pay &amp; HR

                    
                


            
                
                     Edit
                     Delete
                     Save
                     Cancel 
                

                
                    311116
                    
                    
                

                
                    TestAuto6
                    
                    
                

                 
                    Pay &amp; HR
                   
                     Please Select
Payroll
Pay &amp; HR

                    
                


            
                
                     Edit
                     Delete
                     Save
                     Cancel 
                

                
                    111111
                    
                    
                

                
                    SDWorx-Nadia
                    
                    
                

                 
                    Payroll
                   
                     Please Select
Payroll
Pay &amp; HR

                    
                


            
                
                     Edit
                     Delete
                     Save
                     Cancel 
                

                
                    311112
                    
                    
                

                
                    TestAuto1
                    
                    
                

                 
                    Payroll
                   
                     Please Select
Payroll
Pay &amp; HR

                    
                


            
                
                     Edit
                     Delete
                     Save
                     Cancel 
                

                
                    256315
                    
                    
                

                
                    SDWorx-QA
                    
                    
                

                 
                    Payroll
                   
                     Please Select
Payroll
Pay &amp; HR

                    
                


            
                
                     Edit
                     Delete
                     Save
                     Cancel 
                

                
                    311117
                    
                    
                

                
                    TestAuto7
                    
                    
                

                 
                    Payroll
                   
                     Please Select
Payroll
Pay &amp; HR

                    
                


            
                
                     Edit
                     Delete
                     Save
                     Cancel 
                

                
                    311111
                    
                    
                

                
                    TestAuto
                    
                    
                

                 
                    Payroll
                   
                     Please Select
Payroll
Pay &amp; HR

                    
                


            
                
                     Edit
                     Delete
                     Save
                     Cancel 
                

                
                    125896
                    
                    
                

                
                    SDWorx-Test
                    
                    
                

                 
                    Payroll
                   
                     Please Select
Payroll
Pay &amp; HR

                    
                


            
Showing 1 to 10 of 18 entriesPrevious12Next




     table.table tr th {
        color: #337ab7 !important;
        background-color: #f9f9f9 !important;
    }





    
        
            
                ×
                Remove Data
            
            
                
                    
                        
                            Are you sure you want to delete the data?
                            

                        
                    
                
                
                    Yes
                    No
                
            
        
    




















    $(document).ready(function () {
        $('#example').dataTable({
            &quot;columnDefs&quot;: [{
                &quot;targets&quot;: 0,
                &quot;orderable&quot;: false
            }]
        });

        $('.actionsort').removeClass(&quot;sorting_asc&quot;);

        $('#example_paginate').click('page', function () {
            $(&quot;#Pagenumber&quot;).val($(&quot;li.active a&quot;).text());
        });



    });






    function EditCancelClient(index) {
        debugger
        $('#ERightNowID' + index).text(&quot;&quot;);
        $('#EOrgName' + index).text(&quot;&quot;);
        $('#EBundle' + index).text(&quot;&quot;);
        $('.display-mode' + index).removeClass(&quot;hidden&quot;);
        $('.edit-mode' + index).addClass(&quot;hidden&quot;);
        $('#Update_' + index).addClass(&quot;hidden&quot;);
        $('#Cancel_' + index).addClass(&quot;hidden&quot;);
        $('#RightNowID' + index).addClass(&quot;hidden&quot;);
        $('#OrgName' + index).addClass(&quot;hidden&quot;);
        $('#Bundle' + index).addClass(&quot;hidden&quot;);
        $('.lblName' + index).removeClass(&quot;hidden&quot;);
    }

    function Cancel() {
        //debugger
        $('.alternate-row').remove();
        $(&quot;#responseMessage&quot;).html(&quot;&quot;);
        $(&quot;#responseError&quot;).html(&quot;&quot;);
    }

    function myFunctionClient(id) {
        debugger
        var PageNumber = $(&quot;#Pagenumber&quot;).val();

        $('.display-mode' + id).addClass(&quot;hidden&quot;);
        $('.edit-mode' + id).removeClass(&quot;hidden&quot;);
        $('#Update_' + id).removeClass(&quot;hidden&quot;);
        $('#Cancel_' + id).removeClass(&quot;hidden&quot;);
        $('#RightNowID' + id).removeClass(&quot;hidden&quot;);
        $('#OrgName' + id).removeClass(&quot;hidden&quot;);
        $('#Bundle' + id).removeClass(&quot;hidden&quot;);
        $('.lblName' + id).addClass(&quot;hidden&quot;);

    }

    function UpdateClient(id) {
        debugger
        $(&quot;#responseMessage&quot;).html(&quot;&quot;);
        var IsformValid = CheckformValidClient(id)
        if (IsformValid) {
            var obj = BuildUpdateModelClient(id);

            $.ajax({
                url: '/Admin/UpdateClient',
                data: JSON.stringify(obj),
                //data: $(form).serialize(obj),
                type: &quot;POST&quot;,
                contentType: &quot;application/json; charset=utf-8&quot;,
                dataType: &quot;html&quot;,
                success: function (data) {
                    $('#divclientsetup').html(data);
                },
                error: function (errormessage) {
                }
            });
        }
    };

    function CheckformValidClient(index) {
        debugger
        var validate = true;
        //Validate fields
        var regDate = /^(0?[1-9]|[12][0-9]|3[01])[\/\-](0?[1-9]|1[012])[\/\-]\d{4}$/;
        var regDecimal = /^\d*[.]?\d*$/;
        var regInt = /^\d+$/;
        var regChar = /^[a-zA-Z ]+$/;
        var allowOnlyNumeric = /^[0-9]+$/;
        var alphaNumeric = /^[A-Za-z0-9]+$/i;
        var alphaNum = /^[-\w\s]+$/;

        /*RightNow ID*/
        $('#RightNowID' + index).focus(function () {
            $('#ERightNowID' + index).text(&quot;&quot;);
        });

        if ($('#RightNowID' + index).val() == &quot;&quot;) {
            $('#ERightNowID' + index).text(&quot;RightNow ID is required&quot;);
            validate = false;
        }

        if ($('#RightNowID' + index).val().length != &quot;&quot; &amp;&amp; ($('#RightNowID' + index).val().length > 6)) {
            $('#ERightNowID' + index).text(&quot;RightNow ID should be 6 characters&quot;);
            validate = false;
        }

        if ($('#RightNowID' + index).val().length != &quot;&quot; &amp;&amp; ($('#RightNowID' + index).val().length &lt; 6)) {
            $('#ERightNowID' + index).text(&quot;RightNow ID should be 6 characters&quot;);
            validate = false;
        }

        if ($('#RightNowID' + index).val().length != &quot;&quot; &amp;&amp; !($('#RightNowID' + index).val()).match(allowOnlyNumeric)) {
            $('#ERightNowID' + index).text(&quot;RightNow ID should be numeric&quot;);
            validate = false;
        }

        /*Customer Name*/


        $('#OrgName' + index).focus(function () {
            $('#EOrgName' + index).text(&quot;&quot;);
        });

        if ($('#OrgName' + index).val() == &quot;&quot;) {
            $('#EOrgName' + index).text(&quot;Customer Name is required&quot;);
            validate = false;
        }

        if ($('#OrgName' + index).val().length != &quot;&quot; &amp;&amp; ($('#OrgName' + index).val().length > 100)) {
            $('#EOrgName' + index).text(&quot;Customer Name should not exceed 100 characters&quot;);
            validate = false;
        }


        if (validate != false) { return true; }
    }

    function BuildUpdateModelClient(index) {
        debugger
        if (index != 0) {
            debugger
            var currentOrg = {
                &quot;OrgId&quot;: index,
                RightNowID: $(&quot;#RightNowID&quot; + index).val(),
                OrgName: $(&quot;#OrgName&quot; + index).val(),
                Bundle: $(&quot;#Bundle&quot; + index).val()
            }
        }
        else {
            var currentOrg = {
                RightNowID: $(&quot;#RightNowID&quot; + index).val(),
                OrgName: $(&quot;#OrgName&quot; + index).val(),
                Bundle: $(&quot;#Bundle&quot; + index).val()
            }
        }
        return currentOrg;

    }

    function DeleteClient(id) {
        debugger
        //$('#FileUpload1').focus(function () {
        //    $(&quot;#responseMessage&quot;).html(&quot;&quot;);
        //    $(&quot;#responseError&quot;).html(&quot;&quot;);
        //});
        //Get Values from fields
        var s = document.getElementById('AllID');
        s.value = id
    }



    function DeleteByIDClient() {
        debugger
        //Get Values from fields
        var id = document.getElementById('AllID');
        $.ajax({
            url: '/Admin/Delete/' + id.value,
            data: JSON.stringify(id.value),
            type: &quot;POST&quot;,
            contentType: &quot;application/json; charset=utf-8&quot;,
            dataType: &quot;html&quot;,
            success: function (data) {

                $('#RemoveClient').modal('hide'); //Hide the modal
                $('body').removeClass('modal-open');
                $('.modal-backdrop').remove();

                $('#divclientsetup').html(data);


            },
            //Throw error
            error: function (errormessage) {
                alert(errormessage.responseText);
            }
        });

    }

    function Add() {
        debugger
        var existrow = $('.save-user1').length;
        if (existrow == 0) {
            var index = 0;
            var RightNowID = &quot;RightNowID&quot; + index;
            var OrgName = &quot;OrgName&quot; + index;
            var Bundle = &quot;Bundle&quot; + index;

            var tr =
                '&lt;tr class=&quot;alternate-row&quot;>' +
                '&lt;td>&lt;button type=&quot;submit&quot; onclick=&quot;SaveClient()&quot; class=&quot;save-user1 edit-mode btn btn-primary btn-sm&quot;>&lt;i class=&quot;fa fa-floppy-o&quot; aria-hidden=&quot;true&quot;>&lt;/i> Save&lt;/button> &lt;button type=&quot;button&quot; class=&quot;cancel-user edit-mode btn btn-warning btn-sm&quot;  onclick=&quot;Cancel()&quot;>&lt;i class=&quot;fa fa-times&quot; >&lt;/i> Cancel &lt;/button>&lt;/td>' +
                '&lt;td>&lt;input id=&quot;' + RightNowID + '&quot; type=&quot;text&quot; class=&quot;form-control&quot;  />&lt;br/>&lt;span id=&quot;ERightNowID0&quot; style=&quot;color:red;&quot;>&lt;/span>&lt;/td>' +
                '&lt;td>&lt;input id=&quot;' + OrgName + '&quot; type=&quot;text&quot; class=&quot;form-control&quot;  />&lt;br/>&lt;span id=&quot;EOrgName0&quot; style=&quot;color:red;&quot;>&lt;/span>&lt;/td>' +
                    '&lt;td>&lt;span>&lt;select id=&quot;' + Bundle + '&quot; class=&quot;form-control&quot;> &lt;option value=&quot;&quot;>Please Select&lt;/option>&lt;option value=&quot;_1&quot;>Payroll&lt;/option>  &lt;option value=&quot;_2&quot;>Pay &amp; HR&lt;/option> &lt;/select>&lt;/span>&lt;/td>' +
                '&lt;/tr>';

            $(&quot;#example tbody&quot;).prepend(tr);

        }

        else {
            $(&quot;#responseMessage&quot;).html('&lt;div class=&quot;alert alert-warning fade in&quot;> &lt;a href=&quot;#&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot;>&amp;times;&lt;/a>&lt;strong>Warning!&lt;/strong> Please, &lt;a href=&quot;#&quot; class=&quot;alert-link&quot;>save&lt;/a> your previous record before proceeding. &lt;/div>')
        }


    };

    function SaveClient() {
        debugger
        $(&quot;#responseMessage&quot;).html(&quot;&quot;);
        id = 0;
        var IsformValid = CheckformValidClient(id)
        if (IsformValid) {
            var obj = BuildUpdateModelClient(id);

            $.ajax(
                {
                    url: '/Admin/CreateClient',
                    data: JSON.stringify(obj),
                    type: &quot;POST&quot;,
                    contentType: &quot;application/json; charset=utf-8&quot;,
                    dataType: &quot;html&quot;,
                    success: function (data) {
                        $('#divclientsetup').html(data);
                    },
                    //Throw error
                    error: function (errormessage) {
                    }
                });
        }

    }















 
            
        


        
            
                



    /*
    table.dataTable thead th, table.dataTable thead td {
    padding: 0px 0px !important;
    }*/

    .form-control {
    width: 100% !important;
}






$(document).ready(function() {
    $('#msgPayroll').fadeOut(10000);
});







    
         Customer : 
        
        Please Select
SDWorx-Ouma
Sdworx-Ouma2
SDWorx-Sanjeev
TestAuto6
SDWorx-Nadia
TestAuto1
SDWorx-QA
TestAuto7
TestAuto
SDWorx-Test
SDWorx-Prasan
311114
311115
TestAuto2
SDWorx-Nischala
Nadia Test
SDWorx-Dev
SDWorx-Steve

 
         
               
        

 
 Add Payroll















Show 102550100 entriesSearch:
    
        ActionCustomer *Payroll Number *Payroll Name *Pensioner Payroll?Default Payroll?Contract Audit NumberActual date Client goes LivePaper payslips required?First Live Tax Year StartManaged?Exclude from EOY Processing?
    
     Save  Cancel Please select111112-SDWorx-Ouma123456-Sdworx-Ouma2111113-SDWorx-Sanjeev311116-TestAuto6111111-SDWorx-Nadia311112-TestAuto1256315-SDWorx-QA311117-TestAuto7311111-TestAuto125896-SDWorx-Test111115-SDWorx-Prasan311114-311114311115-311115311113-TestAuto2100000-SDWorx-Nischala888888-Nadia Test111116-SDWorx-Dev111118-SDWorx-Steve Please SelectYes  No  Please SelectYes  No   Please SelectYes  No   Please SelectYes  No  Please SelectYes  No 
                
                
                
                
                
                
                
                
                
                
                
    


                    
                         Edit
                         Delete
                         Save
                         Cancel 
                    

                
                    111112 - SDWorx-Ouma
                    
                    
                

                
                    9988
                    
                    
                

                
                    Wickes
                    
                    
                

                
                    Yes
                    Please Select
Yes
No

                

                
                    Yes
                    Please Select
Yes
No


                

                
                    
                    
                    
                



                
                    02/08/2018
                    

                
 




                
                    Yes
                    Please Select
Yes
No


                

                
                    01/06/2018  
                    

                

                
                    Yes
                    Please Select
Yes
No


                

                
                    No
                    Please Select
Yes
No

                


            


                    
                         Edit
                         Delete
                         Save
                         Cancel 
                    

                
                    111112 - SDWorx-Ouma
                    
                    
                

                
                    123456
                    
                    
                

                
                    Web service test
                    
                    
                

                
                    No
                    Please Select
Yes
No

                

                
                    No
                    Please Select
Yes
No


                

                
                    
                    
                    
                



                
                    23/05/2018
                    

                
 




                
                    Yes
                    Please Select
Yes
No


                

                
                    05/04/2018  
                    

                

                
                    No
                    Please Select
Yes
No


                

                
                    Yes
                    Please Select
Yes
No

                


            


                    
                         Edit
                         Delete
                         Save
                         Cancel 
                    

                
                    111113 - SDWorx-Sanjeev
                    
                    
                

                
                    771
                    
                    
                

                
                    Test Autmation
                    
                    
                

                
                    Yes
                    Please Select
Yes
No

                

                
                    No
                    Please Select
Yes
No


                

                
                    63
                    
                    
                



                
                    19/03/2018
                    

                
 




                
                    No
                    Please Select
Yes
No


                

                
                    19/03/2018  
                    

                

                
                    Yes
                    Please Select
Yes
No


                

                
                    No
                    Please Select
Yes
No

                


            


                    
                         Edit
                         Delete
                         Save
                         Cancel 
                    

                
                    111113 - SDWorx-Sanjeev
                    
                    
                

                
                    777
                    
                    
                

                
                    Test
                    
                    
                

                
                    No
                    Please Select
Yes
No

                

                
                    No
                    Please Select
Yes
No


                

                
                    64
                    
                    
                



                
                    19/03/2018
                    

                
 




                
                    Yes
                    Please Select
Yes
No


                

                
                    19/03/2018  
                    

                

                
                    No
                    Please Select
Yes
No


                

                
                    Yes
                    Please Select
Yes
No

                


            


                    
                         Edit
                         Delete
                         Save
                         Cancel 
                    

                
                    111113 - SDWorx-Sanjeev
                    
                    
                

                
                    888
                    
                    
                

                
                    Auto
                    
                    
                

                
                    No
                    Please Select
Yes
No

                

                
                    No
                    Please Select
Yes
No


                

                
                    65
                    
                    
                



                
                    19/03/2018
                    

                
 




                
                    No
                    Please Select
Yes
No


                

                
                    16/03/2018  
                    

                

                
                    Yes
                    Please Select
Yes
No


                

                
                    Yes
                    Please Select
Yes
No

                


            


                    
                         Edit
                         Delete
                         Save
                         Cancel 
                    

                
                    125896 - SDWorx-Test
                    
                    
                

                
                    012587
                    
                    
                

                
                    test
                    
                    
                

                
                    
                    Please Select
Yes
No

                

                
                    
                    Please Select
Yes
No


                

                
                    
                    
                    
                



                
                    30/06/2018
                    

                
 




                
                    
                    Please Select
Yes
No


                

                
                      
                    

                

                
                    
                    Please Select
Yes
No


                

                
                    
                    Please Select
Yes
No

                


            


                    
                         Edit
                         Delete
                         Save
                         Cancel 
                    

                
                    111115 - SDWorx-Prasan
                    
                    
                

                
                    123001
                    
                    
                

                
                    test 1
                    
                    
                

                
                    
                    Please Select
Yes
No

                

                
                    
                    Please Select
Yes
No


                

                
                    
                    
                    
                



                
                    
                    

                
 




                
                    
                    Please Select
Yes
No


                

                
                      
                    

                

                
                    
                    Please Select
Yes
No


                

                
                    
                    Please Select
Yes
No

                


            


                    
                         Edit
                         Delete
                         Save
                         Cancel 
                    

                
                    100000 - SDWorx-Nischala
                    
                    
                

                
                    123000
                    
                    
                

                
                    Lorem test payroll
                    
                    
                

                
                    Yes
                    Please Select
Yes
No

                

                
                    Yes
                    Please Select
Yes
No


                

                
                    
                    
                    
                



                
                    
                    

                
 




                
                    
                    Please Select
Yes
No


                

                
                      
                    

                

                
                    
                    Please Select
Yes
No


                

                
                    
                    Please Select
Yes
No

                


            


                    
                         Edit
                         Delete
                         Save
                         Cancel 
                    

                
                    111116 - SDWorx-Dev
                    
                    
                

                
                    000001
                    
                    
                

                
                    Dev Payroll 1
                    
                    
                

                
                    
                    Please Select
Yes
No

                

                
                    
                    Please Select
Yes
No


                

                
                    
                    
                    
                



                
                    
                    

                
 




                
                    
                    Please Select
Yes
No


                

                
                      
                    

                

                
                    
                    Please Select
Yes
No


                

                
                    
                    Please Select
Yes
No

                


            


                    
                         Edit
                         Delete
                         Save
                         Cancel 
                    

                
                    111116 - SDWorx-Dev
                    
                    
                

                
                    9966
                    
                    
                

                
                    WebTest
                    
                    
                

                
                    Yes
                    Please Select
Yes
No

                

                
                    Yes
                    Please Select
Yes
No


                

                
                    
                    
                    
                



                
                    22/08/2018
                    

                
 




                
                    Yes
                    Please Select
Yes
No


                

                
                    20/07/2018  
                    

                

                
                    Yes
                    Please Select
Yes
No


                

                
                    Yes
                    Please Select
Yes
No

                


            
Showing 1 to 10 of 11 entriesPrevious12Next

 


    table.table tr th {
        color: #337ab7 !important;
        background-color: #f9f9f9 !important;
    }





    
        
            
                ×
                Remove Data
            
            
                
                    
                        
                            Are you sure you want to delete the data?
                            

                        
                    
                
                
                    Yes
                    No
                
            
        
    







  











    $(document).ready(function () {
        debugger;
        $(this).datepicker();
        $('#cmd').click(function () {
            $('#content').append('&lt;br>a datepicker &lt;input class=&quot;datepicker_recurring_start&quot;/>');
        });
        $('body').on('focus', &quot;.datepicker_recurring_start&quot;, function () {
            $(this).datepicker();
        });
    });
























    $(document).ready(function () {
        $('#Payroll').dataTable({
            &quot;columnDefs&quot;: [{
                &quot;targets&quot;: 0,
                &quot;orderable&quot;: false
            }]
        });

        $('.actionsort').removeClass(&quot;sorting_asc&quot;);

        $('#Payroll_paginate').click('page', function () {
            $(&quot;#Pagenumber&quot;).val($(&quot;li.active a&quot;).text());
        });



    });


    function EditCancel(index) {
        debugger

        $('#EOrgName' + index).text(&quot;&quot;);
        $('#EPayrollCompanyNo' + index).text(&quot;&quot;);
        $('#EPayrollCompanyName' + index).text(&quot;&quot;);
        $('#EPensionerPayroll' + index).text(&quot;&quot;);
        $('#EDefaultPayroll' + index).text(&quot;&quot;);
        $('#EContractAuditNo' + index).text(&quot;&quot;);
        $('#EActualGoLive' + index).text(&quot;&quot;);
        $('#EPaperPayslip' + index).text(&quot;&quot;);
        $('#EFirstLiveTaxYear' + index).text(&quot;&quot;);
        $('#EManaged' + index).text(&quot;&quot;);
        $('#EExcludeFromEOYProcessing' + index).text(&quot;&quot;);

        $('.display-mode' + index).removeClass(&quot;hidden&quot;);
        $('.edit-mode' + index).addClass(&quot;hidden&quot;);
        $('#Update_' + index).addClass(&quot;hidden&quot;);
        $('#Cancel_' + index).addClass(&quot;hidden&quot;);

        $('#OrgName' + index).addClass(&quot;hidden&quot;);
        $('#PayrollCompanyNo' + index).addClass(&quot;hidden&quot;);
        $('#PayrollCompanyName' + index).addClass(&quot;hidden&quot;);
        $('#PensionerPayroll' + index).addClass(&quot;hidden&quot;);
        $('#DefaultPayroll' + index).addClass(&quot;hidden&quot;);
        $('#ContractAuditNo' + index).addClass(&quot;hidden&quot;);
        $('#ActualGoLive' + index).addClass(&quot;hidden&quot;);
        $('#PaperPayslip' + index).addClass(&quot;hidden&quot;);
        $('#FirstLiveTaxYear' + index).addClass(&quot;hidden&quot;);
        $('#Managed' + index).addClass(&quot;hidden&quot;);
        $('#ExcludeFromEOYProcessing' + index).addClass(&quot;hidden&quot;);

        $('.lblName' + index).removeClass(&quot;hidden&quot;);
    }


    function Cancel() {
        //debugger
        $('.alternate-row').remove();
        $(&quot;#responseMessage&quot;).html(&quot;&quot;);
        $(&quot;#responseError&quot;).html(&quot;&quot;);
    }


    function myFunction(id) {
        debugger
        var PageNumber = $(&quot;#Pagenumber&quot;).val();

        $('.display-mode' + id).addClass(&quot;hidden&quot;);
        $('.edit-mode' + id).removeClass(&quot;hidden&quot;);
        $('#Update_' + id).removeClass(&quot;hidden&quot;);
        $('#Cancel_' + id).removeClass(&quot;hidden&quot;);

        $('#OrgName' + id).removeClass(&quot;hidden&quot;);
        $('#PayrollCompanyNo' + id).removeClass(&quot;hidden&quot;);
        $('#PayrollCompanyName' + id).removeClass(&quot;hidden&quot;);
        $('#PensionerPayroll' + id).removeClass(&quot;hidden&quot;);
        $('#DefaultPayroll' + id).removeClass(&quot;hidden&quot;);
        $('#ContractAuditNo' + id).removeClass(&quot;hidden&quot;);
        $('#ActualGoLive' + id).removeClass(&quot;hidden&quot;);
        $('#PaperPayslip' + id).removeClass(&quot;hidden&quot;);
        $('#FirstLiveTaxYear' + id).removeClass(&quot;hidden&quot;);
        $('#Managed' + id).removeClass(&quot;hidden&quot;);
        $('#ExcludeFromEOYProcessing' + id).removeClass(&quot;hidden&quot;);

        $('.lblName' + id).addClass(&quot;hidden&quot;);

    }


    function Update(id) {
        debugger
        $(&quot;#responseMessage&quot;).html(&quot;&quot;);
        var IsformValid = CheckformValid(id)
        if (IsformValid) {
            var obj = BuildUpdateModel(id);

            $.ajax({
                url: '/Admin/UpdatePayroll',
                data: JSON.stringify(obj),
                //data: $(form).serialize(obj),
                type: &quot;POST&quot;,
                contentType: &quot;application/json; charset=utf-8&quot;,
                dataType: &quot;html&quot;,
                success: function (data) {
                    $('#divPayrollsetup').html(data);
                },
                error: function (errormessage) {
                }
            });
        }
    };


    function CheckformValid(index) {
        debugger
        var validate = true;
        //Validate fields
        var regDate = /^(0?[1-9]|[12][0-9]|3[01])[\/\-](0?[1-9]|1[012])[\/\-]\d{4}$/;
        var regDecimal = /^\d*[.]?\d*$/;
        var regInt = /^\d+$/;
        var regChar = /^[a-zA-Z ]+$/;
        var allowOnlyNumeric = /^[0-9]+$/;
        var alphaNumeric = /^[A-Za-z0-9]+$/i;
        var alphaNum = /^[-\w\s]+$/;

        /*Payroll Number*/
        $('#PayrollCompanyNo' + index).focus(function () {
            $('#EPayrollCompanyNo' + index).text(&quot;&quot;);
        });

        if ($('#PayrollCompanyNo' + index).val() == &quot;&quot;) {
            $('#EPayrollCompanyNo' + index).text(&quot;Data is required&quot;);
            validate = false;
        }

        if ($('#PayrollCompanyNo' + index).val().length != &quot;&quot; &amp;&amp; ($('#PayrollCompanyNo' + index).val().length > 6)) {
            $('#EPayrollCompanyNo' + index).text(&quot;Data should be 6 characters&quot;);
            validate = false;
        }


        if ($('#PayrollCompanyNo' + index).val().length != &quot;&quot; &amp;&amp; !($('#PayrollCompanyNo' + index).val()).match(allowOnlyNumeric)) {
            $('#EPayrollCompanyNo' + index).text(&quot;Data should be numeric&quot;);
            validate = false;
        }

        /* Payroll Name*/
        $('#PayrollCompanyName' + index).focus(function () {
            $('#EPayrollCompanyName' + index).text(&quot;&quot;);
        });

        if ($('#PayrollCompanyName' + index).val() == &quot;&quot;) {
            $('#EPayrollCompanyName' + index).text(&quot;Data is required&quot;);
            validate = false;
        }

        if ($('#PayrollCompanyName' + index).val().length != &quot;&quot; &amp;&amp; ($('#PayrollCompanyName' + index).val().length > 100)) {
            $('#EPayrollCompanyName' + index).text(&quot;Data should be 100 characters&quot;);
            validate = false;
        }


        /*Customer Name*/
        $('#OrgName' + index).focus(function () {
            $('#EOrgName' + index).text(&quot;&quot;);
        });

        if ($('#OrgName' + index).val() == &quot;&quot;) {
            $('#EOrgName' + index).text(&quot;Please select customer&quot;);
            validate = false;
        }


        /*Contract Audit No*/
        $('#ContractAuditNo' + index).focus(function () {
            $('#EContractAuditNo' + index).text(&quot;&quot;);
        });

        if ($('#ContractAuditNo' + index).val().length != &quot;&quot; &amp;&amp; ($('#ContractAuditNo' + index).val().length > 6)) {
            $('#EContractAuditNo' + index).text(&quot;Data should be 6 characters&quot;);
            validate = false;
        }


        if ($('#ContractAuditNo' + index).val().length != &quot;&quot; &amp;&amp; !($('#ContractAuditNo' + index).val()).match(alphaNumeric)) {
            $('#EContractAuditNo' + index).text(&quot;Data should be alphanumeric&quot;);
            validate = false;
        }

        if (validate != false) { return true; }
    }


    function BuildUpdateModel(index) {
        debugger
        if (index != 0) {
            debugger
            var currentPayroll = {
                &quot;Id&quot;: index,
                &quot;OrgId&quot;: $(&quot;#OrgName&quot; + index).val(),
                PayrollCompanyNo: $(&quot;#PayrollCompanyNo&quot; + index).val(),
                PayrollCompanyName: $(&quot;#PayrollCompanyName&quot; + index).val(),
                IsPensionerPayroll: $(&quot;#PensionerPayroll&quot; + index).val(),
                IsDefaultPayroll: $(&quot;#DefaultPayroll&quot; + index).val(),
                ContractAuditNo: $(&quot;#ContractAuditNo&quot; + index).val(),
                ActualGoLive: $(&quot;#ActualGoLive&quot; + index).val(),
                IsPaperPayslip: $(&quot;#PaperPayslip&quot; + index).val(),
                FirstLiveTaxYear: $(&quot;#FirstLiveTaxYear&quot; + index).val(),
                IsManaged: $(&quot;#Managed&quot; + index).val(),
                IsExcludeFromEOYProcessing: $(&quot;#ExcludeFromEOYProcessing&quot; + index).val()
            }
        }
        else {
            var currentPayroll = {
                &quot;OrgId&quot;: $(&quot;#OrgName&quot; + index).val(),
                PayrollCompanyNo: $(&quot;#PayrollCompanyNo&quot; + index).val(),
                PayrollCompanyName: $(&quot;#PayrollCompanyName&quot; + index).val(),
                IsPensionerPayroll: $(&quot;#PensionerPayroll&quot; + index).val(),
                IsDefaultPayroll: $(&quot;#DefaultPayroll&quot; + index).val(),
                ContractAuditNo: $(&quot;#ContractAuditNo&quot; + index).val(),
                ActualGoLive: $(&quot;#ActualGoLive&quot; + index).val(),
                IsPaperPayslip: $(&quot;#PaperPayslip&quot; + index).val(),
                FirstLiveTaxYear: $(&quot;#FirstLiveTaxYear&quot; + index).val(),
                IsManaged: $(&quot;#Managed&quot; + index).val(),
                IsExcludeFromEOYProcessing: $(&quot;#ExcludeFromEOYProcessing&quot; + index).val()
            }
        }
        return currentPayroll;

    }


    function Delete(id) {
        debugger
        //$('#FileUpload1').focus(function () {
        //    $(&quot;#responseMessage&quot;).html(&quot;&quot;);
        //    $(&quot;#responseError&quot;).html(&quot;&quot;);
        //});
        //Get Values from fields
        var s = document.getElementById('AllID');
        s.value = id
    }


    function DeleteByID() {
        debugger
        //Get Values from fields
        var id = document.getElementById('AllID');
        $.ajax({
            url: '/Admin/DeletePayroll/' + id.value,
            data: JSON.stringify(id.value),
            type: &quot;POST&quot;,
            contentType: &quot;application/json; charset=utf-8&quot;,
            dataType: &quot;html&quot;,
            success: function (data) {

                $('#RemoveUser').modal('hide'); //Hide the modal
                $('body').removeClass('modal-open');
                $('.modal-backdrop').remove();
                $('#divPayrollsetup').html(data);

            },
            //Throw error
            error: function (errormessage) {
                alert(errormessage.responseText);
            }
        });

    }


    function SavePayroll(orgname) {
        debugger
        $(&quot;#responseMessage&quot;).html(&quot;&quot;);
        id = 0;
        var IsformValid = CheckformValid(id)
        if (IsformValid) {
            var obj = BuildUpdateModel(id);

            $.ajax(
                {
                    url: '/Admin/CreatePayroll'   ,
                    data: JSON.stringify(obj),
                    type: &quot;POST&quot;,
                    contentType: &quot;application/json; charset=utf-8&quot;,
                    dataType: &quot;html&quot;,
                    success: function (data) {
                        $('#divPayrollsetup').html(data);
                    },
                    //Throw error
                    error: function (errormessage) {
                    }
                });
        }

    }


    function AddPayroll()
    {
        var existrow = $('.save-user1').length;
        if (existrow == 0) {
            var index = 0;
            var OrgName = &quot;OrgName&quot; + index;
            var PayrollCompanyNo = &quot;PayrollCompanyNo&quot; + index;
            var PayrollCompanyName = &quot;PayrollCompanyName&quot; + index;
            var PensionerPayroll = &quot;PensionerPayroll&quot; + index;
            var DefaultPayroll = &quot;DefaultPayroll&quot; + index;
            var ContractAuditNo = &quot;ContractAuditNo&quot; + index;
            var ActualGoLive = &quot;ActualGoLive&quot; + index;
            var PaperPayslip = &quot;PaperPayslip&quot; + index;
            var FirstLiveTaxYear = &quot;FirstLiveTaxYear&quot; + index;
            var Managed = &quot;Managed&quot; + index;
            var ExcludeFromEOYProcessing = &quot;ExcludeFromEOYProcessing&quot; + index;


            var tr =
                '&lt;tr class=&quot;alternate-row&quot;>' +
                '&lt;td>&lt;button type=&quot;submit&quot; onclick=&quot;SavePayroll()&quot; class=&quot;save-user1 edit-mode btn btn-primary btn-sm&quot;>&lt;i class=&quot;fa fa-floppy-o&quot; aria-hidden=&quot;true&quot;>&lt;/i> Save&lt;/button> &lt;button type=&quot;button&quot; class=&quot;cancel-user edit-mode btn btn-warning btn-sm&quot;  onclick=&quot;Cancel()&quot;>&lt;i class=&quot;fa fa-times&quot; >&lt;/i> Cancel &lt;/button>&lt;/td>' +
                '&lt;td>&lt;span>&lt;select id=&quot;' + OrgName + '&quot; class=&quot;form-control&quot;>&lt;/span>&lt;/select >&lt;br/>&lt;span id=&quot;EOrgName0&quot; style=&quot;color:red;&quot;>&lt;/span>&lt;/td>' +
                '&lt;td>&lt;input id=&quot;' + PayrollCompanyNo + '&quot; type=&quot;text&quot; class=&quot;form-control&quot;  />&lt;br/>&lt;span id=&quot;EPayrollCompanyNo0&quot; style=&quot;color:red;&quot;>&lt;/span>&lt;/td>' +
                '&lt;td>&lt;input id=&quot;' + PayrollCompanyName + '&quot; type=&quot;text&quot; class=&quot;form-control&quot;  />&lt;br/>&lt;span id=&quot;EPayrollCompanyName0&quot; style=&quot;color:red;&quot;>&lt;/span>&lt;/td>' +
                '&lt;td>&lt;span>&lt;select id=&quot;' + PensionerPayroll + '&quot; class=&quot;form-control&quot;> &lt;option value=&quot;&quot;>Please Select&lt;/option>&lt;option value=&quot;True&quot;>Yes&lt;/option>  &lt;option value=&quot;False&quot;>No&lt;/option> &lt;/select>&lt;/span>&lt;/td>' +
                '&lt;td>&lt;span>&lt;select id=&quot;' + DefaultPayroll + '&quot; class=&quot;form-control&quot;> &lt;option value=&quot;&quot;>Please Select&lt;/option>&lt;option value=&quot;True&quot;>Yes&lt;/option>  &lt;option value=&quot;False&quot;>No&lt;/option> &lt;/select>&lt;/span>&lt;/td>' +
                '&lt;td>&lt;input id=&quot;' + ContractAuditNo + '&quot; type=&quot;text&quot; class=&quot;form-control&quot;  />&lt;br/>&lt;span id=&quot;EContractAuditNo0&quot; style=&quot;color:red;&quot;>&lt;/span>&lt;/td>' +

                '&lt;td>&lt;input class=&quot;form-control datepicker_recurring_start&quot; id=&quot;' + ActualGoLive + '&quot; name=&quot;date&quot; placeholder=&quot;DD/MM/YYYY&quot; type=&quot;text&quot; data-date-format=&quot;dd/mm/yyyy&quot;> &lt;br/>&lt;span id=&quot;EActualGoLive0&quot; style=&quot;color:red;&quot;>&lt;/span>&lt;/td>' +

               '&lt;td>&lt;span>&lt;select id=&quot;' + PaperPayslip + '&quot; class=&quot;form-control&quot;> &lt;option value=&quot;&quot;>Please Select&lt;/option>&lt;option value=&quot;True&quot;>Yes&lt;/option>  &lt;option value=&quot;False&quot;>No&lt;/option> &lt;/select>&lt;/span>&lt;/td>' +

               '&lt;td>&lt;input class=&quot;form-control datepicker_recurring_start&quot; id=&quot;' + FirstLiveTaxYear + '&quot; name=&quot;date&quot; placeholder=&quot;DD/MM/YYYY&quot; type=&quot;text&quot; data-date-format=&quot;dd/mm/yyyy&quot;> &lt;br/>&lt;span id=&quot;EFirstLiveTaxYear0&quot; style=&quot;color:red;&quot;>&lt;/span>&lt;/td>' +


               '&lt;td>&lt;span>&lt;select id=&quot;' + Managed + '&quot; class=&quot;form-control&quot;> &lt;option value=&quot;&quot;>Please Select&lt;/option>&lt;option value=&quot;True&quot;>Yes&lt;/option>  &lt;option value=&quot;False&quot;>No&lt;/option> &lt;/select>&lt;/span>&lt;/td>' +


               '&lt;td>&lt;span>&lt;select id=&quot;' + ExcludeFromEOYProcessing + '&quot; class=&quot;form-control&quot;> &lt;option value=&quot;&quot;>Please Select&lt;/option>&lt;option value=&quot;True&quot;>Yes&lt;/option>  &lt;option value=&quot;False&quot;>No&lt;/option> &lt;/select>&lt;/span>&lt;/td>' +
                '&lt;/tr>';

            $(&quot;#Payroll tbody&quot;).prepend(tr);
            debugger
            var OrgName = $(&quot;#OrgName0&quot;);
            OrgName.empty().append('&lt;option selected=&quot;selected&quot; value=&quot;0&quot; disabled = &quot;disabled&quot;>Loading.....&lt;/option>');

            $.ajax({
                type: &quot;POST&quot;,
                url: &quot;/Admin/AddCustomer&quot;,
                data: '{}',
                contentType: &quot;application/json; charset=utf-8&quot;,
                dataType: &quot;json&quot;,
                success: function (response) {
                    OrgName.empty().append('&lt;option selected=&quot;selected&quot; value=&quot;&quot;>Please select&lt;/option>');
                    $.each(response, function () {
                        OrgName.append($(&quot;&lt;option>&lt;/option>&quot;).val(this['Value']).html(this['Text']));
                    });

                },
                failure: function (response) {
                    alert(response.responseText);
                },
                error: function (response) {
                    alert(response.responseText);
                }
            });

        }

        else {
            $(&quot;#responseMessage&quot;).html('&lt;div class=&quot;alert alert-warning fade in&quot;> &lt;a href=&quot;#&quot; class=&quot;close&quot; data-dismiss=&quot;alert&quot;>&amp;times;&lt;/a>&lt;strong>Warning!&lt;/strong> Please, &lt;a href=&quot;#&quot; class=&quot;alert-link&quot;>save&lt;/a> your previous record before proceeding. &lt;/div>')
        }


    };


    function changeOrganisation(value) {
       debugger
        var value = $(&quot;#OrganisationList&quot;).val();
 //alert(value);
        $.ajax({
            url: '/Admin/changeOrganisation',
            type: &quot;POST&quot;,
            data: { id: value },
 
                dataType: 'html',
                success: function (data) {
                $('#divPayrollsetup').html(data);
               // $('#RemoveUser').html();
                
            }
           
        });
   
    }







 
            
        
     

 








 

       
        

    
   



    
    
    
    








function chat() {
     $('#chatbotDiv').show();
        };


function removeIFrame() {
     $('#chatbotDiv').hide();
}



$(document).scroll(function () {
     var y = $(this).scrollTop();
     if (y > 80) {
         $('.fixHead').fadeIn();

     } else {
         $('.fixHead').fadeOut();
     }
});





    function removeCookie() {
        $.cookie('AccountInfo', '');
    }

    
/html[@class=&quot;js flexbox flexboxlegacy canvas canvastext webgl no-touch geolocation postmessage websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms no-csstransforms3d csstransitions fontface no-generatedcontent video audio localstorage sessionstorage webworkers applicationcache svg inlinesvg smil svgclippaths&quot;]«March 2018»SuMoTuWeThFrSa25262728123456789101112131415161718192021222324252627282930311234567TodayClear«2018»JanFebMarAprMayJunJulAugSepOctNovDecTodayClear«2010-2019»200920102011201220132014201520162017201820192020TodayClear</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;js flexbox flexboxlegacy canvas canvastext webgl no-touch geolocation postmessage websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms no-csstransforms3d csstransitions fontface no-generatedcontent video audio localstorage sessionstorage webworkers applicationcache svg inlinesvg smil svgclippaths&quot;]</value>
   </webElementProperties>
</WebElementEntity>
