<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Make ancor tag unclickable</name>
   <tag></tag>
   <elementGuidId>b1d1103f-8ced-49e1-bbb5-b84c107b7b04</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>container body-content</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
        





    /*Make ancor tag unclickable*/
     .inactiveLink {
         pointer-events: none;
         cursor: default;
     }

    select#PayrollCompany_Location {
        width: 230% !important;
    }

    select#PayrollCompany_Frequency {
        width: 211% !important;
    }

    .form-control {
        width: 170% !important;
        border-radius: 4px !important;
    }

    /*FOR FILTER*/
    /*select#PayrollCompanyList {
        margin-left: 105%;
        margin-top: -21%;
    }*/

    #required {
        text-align: left;
    }

    .col-md-4.control-label {
        text-align: left;
    }


    /*label {
        display: inline;
    }*/

    #required {
        margin-top: -16px;
    }


    body {
        font-family: Tahoma,Verdana,Segoe,sans-serif;
    }

    span.field-validation-error.errorMessage {
        color: #f94141;
    }

    body {
        font-size: 17px;
    }

    span.field-validation-error.text-danger {
        margin-left: 34%;
    }

    #required:after {
        content: &quot;*&quot;;
        color: red;
        margin-left: 10px;
        font-size: 25px;
    }

    .stepwizard-step p {
        margin-top: 10px;
    }

    .stepwizard-row {
        display: table-row;
    }

    .stepwizard {
        display: table;
        width: 1500px;
        position: relative;
        margin-top: 20px; 
        margin-left: -15%;
    }

    /*.stepwizard {
        display: table;
        width: 100%;
        position: relative;
        margin-top: 20px;
    }*/

    .stepwizard-step button[disabled] {
        opacity: 1 !important;
        filter: alpha(opacity=100) !important;
    }

    .stepwizard-row:before {
        top: 22px;
        bottom: 0;
        position: absolute;
        content: &quot; &quot;;
        width: 100%;
        height: 3px;
        background-color: #ccc;
        z-order: 0;
    }

    .stepwizard-step {
        display: table-cell;
        text-align: center;
        position: relative;
    }

    .btn-circle {
        width: 50px;
        height: 50px;
        text-align: center;
        padding: 6px 0;
        font-size: 18px;
        line-height: 1.8;
        border-radius: 30px;
        border: 4px solid #ccc;
        font-weight: bold;
    }

    .btn-circle-small {
        width: 12px;
        height: 12px !important;
        text-align: center;
        padding: 6px 0;
        font-size: 18px;
        line-height: 1.8;
        border-radius: 20px;
        background-color: #ccc;
    }

    .btn-circle-small-right {
        width: 12px;
        height: 12px !important;
        text-align: center;
        padding: 6px 0;
        font-size: 18px;
        line-height: 1.8;
        border-radius: 20px;
        background-color: #ccc;
        float: right !important
    }

    .adv_blue {
        background-color: #0069aa;
    }

    .adv_orange {
        background-color: #f8981d;
        color: #fff;
    }

    .adv_grey {
        background-color: #fff;
        color: #ccc;
    }

    @media (min-width: 1200px) {
        .posHelpText {
            width: 50%;
            margin-left:735px;
        }
    }

    @media (min-width: 360px) {
        .posHelpText {
            width: 50%;
            margin-left: 735px;
        }
    }




    function displaySuccess(data) {
        //$('#divCompany').hide();
        $('#step-1').html(data);
    }

    $(document).ready(function () {
        var navListItems = $('div.setup-panel div a'),
            allWells = $('.setup-content'),
            allNextBtn = $('.nextBtn');

        allWells.hide();

        navListItems.click(function (e) {
            e.preventDefault();
            var $target = $($(this).attr('href')),
                $item = $(this);

            if (!$item.hasClass('disabled')) {
                navListItems.removeClass('btn-primary').addClass('btn-default');
                $item.addClass('btn-primary');
                allWells.hide();
                $target.show();
                $target.find('input:eq(0)').focus();
            }
        });

        allNextBtn.click(function () {
            var curStep = $(this).closest(&quot;.setup-content&quot;),
                curStepBtn = curStep.attr(&quot;id&quot;),
                nextStepWizard = $('div.setup-panel div a[href=&quot;#' + curStepBtn + '&quot;]').parent().next().children(&quot;a&quot;),
                curInputs = curStep.find(&quot;input[type='text'],input[type='url']&quot;),
                isValid = true;

            $(&quot;.form-group&quot;).removeClass(&quot;has-error&quot;);
            for (var i = 0; i &lt; curInputs.length; i++) {
                if (!curInputs[i].validity.valid) {
                    isValid = false;
                    $(curInputs[i]).closest(&quot;.form-group&quot;).addClass(&quot;has-error&quot;);
                }
            }

            if (isValid)
                nextStepWizard.removeAttr('disabled').trigger('click');
        });

        $('div.setup-panel div a.btn-primary').trigger('click');
    });




    
        
             Payroll Number: 
            test
Dev Payroll 1
WebTest


        
    
    


    
        
        

        
            1
            Step 1
        
        
            2
            Step 2
        
        
            3
            Step 3
        
        
            4
            Step 4
        
        
            5
            Step 5
        

        
    




   


    label {
        display: inline;
    }



    
        
        
        

            
                What is the frequency of this payroll? :
                
                    
                        Please select
Annual
Fortnightly
Hourly
Lunar
Monthly
Quarterly
Weekly

                    
                    
                
                
                    This will apply to all employees / pensioners on the payroll, it is not possible to pay two different frequencies on one payroll.
                
            
            

            
                What is the registered address of this payroll :
                
                    
                        
                    
                

                
                    Enter the primary address to be used by SDWorx for communications regarding this payroll.
                

            

            
                  
                
                    
                        
                    
                
                
                    
                
            

            
                  
                
                    
                        
                    
                

                
                    
                
            
            
            
                Town/City :
                
                    
                        
                    
                
                
                    
                
                     
            
            
                County :
                
                    
                        
                    
                
            

            
                Country :
                
                    
                        
                    
                
            
            
            
                Post Code :
                
                    
                        
                       
                
            
            

            
                Enter the primary telephone number to be used by SDWorx for communications regarding this payroll (including STD code) :
                
                    
                        
                    
                    
                
            
            
            
                Location :
                
                    
                        
                    

                
            
            

            
                What statutory fiscal authority does this payroll belong to? :
                
                    
                        UK
Jersey
Guernsey
Gibraltar
Isle of Man

                    

                

                
                    This determines what statutory rules (PAYE, NI, SSP etc) will be applied to all employees paid on this payroll
                
            
            

            
                
                
                    Next 
                
            
               
    




       
        

    </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;js flexbox flexboxlegacy canvas canvastext webgl no-touch geolocation postmessage websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms csstransforms3d csstransitions fontface no-generatedcontent video audio localstorage sessionstorage webworkers applicationcache svg inlinesvg smil svgclippaths&quot;]/body[1]/div[@class=&quot;container body-content&quot;]</value>
   </webElementProperties>
</WebElementEntity>
