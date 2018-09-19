<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Allowance Details        E</name>
   <tag></tag>
   <elementGuidId>884c7de2-5469-4a2b-a30f-9807ca1885ab</elementGuidId>
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
      <value>container</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
        Allowance Details
        Earnings Accumulators
        Hours Accumulators
    

    
    
        
            Allowance Details
            
            
                Please list all gross pay elements that are used on your payroll. &quot;Hours&quot; entry allowances can be created from allowance codes 006 - 099. &quot;Cash&quot; entry codes can be created from allowances 100 - 998.
                Codes highlighted in bold blue are restricted or reserved codes that must remain under the existing allowance code. Parameters can be amended for these codes however (Tax, NI, Pension settings for example).These entries should not be removed.
                Where the clear down calendar is not specified, it will be assumed as tax year end clear down.
            
            
            
                
                     Payroll Number: 
                    
                        000001 -SD Worx Mauritius
                        000002 -SD Worx Ireland
                    
                
            
            
                 Set Status to:   Not Required
In Progress
Completed
 
            
            
             Add Allowance
            
            

       

            
            
            
            
                










       th:first-of-type {
        display: none !important;
    } 


 th.action {
    display: inline-grid !important;
}

    table#grid {
        height: 400px;
    }

    table {
        border-collapse: collapse;
        background: white;
        table-layout: fixed;
        width: 100%;
    }

    th, td {
        padding: 8px 16px;
        border: 1px solid #ddd;
        width: 160px;
        overflow: hidden;
        /*text-overflow: ellipsis;*/
        /*white-space: nowrap;*/
    }

    .pane {
        background: #eee;
    }

    .pane-hScroll {
        overflow: auto;
        width: 100%;
        /*background: green*/
    }

    .pane-vScroll {
        overflow-y: auto;
        /*overflow-x: hidden;*/
        /*height: 200px;*/
        /*background: red;*/
        width: 5760px;
    
    }

    .pane--table2 {
        width: 400px;
        overflow-x: scroll;
    }

        .pane--table2 th, .pane--table2 td {
            width: auto;
            min-width: 160px;
        }

        .pane--table2 tbody {
            overflow-y: scroll;
            overflow-x: hidden;
            display: block;
            height: 200px;
        }

        .pane--table2 thead {
            display: table-row;
        }


    /*.header thead tr th {
        display: none;
    }*/


    table#grid thead {
        display: none;
    }


    /*Align Header and body column*/
    .action {
        width: 159px;
    }


    .tooltip-9 {
        width: 158px;
    }

    th.tooltip-AlloSDesc {
        width: 158px;
    }

    th.tooltip-AlloLDesc {
        width: 158px;
    }

    .tooltip-Tax {
        width: 159.5px;
    }

    .tooltip-Pens {
        width: 159px;
    }


    .tooltip-Nble {
        width: 159px;
    }

    .tooltip-PensionReform {
        width: 159px;
    }

    .tooltip-Benefits {
        width: 159px;
    }

    .tooltip-ONS {
        width: 160px;
    }

    th.tooltip-Hours {
        width: 160px;
    }

    th.tooltip-NetGross {
        width: 159px;
    }

    th.tooltip-Salary {
        width: 159px;
    }

    th.tooltip-PAY {
        width: 159px;
    }

    th.tooltip-print {
        width: 159px;
    }

    th.tooltip-printBal {
        width: 159px;
    }

    th.tooltip-APS {
        width: 159px;
    }

    th.tooltip-APL {
        width: 159px;
    }











    $(document).ready(function () {
        $(&quot;th:first&quot;).next().addClass(&quot;tooltip-9&quot;);
        $(&quot;th:first&quot;).next().next().addClass(&quot;tooltip-AlloSDesc&quot;);
        $(&quot;th:first&quot;).next().next().next().addClass(&quot;tooltip-AlloLDesc&quot;);
        $(&quot;th:first&quot;).next().next().next().next().addClass(&quot;tooltip-Tax&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().addClass(&quot;tooltip-Pens&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().addClass(&quot;tooltip-Nble&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().addClass(&quot;tooltip-PensionReform&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().addClass(&quot;tooltip-Benefits&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-ONS&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-Cash&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-Hours&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-Minimum&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-NetGross&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-Calcu&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-Salary&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-PAY&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-print&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-printBal&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-APS&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-APL&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-PT&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-IT&quot;);


        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-PV&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-PA&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-PBH&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-CE&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-FE&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-PDH&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-PS&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-CP&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-SW&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-LLO&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-LLV&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-ULO&quot;);
        $(&quot;th:first&quot;).next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().next().addClass(&quot;tooltip-ULV&quot;);
        //$(&quot;th:first&quot;).next().attr(&quot;title&quot;, &quot;Numeric 3 digit field between 001 &amp; 898 to identify the allowance in the Payroll engine&quot;);

    });


    //FOR ALLOWANCE NUMBER
    $(function () {
        $('.tooltip-9').hover(function () {
            $('.tooltipTest').css('visibility', 'visible');
            $('.tooltipAlloNum').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-9').mouseout(function () {
            $('.tooltipTest').css('visibility', 'hidden');
            $('.tooltipAlloNum').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-9 > a').hover(function () {
            $('.tooltipTest').css('visibility', 'visible');
            $('.tooltipAlloNum').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-9 > a').mouseout(function () {
            $('.tooltipTest').css('visibility', 'hidden');
            $('.tooltipAlloNum').css('visibility', 'hidden');
        });
    });
    //END ALLOWANCE NUMBER

    //--------

    //FOR ALLOWANCE SHORT DESC
    $(function () {
        $('.tooltip-AlloSDesc').hover(function () {
            $('.tooltipAlloHead').css('visibility', 'visible');
            $('.tooltipAlloShort').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-AlloSDesc').mouseout(function () {
            $('.tooltipAlloHead').css('visibility', 'hidden');
            $('.tooltipAlloShort').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-AlloSDesc > a').hover(function () {
            $('.tooltipAlloHead').css('visibility', 'visible');
            $('.tooltipAlloShort').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-AlloSDesc > a').mouseout(function () {
            $('.tooltipAlloHead').css('visibility', 'hidden');
            $('.tooltipAlloShort').css('visibility', 'hidden');
        });
    });
    //END FOR ALLOWANCE SHORT DESC

    //--------


    //FOR ALLOWANCE Long DESC
    $(function () {
        $('.tooltip-AlloLDesc').hover(function () {
            $('.tooltipAlloLongHead').css('visibility', 'visible');
            $('.tooltipAlloLong').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-AlloLDesc').mouseout(function () {
            $('.tooltipAlloLongHead').css('visibility', 'hidden');
            $('.tooltipAlloLong').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-AlloLDesc > a').hover(function () {
            $('.tooltipAlloLongHead').css('visibility', 'visible');
            $('.tooltipAlloLong').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-AlloLDesc > a').mouseout(function () {
            $('.tooltipAlloLongHead').css('visibility', 'hidden');
            $('.tooltipAlloLong').css('visibility', 'hidden');
        });
    });
    //END FOR ALLOWANCE Long DESC

    //--------


    //FOR Taxable
    $(function () {
        $('.tooltip-Tax').hover(function () {
            $('.tooltipTaxableHead').css('visibility', 'visible');
            $('.tooltipTaxable').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-Tax').mouseout(function () {
            $('.tooltipTaxableHead').css('visibility', 'hidden');
            $('.tooltipTaxable').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-Tax > a').hover(function () {
            $('.tooltipTaxableHead').css('visibility', 'visible');
            $('.tooltipTaxable').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-Tax > a').mouseout(function () {
            $('.tooltipTaxableHead').css('visibility', 'hidden');
            $('.tooltipTaxable').css('visibility', 'hidden');
        });
    });
    //END FOR Taxable

    //--------


    //FOR Pensionable
    $(function () {
        $('.tooltip-Pens').hover(function () {
            $('.tooltipPensionHead').css('visibility', 'visible');
            $('.tooltipPension').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-Pens').mouseout(function () {
            $('.tooltipPensionHead').css('visibility', 'hidden');
            $('.tooltipPension').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-Pens > a').hover(function () {
            $('.tooltipPensionHead').css('visibility', 'visible');
            $('.tooltipPension').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-Pens > a').mouseout(function () {
            $('.tooltipPensionHead').css('visibility', 'hidden');
            $('.tooltipPension').css('visibility', 'hidden');
        });
    });
    //END FOR Pensionable

    //--------


    //FOR Nble
    $(function () {
        $('.tooltip-Nble').hover(function () {
            $('.tooltipNbleHead').css('visibility', 'visible');
            $('.tooltipNble').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-Nble').mouseout(function () {
            $('.tooltipNbleHead').css('visibility', 'hidden');
            $('.tooltipNble').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-Nble > a').hover(function () {
            $('.tooltipNbleHead').css('visibility', 'visible');
            $('.tooltipNble').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-Nble > a').mouseout(function () {
            $('.tooltipNbleHead').css('visibility', 'hidden');
            $('.tooltipNble').css('visibility', 'hidden');
        });
    });
    //END FOR Nble

    //--------



    //FOR PensionReform
    $(function () {
        $('.tooltip-PensionReform').hover(function () {
            $('.tooltipPensionRefHead').css('visibility', 'visible');
            $('.tooltipPensionRef').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-PensionReform').mouseout(function () {
            $('.tooltipPensionRefHead').css('visibility', 'hidden');
            $('.tooltipPensionRef').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-PensionReform > a').hover(function () {
            $('.tooltipPensionRefHead').css('visibility', 'visible');
            $('.tooltipPensionRef').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-PensionReform > a').mouseout(function () {
            $('.tooltipPensionRefHead').css('visibility', 'hidden');
            $('.tooltipPensionRef').css('visibility', 'hidden');
        });
    });
    //END FOR PensionReform

    //--------


    //FOR Benefits
    $(function () {
        $('.tooltip-Benefits').hover(function () {
            $('.tooltipBenefitsHead').css('visibility', 'visible');
            $('.tooltipBenefits').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-Benefits').mouseout(function () {
            $('.tooltipBenefitsHead').css('visibility', 'hidden');
            $('.tooltipBenefits').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-Benefits > a').hover(function () {
            $('.tooltipBenefitsHead').css('visibility', 'visible');
            $('.tooltipBenefits').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-Benefits > a').mouseout(function () {
            $('.tooltipBenefitsHead').css('visibility', 'hidden');
            $('.tooltipBenefits').css('visibility', 'hidden');
        });
    });
    //END FOR Benefits

    //--------


    //FOR ONS
    $(function () {
        $('.tooltip-ONS').hover(function () {
            $('.tooltipONSHead').css('visibility', 'visible');
            $('.tooltipONS').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-ONS').mouseout(function () {
            $('.tooltipONSHead').css('visibility', 'hidden');
            $('.tooltipONS').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-ONS > a').hover(function () {
            $('.tooltipONSHead').css('visibility', 'visible');
            $('.tooltipONS').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-ONS > a').mouseout(function () {
            $('.tooltipONSHead').css('visibility', 'hidden');
            $('.tooltipONS').css('visibility', 'hidden');
        });
    });
    //END FOR ONS

    //--------


    //FOR Cash
    $(function () {
        $('.tooltip-Cash').hover(function () {
            $('.tooltipCashHead').css('visibility', 'visible');
            $('.tooltipCash').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-Cash').mouseout(function () {
            $('.tooltipCashHead').css('visibility', 'hidden');
            $('.tooltipCash').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-Cash > a').hover(function () {
            $('.tooltipCashHead').css('visibility', 'visible');
            $('.tooltipCash').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-Cash > a').mouseout(function () {
            $('.tooltipCashHead').css('visibility', 'hidden');
            $('.tooltipCash').css('visibility', 'hidden');
        });
    });
    //END FOR Cash

    //--------


    //FOR Hours
    $(function () {
        $('.tooltip-Hours').hover(function () {
            $('.tooltipHoursHead').css('visibility', 'visible');
            $('.tooltipHours').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-Hours').mouseout(function () {
            $('.tooltipHoursHead').css('visibility', 'hidden');
            $('.tooltipHours').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-Hours > a').hover(function () {
            $('.tooltipHoursHead').css('visibility', 'visible');
            $('.tooltipHours').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-Hours > a').mouseout(function () {
            $('.tooltipHoursHead').css('visibility', 'hidden');
            $('.tooltipHours').css('visibility', 'hidden');
        });
    });
    //END FOR Hours

    //--------

    //FOR Minimum
    $(function () {
        $('.tooltip-Minimum').hover(function () {
            $('.tooltipMinimumHead').css('visibility', 'visible');
            $('.tooltipMinimum').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-Minimum').mouseout(function () {
            $('.tooltipMinimumHead').css('visibility', 'hidden');
            $('.tooltipMinimum').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-Minimum > a').hover(function () {
            $('.tooltipMinimumHead').css('visibility', 'visible');
            $('.tooltipMinimum').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-Minimum > a').mouseout(function () {
            $('.tooltipMinimumHead').css('visibility', 'hidden');
            $('.tooltipMinimum').css('visibility', 'hidden');
        });
    });
    //END FOR Minimum

    //--------



    //FOR NetGross
    $(function () {
        $('.tooltip-NetGross').hover(function () {
            $('.tooltipNetGrossHead').css('visibility', 'visible');
            $('.tooltipNetGross').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-NetGross').mouseout(function () {
            $('.tooltipNetGrossHead').css('visibility', 'hidden');
            $('.tooltipNetGross').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-NetGross > a').hover(function () {
            $('.tooltipNetGrossHead').css('visibility', 'visible');
            $('.tooltipNetGross').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-NetGross > a').mouseout(function () {
            $('.tooltipNetGrossHead').css('visibility', 'hidden');
            $('.tooltipNetGross').css('visibility', 'hidden');
        });
    });
    //END FOR NetGross

    //--------


    //FOR Calcu
    $(function () {
        $('.tooltip-Calcu').hover(function () {
            $('.tooltipCalcuHead').css('visibility', 'visible');
            $('.tooltipCalcu').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-Calcu').mouseout(function () {
            $('.tooltipCalcuHead').css('visibility', 'hidden');
            $('.tooltipCalcu').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-Calcu > a').hover(function () {
            $('.tooltipCalcuHead').css('visibility', 'visible');
            $('.tooltipCalcu').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-Calcu > a').mouseout(function () {
            $('.tooltipCalcuHead').css('visibility', 'hidden');
            $('.tooltipCalcu').css('visibility', 'hidden');
        });
    });
    //END FOR Calcu

    //--------

    //FOR Salary
    $(function () {
        $('.tooltip-Salary').hover(function () {
            $('.tooltipSalaryHead').css('visibility', 'visible');
            $('.tooltipSalary').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-Salary').mouseout(function () {
            $('.tooltipSalaryHead').css('visibility', 'hidden');
            $('.tooltipSalary').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-Salary > a').hover(function () {
            $('.tooltipSalaryHead').css('visibility', 'visible');
            $('.tooltipSalary').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-Salary > a').mouseout(function () {
            $('.tooltipSalaryHead').css('visibility', 'hidden');
            $('.tooltipSalary').css('visibility', 'hidden');
        });
    });
    //END FOR Salary

    //--------

    //FOR PAY
    $(function () {
        $('.tooltip-PAY').hover(function () {
            $('.tooltipPAYHead').css('visibility', 'visible');
            $('.tooltipPAY').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-PAY').mouseout(function () {
            $('.tooltipPAYHead').css('visibility', 'hidden');
            $('.tooltipPAY').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-PAY > a').hover(function () {
            $('.tooltipPAYHead').css('visibility', 'visible');
            $('.tooltipPAY').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-PAY > a').mouseout(function () {
            $('.tooltipPAYHead').css('visibility', 'hidden');
            $('.tooltipPAY').css('visibility', 'hidden');
        });




    });
    //END FOR PAY

    //--------


    //$(function () {
    //    var tr = $('#grid').find('tr');
    //    tr.bind('click', function (event) {
    //        var values = '';
    //        var tds = $(this).find('td');
    //FOR print
    $(function () {
        $('.tooltip-print').hover(function () {
            $('.tooltipprintHead').css('visibility', 'visible');
            $('.tooltipprint').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-print').mouseout(function () {
            $('.tooltipprintHead').css('visibility', 'hidden');
            $('.tooltipprint').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-print > a').hover(function () {
            $('.tooltipprintHead').css('visibility', 'visible');
            $('.tooltipprint').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-print > a').mouseout(function () {
            $('.tooltipprintHead').css('visibility', 'hidden');
            $('.tooltipprint').css('visibility', 'hidden');
        });
    });
    //END FOR print

    //--------

    //FOR Print balance
    $(function () {
        $('.tooltip-printBal').hover(function () {
            $('.tooltipprintBalHead').css('visibility', 'visible');
            $('.tooltipprintBal').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-printBal').mouseout(function () {
            $('.tooltipprintBalHead').css('visibility', 'hidden');
            $('.tooltipprintBal').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-printBal > a').hover(function () {
            $('.tooltipprintBalHead').css('visibility', 'visible');
            $('.tooltipprintBal').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-printBal > a').mouseout(function () {
            $('.tooltipprintBalHead').css('visibility', 'hidden');
            $('.tooltipprintBal').css('visibility', 'hidden');
        });
    });
    //END FOR Print balance

    //--------

    //FOR Print APS
    $(function () {
        $('.tooltip-APS').hover(function () {
            $('.tooltipAPSHead').css('visibility', 'visible');
            $('.tooltipAPS').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-APS').mouseout(function () {
            $('.tooltipAPSHead').css('visibility', 'hidden');
            $('.tooltipAPS').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-APS > a').hover(function () {
            $('.tooltipAPSHead').css('visibility', 'visible');
            $('.tooltipAPS').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-APS > a').mouseout(function () {
            $('.tooltipAPSHead').css('visibility', 'hidden');
            $('.tooltipAPS').css('visibility', 'hidden');
        });
    });
    //END FOR APS

    //--------

    //FOR Print APL
    $(function () {
        $('.tooltip-APL').hover(function () {
            $('.tooltipAPLHead').css('visibility', 'visible');
            $('.tooltipAPL').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-APL').mouseout(function () {
            $('.tooltipAPLHead').css('visibility', 'hidden');
            $('.tooltipAPL').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-APL > a').hover(function () {
            $('.tooltipAPLHead').css('visibility', 'visible');
            $('.tooltipAPL').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-APL > a').mouseout(function () {
            $('.tooltipAPLHead').css('visibility', 'hidden');
            $('.tooltipAPL').css('visibility', 'hidden');
        });
    });
    //END FOR APL

    //--------


    //FOR Print PT
    $(function () {
        $('.tooltip-PT').hover(function () {
            $('.tooltipPTHead').css('visibility', 'visible');
            $('.tooltipPT').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-PT').mouseout(function () {
            $('.tooltipPTHead').css('visibility', 'hidden');
            $('.tooltipPT').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-PT > a').hover(function () {
            $('.tooltipPTHead').css('visibility', 'visible');
            $('.tooltipPT').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-PT > a').mouseout(function () {
            $('.tooltipPTHead').css('visibility', 'hidden');
            $('.tooltipPT').css('visibility', 'hidden');
        });
    });
    //END FOR PT

    //--------

    //FOR Print IT
    $(function () {
        $('.tooltip-IT').hover(function () {
            $('.tooltipITHead').css('visibility', 'visible');
            $('.tooltipIT').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-IT').mouseout(function () {
            $('.tooltipITHead').css('visibility', 'hidden');
            $('.tooltipIT').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-IT > a').hover(function () {
            $('.tooltipITHead').css('visibility', 'visible');
            $('.tooltipIT').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-IT > a').mouseout(function () {
            $('.tooltipITHead').css('visibility', 'hidden');
            $('.tooltipIT').css('visibility', 'hidden');
        });
    });
    //END FOR IT

    //FOR Print PV
    $(function () {
        $('.tooltip-PV').hover(function () {
            $('.tooltipPVHead').css('visibility', 'visible');
            $('.tooltipPV').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-PV').mouseout(function () {
            $('.tooltipPVHead').css('visibility', 'hidden');
            $('.tooltipPV').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-PV > a').hover(function () {
            $('.tooltipPVHead').css('visibility', 'visible');
            $('.tooltipPV').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-PV > a').mouseout(function () {
            $('.tooltipPVHead').css('visibility', 'hidden');
            $('.tooltipPV').css('visibility', 'hidden');
        });
    });
    //END FOR PV

    //FOR Print PA
    $(function () {
        $('.tooltip-PA').hover(function () {
            $('.tooltipPAHead').css('visibility', 'visible');
            $('.tooltipPA').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-PA').mouseout(function () {
            $('.tooltipPAHead').css('visibility', 'hidden');
            $('.tooltipPA').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-PA > a').hover(function () {
            $('.tooltipPAHead').css('visibility', 'visible');
            $('.tooltipPA').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-PA > a').mouseout(function () {
            $('.tooltipPAHead').css('visibility', 'hidden');
            $('.tooltipPA').css('visibility', 'hidden');
        });
    });
    //END FOR PA

    //FOR Print PBH
    $(function () {
        $('.tooltip-PBH').hover(function () {
            $('.tooltipPBHPAHead').css('visibility', 'visible');
            $('.tooltipPBH').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-PBH').mouseout(function () {
            $('.tooltipPBHHead').css('visibility', 'hidden');
            $('.tooltipPBH').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-PBH > a').hover(function () {
            $('.tooltipPBHHead').css('visibility', 'visible');
            $('.tooltipPBH').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-PBH > a').mouseout(function () {
            $('.tooltipPBHHead').css('visibility', 'hidden');
            $('.tooltipPBH').css('visibility', 'hidden');
        });
    });
    //END FOR PBH



    //FOR Print CE
    $(function () {
        $('.tooltip-CE').hover(function () {
            $('.tooltipCEHead').css('visibility', 'visible');
            $('.tooltipCE').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-CE').mouseout(function () {
            $('.tooltipCEHead').css('visibility', 'hidden');
            $('.tooltipCE').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-CE > a').hover(function () {
            $('.tooltipCEHead').css('visibility', 'visible');
            $('.tooltipCE').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-CE > a').mouseout(function () {
            $('.tooltipCEHead').css('visibility', 'hidden');
            $('.tooltipCE').css('visibility', 'hidden');
        });
    });
    //END FOR CE

    //FOR Print FE
    $(function () {
        $('.tooltip-FE').hover(function () {
            $('.tooltipFEHead').css('visibility', 'visible');
            $('.tooltipFE').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-FE').mouseout(function () {
            $('.tooltipFEHead').css('visibility', 'hidden');
            $('.tooltipFE').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-FE > a').hover(function () {
            $('.tooltipFEHead').css('visibility', 'visible');
            $('.tooltipFE').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-FE > a').mouseout(function () {
            $('.tooltipFEHead').css('visibility', 'hidden');
            $('.tooltipFE').css('visibility', 'hidden');
        });
    });
    //END FOR FE
    //FOR Print PDH
    $(function () {
        $('.tooltip-PDH').hover(function () {
            $('.tooltipPDHHead').css('visibility', 'visible');
            $('.tooltipPDH').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-PDH').mouseout(function () {
            $('.tooltipPDHHead').css('visibility', 'hidden');
            $('.tooltipPDH').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-PDH > a').hover(function () {
            $('.tooltipPDHHead').css('visibility', 'visible');
            $('.tooltipPDH').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-PDH > a').mouseout(function () {
            $('.tooltipPDHHead').css('visibility', 'hidden');
            $('.tooltipPDH').css('visibility', 'hidden');
        });
    });
    //END FOR PDH

    //FOR Print PS
    $(function () {
        $('.tooltip-PS').hover(function () {
            $('.tooltipPSHead').css('visibility', 'visible');
            $('.tooltipPS').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-PS').mouseout(function () {
            $('.tooltipPSHead').css('visibility', 'hidden');
            $('.tooltipPS').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-PS > a').hover(function () {
            $('.tooltipPSHead').css('visibility', 'visible');
            $('.tooltipPS').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-PS > a').mouseout(function () {
            $('.tooltipPSHead').css('visibility', 'hidden');
            $('.tooltipPS').css('visibility', 'hidden');
        });
    });
    //END FOR PS

    //FOR Print CP
    $(function () {
        $('.tooltip-CP').hover(function () {
            $('.tooltipCPHead').css('visibility', 'visible');
            $('.tooltipCP').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-CP').mouseout(function () {
            $('.tooltipCPHead').css('visibility', 'hidden');
            $('.tooltipCP').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-CP > a').hover(function () {
            $('.tooltipCPHead').css('visibility', 'visible');
            $('.tooltipCP').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-CP > a').mouseout(function () {
            $('.tooltipCPHead').css('visibility', 'hidden');
            $('.tooltipCP').css('visibility', 'hidden');
        });
    });
    //END FOR CP

    //FOR Print SW
    $(function () {
        $('.tooltip-SW').hover(function () {
            $('.tooltipSWHead').css('visibility', 'visible');
            $('.tooltipSW').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-SW').mouseout(function () {
            $('.tooltipSWHead').css('visibility', 'hidden');
            $('.tooltipSW').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-SW > a').hover(function () {
            $('.tooltipSWHead').css('visibility', 'visible');
            $('.tooltipSW').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-SW > a').mouseout(function () {
            $('.tooltipSWHead').css('visibility', 'hidden');
            $('.tooltipSW').css('visibility', 'hidden');
        });
    });
    //END FOR SW

    //FOR Print LLO
    $(function () {
        $('.tooltip-LLO').hover(function () {
            $('.tooltipLLOHead').css('visibility', 'visible');
            $('.tooltipLLO').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-LLO').mouseout(function () {
            $('.tooltipLLOHead').css('visibility', 'hidden');
            $('.tooltipLLO').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-LLO > a').hover(function () {
            $('.tooltipLLOHead').css('visibility', 'visible');
            $('.tooltipLLO').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-LLO > a').mouseout(function () {
            $('.tooltipLLOHead').css('visibility', 'hidden');
            $('.tooltipLLO').css('visibility', 'hidden');
        });
    });
    //END FOR LLO

    //FOR Print LLV
    $(function () {
        $('.tooltip-LLV').hover(function () {
            $('.tooltipLLVHead').css('visibility', 'visible');
            $('.tooltipLLV').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-LLV').mouseout(function () {
            $('.tooltipLLVHead').css('visibility', 'hidden');
            $('.tooltipLLV').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-LLV > a').hover(function () {
            $('.tooltipLLVHead').css('visibility', 'visible');
            $('.tooltipLLV').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-LLV > a').mouseout(function () {
            $('.tooltipLLVHead').css('visibility', 'hidden');
            $('.tooltipLLV').css('visibility', 'hidden');
        });
    });
    //END FOR LLV
    //FOR Print ULO
    $(function () {
        $('.tooltip-ULO').hover(function () {
            $('.tooltipULOHead').css('visibility', 'visible');
            $('.tooltipULO').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-ULO').mouseout(function () {
            $('.tooltipULOHead').css('visibility', 'hidden');
            $('.tooltipULO').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-ULO > a').hover(function () {
            $('.tooltipULOHead').css('visibility', 'visible');
            $('.tooltipULO').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-ULO > a').mouseout(function () {
            $('.tooltipULOHead').css('visibility', 'hidden');
            $('.tooltipULO').css('visibility', 'hidden');
        });
    });
    //END FOR ULO

    //FOR Print ULV
    $(function () {
        $('.tooltip-ULV').hover(function () {
            $('.tooltipULVHead').css('visibility', 'visible');
            $('.tooltipULV').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-ULV').mouseout(function () {
            $('.tooltipULVHead').css('visibility', 'hidden');
            $('.tooltipULV').css('visibility', 'hidden');
        });
    });

    $(function () {
        $('.tooltip-ULV > a').hover(function () {
            $('.tooltipULVHead').css('visibility', 'visible');
            $('.tooltipULV').css('visibility', 'visible');
        });
    });

    $(function () {
        $('.tooltip-ULV > a').mouseout(function () {
            $('.tooltipULVHead').css('visibility', 'hidden');
            $('.tooltipULV').css('visibility', 'hidden');
        });
    });
    //END FOR ULV

    function initialize() {
        $('.edit-mode').hide();
    };

    $(function () {
        $('th a').on('click', function () {
            $(this).attr('href', '#grid-anchor');
        });
    });
    //--------


    function sortTable(n) {
        var table, rows, switching, i, x, y, shouldSwitch, dir, switchcount = 0;
        table = document.getElementById(&quot;grid&quot;);
        switching = true;
        //Set the sorting direction to ascending:
        dir = &quot;asc&quot;;
        /*Make a loop that will continue until
        no switching has been done:*/
        while (switching) {
            //start by saying: no switching is done:
            switching = false;
            rows = table.getElementsByTagName(&quot;TR&quot;);
            /*Loop through all table rows (except the
            first, which contains table headers):*/
            for (i = 1; i &lt; (rows.length - 1); i++) {
                //start by saying there should be no switching:
                shouldSwitch = false;
                /*Get the two elements you want to compare,
                one from current row and one from the next:*/
                x = rows[i].getElementsByTagName(&quot;TD&quot;)[n];
                y = rows[i + 1].getElementsByTagName(&quot;TD&quot;)[n];
                /*check if the two rows should switch place,
                based on the direction, asc or desc:*/
                if (dir == &quot;asc&quot;) {
                    if (x.innerHTML.toLowerCase() > y.innerHTML.toLowerCase()) {
                        //if so, mark as a switch and break the loop:
                        shouldSwitch = true;
                        break;
                    }
                } else if (dir == &quot;desc&quot;) {
                    if (x.innerHTML.toLowerCase() &lt; y.innerHTML.toLowerCase()) {
                        //if so, mark as a switch and break the loop:
                        shouldSwitch = true;
                        break;
                    }
                }
            }
            if (shouldSwitch) {
                /*If a switch has been marked, make the switch
                and mark that a switch has been done:*/
                rows[i].parentNode.insertBefore(rows[i + 1], rows[i]);
                switching = true;
                //Each time a switch is done, increase this count by 1:
                switchcount++;
            } else {
                /*If no switching has been done AND the direction is &quot;asc&quot;,
                set the direction to &quot;desc&quot; and run the while loop again.*/
                if (switchcount == 0 &amp;&amp; dir == &quot;asc&quot;) {
                    dir = &quot;desc&quot;;
                    switching = true;
                }
            }
        }
    }




$(document).ready(function() {
    $('#messageAllowance').fadeOut(10000);
});








    


    
        Unique 3 digit number between 001 and 898 used to identify the allowance in payroll processing
    

    
        Unique short description (max 8 characters) that will be displayed on the payslip &amp; payroll reports
    

    
        Long description (max 25 characters) that will be used for screen display &amp; reporting
    

    
        Select True if the allowance is subject to tax 
    

    
        Select True if the allowance is classified as pensionable
    

    
        Select True if the allowance is subject to tax 
    

    
        Select True if the allowance should be included in overall pension reform earnings as part of regular pay
    

    
        Select True if the balance calculated against the allowance should shown on the payslip
    

    
        Defines if and what values should  be included in Office of National Statistics reporting he allowance 
    

    
        How should cash payments for this allowance be categorised in Office of National Statistics reporting?
    

    
        How should hours values for this allowance be categorised in Office of National Statistics reporting?
    

    
        If this allowance be included in minimum pay validation, select the option that defines how it should be used
    

    
        Is the allowance input as a net value and grossed for tax and NI purposes if necessary
    

    
        The allowance will be added to gross after the net to gross calculation has been done. Default is the allowance will be calculated for tax and ni prior to net to gross items
    

    
        Select True if the allowance is subject to salary sacrifice
    

    
        Select True if values calculated against the allowance are payable to the employee
    

    
        Select True if values calculated against the allowance are payable to the employee
    

    
        Select True if the payments calculated against the allowance should shown on the payslip 
    

    
        Select True if the balance (year to date or increasing / reducing) calculated against the allowance should shown on the payslip 
    

    
        Select True if back-dated starter payments should be pro-rated. This only applies to permanent allowances. Pro-Ration rules are set on the Base Parameters screen
    

    
        Select True if back-dated leaver payments should be pro-rated. This only applies to permanent allowances. Pro-Ration rules are defined on the Base Parameters screen
    

    
        Define whether this allowance is a permanent fixed value, variable or both
    

    
        Defines how values entered against this allowance should be processed
    

    
        Where Input Type is set to one of the percentage options, this field can be used to define the default percentage to be applied to all employees (unless individual override entries set at employee level)
    

    
        Where Input Type is set to one of the percentage options, this field can be used to define the value it is a percentage of, where that value is the sum of payments for a number of other allowances.  This sum will need to first be defined against one of the 20 available accumulators
    

    
        Select True if the allowance payment should reference the employee's contracted hours per period
    

    
        Select True if the allowance is exempt from Court Order calculations
    

    
        Select True if the allowances should be excluded from the Financial Summary Report
    

    
        Select True if the allowance should continue to paid if employee marked as on holiday
    

    
        Select True if the allowance should continue to paid if employee marked as suspended
    

    
        Defines whether the allowance To Date value should be cleared down each tax year before a fixed period each year (e.g.setting to 01 will clear to date before processing period 01 of a new tax year))
    

    
        Select True to suppress warning messages for this allowance from the input exception report
    

    
        If there is a minimum value for this allowance per pay period, select the appropriate option from the list then enter a corresponding value in Lower Limit Value
    

    
        Enter a value in here if Lower Limit Option has been selected.  If no entry is made, default will be 0.00
    

    
        If there is a maximum value for this allowance per pay period, select the appropriate option from the list then enter a corresponding value in Upper Limit Value
    

    
        Enter a value in here if Upper Limit Option has been selected.  If no entry is made, default will be 40,000.00
    



    
        
            
                
                    
                        Action
                        Allowance Number
                        Allowance Short Description
                        Allowance Long Description            
                        Taxable
                        Pensionable
                        Ni'ble
                        Pension Reform earnings?
                        Benefit in kind?
                        Part of ONS reporting
                        ONS Cash category
                        ONS hours Category
                        Minimum pay option 
                        Net to Gross
                        Calculate allowance after Net to Gross            
                        Salary Sacrifice
                        Pay requirement
                        Print requirement
                        Print balance
                        Auto Proportion Starter
                        Auto Proportion Leaver
                        Permanent /Variable / Both
                        Input Type
                        Percentage Value
                        Percentage of Accumulator
                        Pay Basic Hours
                        Court Order Eligibility
                        Financial summary exemption
                        
                            Pay during holidays
                        
                        
                            Pay if suspended
                        
                        
                            Clear down Tax Period
                        
                        
                            Suppress Warning
                        
                        
                            Lower Limit Option
                        
                        
                            Lower Limit Value
                        
                        
                            Upper Limit Option
                        
                        
                            Upper Limit Value
                        
                    
                
            


            
                
                            
        (function($) {
            $.fn.swhgLoad = function(url, containerId, callback) {
                url = url + (url.indexOf('?') == -1 ? '?' : '&amp;') + '__swhg=' + new Date().getTime();

                $('&lt;div/>').load(url + ' ' + containerId, function(data, status, xhr) {
                    $(containerId).replaceWith($(this).html());
                    if (typeof(callback) === 'function') {
                        callback.apply(this, arguments);
                    }
                });
                return this;
            }

            $(function() {
                $('table[data-swhgajax=&quot;true&quot;],span[data-swhgajax=&quot;true&quot;]').each(function() {
                    var self = $(this);
                    var containerId = '#' + self.data('swhgcontainer');
                    var callback = getFunction(self.data('swhgcallback'));

                    $(containerId).parent().delegate(containerId + ' a[data-swhglnk=&quot;true&quot;]', 'click', function() {
                        $(containerId).swhgLoad($(this).attr('href'), containerId, callback);
                        return false;
                    });
                })
            });

            function getFunction(code, argNames) {
                argNames = argNames || [];
                var fn = window, parts = (code || &quot;&quot;).split(&quot;.&quot;);
                while (fn &amp;&amp; parts.length) {
                    fn = fn[parts.shift()];
                }
                if (typeof (fn) === &quot;function&quot;) {
                    return fn;
                }
                argNames.push(code);
                return Function.constructor.apply(null, argNames);
            }
        })(jQuery);
        
    
    
        
            
Action            
            
Allowance Number            
            
Allowance Short Description            
            
Allowance Long Description            
            
Taxable            
            
Pensionable            
            
Ni'ble            
            
Pension Reform earnings?            
            
Benefit in kind?            
            
Part of ONS reporting            
            
ONS Cash category            
            
ONS hours Category            
            
Minimum pay option            
            
Net to Gross            
            
Calculate allowance after Net to Gross            
            
Salary Sacrifice            
            
Pay requirement            
            
Print requirement            
            
Print balance            
            
Auto Proportion Starter            
            
Auto Proportion Leaver            
            
Payment Type            
            
Input Type            
            
Percentage Value            
            
Percentage of Accumulator            
            
Pay Basic Hours            
            
Court Order Eligibility            
            
Financial summary exemption            
            
Pay during holidays            
            
Pay if suspended            
            
Clear down Tax Period            
            
Suppress Warning            
            
Lower Limit Option            
            
Lower Limit Value            
            
Upper Limit Option            
            
Upper Limit Value            
        
    
    
        
            1 2 3 4 Next > Last >>
        
    
    
        
             
             Edit
             Delete
             Save
             Cancel 
            
            


              386  
               test  
              mobile phone package  
                Yes  Yes
No
  
               Yes  Yes
No
 
               Yes  Yes
No
 
              Yes Yes
No
 
               No  Yes
No
 
              
                     
                    Exclude from ONS reporting
                    None
Exclude from ONS reporting
This Period Hours / Cash (Base only)
Include as ONS Bonus
This Period Cash Only (No Hours)
This Period Cash (Base Only)
This Period Hours / Cash (Default)

                     
               SALARY SALARY
ABSENCE
BONUS
COMMISSION
EXPENSES
HOLIDAY PAY
OVERTIME
OTHER PAY
OCCUPATIONAL PARENTAL PAY
OCCUPATIONAL SICK PAY
SALARY EXCHANGE
SHIFT PAY
TERMINATION

               BASIC HOURS  BASIC HOURS
ADDITIONAL HOURS
HOLIDAY HOURS
OVERTIME HOURS
UNPAID HOURS

               Exclude from min pay calculation Exclude from min pay calculation
This Period Hours / Cash (Base only)
This Period Hours Only
This Period Cash Only
This Period Cash (Base Only)
Do not Include
Salary Sacrifice Offset Validation
This Period Hours / Cash

              No  Yes
No
 
               No Yes
No
 
             Yes  Yes
No
 
              Yes  Yes
No
 
              Yes  Yes
No
 
              No  Yes
No
 
               No  Yes
No
 
               No  Yes
No
 
               Permanent  Permanent
Variable
Both
  
              Cash  Cash
Units
Hours &amp; Rate
Piece Rates
Percentage this run
Percentage Cumulative
Percentage Hours
Percentage this run

                 
               EA01 EA01
HA01
EA02
HA02
EA03
HA03
EA04
HA04
EA05
HA05
EA06
HA06
EA07
HA07
EA08
HA08
EA09
HA09
EA10
HA10
EA11
EA12
EA13
EA14
EA15
EA16
EA17
EA18
EA19
EA20

              No  Yes
No
 
               No  Yes
No

               No  Yes
No
 
               No Yes
No
 
               No  Yes
No
 
                
               No  Yes
No
 
               Pay the Calculated amountPay the Calculated amount
Pay Min Value, balance as Cap
Pay Cumulative amount
Pay the Minimum amount
Subtract Min from Calc Amount : Round up to 0
Pay the  Allowance value
Pay only if Min amount reached

                 
               Pay the Calculated amount Pay the Calculated amount
Pay Max Value, balance as Cap
Pay Cummulative amount
Pay the Maximum amount
Pay up to Allowance value as Cap
Pay only if value does not exceed Cap

                 
        
        
             
             Edit
             Delete
             Save
             Cancel 
            
            


              200  
               test4  
              sample test allowance  
                Yes  Yes
No
  
               Yes  Yes
No
 
               Yes  Yes
No
 
              Yes Yes
No
 
               No  Yes
No
 
              
                     
                    Exclude from ONS reporting
                    None
Exclude from ONS reporting
This Period Hours / Cash (Base only)
Include as ONS Bonus
This Period Cash Only (No Hours)
This Period Cash (Base Only)
This Period Hours / Cash (Default)

                     
               SALARY SALARY
ABSENCE
BONUS
COMMISSION
EXPENSES
HOLIDAY PAY
OVERTIME
OTHER PAY
OCCUPATIONAL PARENTAL PAY
OCCUPATIONAL SICK PAY
SALARY EXCHANGE
SHIFT PAY
TERMINATION

               BASIC HOURS  BASIC HOURS
ADDITIONAL HOURS
HOLIDAY HOURS
OVERTIME HOURS
UNPAID HOURS

               Exclude from min pay calculation Exclude from min pay calculation
This Period Hours / Cash (Base only)
This Period Hours Only
This Period Cash Only
This Period Cash (Base Only)
Do not Include
Salary Sacrifice Offset Validation
This Period Hours / Cash

              No  Yes
No
 
               No Yes
No
 
             No  Yes
No
 
              Yes  Yes
No
 
              Yes  Yes
No
 
              No  Yes
No
 
               No  Yes
No
 
               No  Yes
No
 
               Permanent  Permanent
Variable
Both
  
              Cash  Cash
Units
Hours &amp; Rate
Piece Rates
Percentage this run
Percentage Cumulative
Percentage Hours
Percentage this run

                 
               EA01 EA01
HA01
EA02
HA02
EA03
HA03
EA04
HA04
EA05
HA05
EA06
HA06
EA07
HA07
EA08
HA08
EA09
HA09
EA10
HA10
EA11
EA12
EA13
EA14
EA15
EA16
EA17
EA18
EA19
EA20

              No  Yes
No
 
               No  Yes
No

               No  Yes
No
 
               No Yes
No
 
               No  Yes
No
 
                
               No  Yes
No
 
               Pay the Calculated amountPay the Calculated amount
Pay Min Value, balance as Cap
Pay Cumulative amount
Pay the Minimum amount
Subtract Min from Calc Amount : Round up to 0
Pay the  Allowance value
Pay only if Min amount reached

                 
               Pay the Calculated amount Pay the Calculated amount
Pay Max Value, balance as Cap
Pay Cummulative amount
Pay the Maximum amount
Pay up to Allowance value as Cap
Pay only if value does not exceed Cap

                 
        
        
             
             Edit
             Delete
             Save
             Cancel 
            
            


              006  
               test1  
              test  
                Yes  Yes
No
  
               Yes  Yes
No
 
               Yes  Yes
No
 
              Yes Yes
No
 
               No  Yes
No
 
              
                     
                    Exclude from ONS reporting
                    None
Exclude from ONS reporting
This Period Hours / Cash (Base only)
Include as ONS Bonus
This Period Cash Only (No Hours)
This Period Cash (Base Only)
This Period Hours / Cash (Default)

                     
               SALARY SALARY
ABSENCE
BONUS
COMMISSION
EXPENSES
HOLIDAY PAY
OVERTIME
OTHER PAY
OCCUPATIONAL PARENTAL PAY
OCCUPATIONAL SICK PAY
SALARY EXCHANGE
SHIFT PAY
TERMINATION

               BASIC HOURS  BASIC HOURS
ADDITIONAL HOURS
HOLIDAY HOURS
OVERTIME HOURS
UNPAID HOURS

               Exclude from min pay calculation Exclude from min pay calculation
This Period Hours / Cash (Base only)
This Period Hours Only
This Period Cash Only
This Period Cash (Base Only)
Do not Include
Salary Sacrifice Offset Validation
This Period Hours / Cash

              No  Yes
No
 
               No Yes
No
 
             Yes  Yes
No
 
              Yes  Yes
No
 
              Yes  Yes
No
 
              No  Yes
No
 
               No  Yes
No
 
               No  Yes
No
 
               Permanent  Permanent
Variable
Both
  
              Cash  Cash
Units
Hours &amp; Rate
Piece Rates
Percentage this run
Percentage Cumulative
Percentage Hours
Percentage this run

                 
               EA01 EA01
HA01
EA02
HA02
EA03
HA03
EA04
HA04
EA05
HA05
EA06
HA06
EA07
HA07
EA08
HA08
EA09
HA09
EA10
HA10
EA11
EA12
EA13
EA14
EA15
EA16
EA17
EA18
EA19
EA20

              No  Yes
No
 
               No  Yes
No

               No  Yes
No
 
               No Yes
No
 
               No  Yes
No
 
                
               No  Yes
No
 
               Pay the Calculated amountPay the Calculated amount
Pay Min Value, balance as Cap
Pay Cumulative amount
Pay the Minimum amount
Subtract Min from Calc Amount : Round up to 0
Pay the  Allowance value
Pay only if Min amount reached

                 
               Pay the Calculated amount Pay the Calculated amount
Pay Max Value, balance as Cap
Pay Cummulative amount
Pay the Maximum amount
Pay up to Allowance value as Cap
Pay only if value does not exceed Cap

                 
        
        
             
             Edit
             Delete
             Save
             Cancel 
            
            


              555  
               test  
              test  
                Yes  Yes
No
  
               Yes  Yes
No
 
               Yes  Yes
No
 
              Yes Yes
No
 
               No  Yes
No
 
              
                     
                    Exclude from ONS reporting
                    None
Exclude from ONS reporting
This Period Hours / Cash (Base only)
Include as ONS Bonus
This Period Cash Only (No Hours)
This Period Cash (Base Only)
This Period Hours / Cash (Default)

                     
               SALARY SALARY
ABSENCE
BONUS
COMMISSION
EXPENSES
HOLIDAY PAY
OVERTIME
OTHER PAY
OCCUPATIONAL PARENTAL PAY
OCCUPATIONAL SICK PAY
SALARY EXCHANGE
SHIFT PAY
TERMINATION

               BASIC HOURS  BASIC HOURS
ADDITIONAL HOURS
HOLIDAY HOURS
OVERTIME HOURS
UNPAID HOURS

               Exclude from min pay calculation Exclude from min pay calculation
This Period Hours / Cash (Base only)
This Period Hours Only
This Period Cash Only
This Period Cash (Base Only)
Do not Include
Salary Sacrifice Offset Validation
This Period Hours / Cash

              No  Yes
No
 
               No Yes
No
 
             Yes  Yes
No
 
              Yes  Yes
No
 
              Yes  Yes
No
 
              No  Yes
No
 
               No  Yes
No
 
               No  Yes
No
 
               Permanent  Permanent
Variable
Both
  
              Cash  Cash
Units
Hours &amp; Rate
Piece Rates
Percentage this run
Percentage Cumulative
Percentage Hours
Percentage this run

                 
               EA01 EA01
HA01
EA02
HA02
EA03
HA03
EA04
HA04
EA05
HA05
EA06
HA06
EA07
HA07
EA08
HA08
EA09
HA09
EA10
HA10
EA11
EA12
EA13
EA14
EA15
EA16
EA17
EA18
EA19
EA20

              No  Yes
No
 
               No  Yes
No

               No  Yes
No
 
               No Yes
No
 
               No  Yes
No
 
                
               No  Yes
No
 
               Pay the Calculated amountPay the Calculated amount
Pay Min Value, balance as Cap
Pay Cumulative amount
Pay the Minimum amount
Subtract Min from Calc Amount : Round up to 0
Pay the  Allowance value
Pay only if Min amount reached

                 
               Pay the Calculated amount Pay the Calculated amount
Pay Max Value, balance as Cap
Pay Cummulative amount
Pay the Maximum amount
Pay up to Allowance value as Cap
Pay only if value does not exceed Cap

                 
        
        
             
             Edit
             Delete
             Save
             Cancel 
            
            


              234  
               test  
              testlong  
                Yes  Yes
No
  
               Yes  Yes
No
 
               Yes  Yes
No
 
              Yes Yes
No
 
               No  Yes
No
 
              
                     
                    Exclude from ONS reporting
                    None
Exclude from ONS reporting
This Period Hours / Cash (Base only)
Include as ONS Bonus
This Period Cash Only (No Hours)
This Period Cash (Base Only)
This Period Hours / Cash (Default)

                     
               SALARY SALARY
ABSENCE
BONUS
COMMISSION
EXPENSES
HOLIDAY PAY
OVERTIME
OTHER PAY
OCCUPATIONAL PARENTAL PAY
OCCUPATIONAL SICK PAY
SALARY EXCHANGE
SHIFT PAY
TERMINATION

               BASIC HOURS  BASIC HOURS
ADDITIONAL HOURS
HOLIDAY HOURS
OVERTIME HOURS
UNPAID HOURS

               Exclude from min pay calculation Exclude from min pay calculation
This Period Hours / Cash (Base only)
This Period Hours Only
This Period Cash Only
This Period Cash (Base Only)
Do not Include
Salary Sacrifice Offset Validation
This Period Hours / Cash

              No  Yes
No
 
               No Yes
No
 
             No  Yes
No
 
              Yes  Yes
No
 
              Yes  Yes
No
 
              No  Yes
No
 
               No  Yes
No
 
               No  Yes
No
 
               Permanent  Permanent
Variable
Both
  
              Cash  Cash
Units
Hours &amp; Rate
Piece Rates
Percentage this run
Percentage Cumulative
Percentage Hours
Percentage this run

                 
               EA01 EA01
HA01
EA02
HA02
EA03
HA03
EA04
HA04
EA05
HA05
EA06
HA06
EA07
HA07
EA08
HA08
EA09
HA09
EA10
HA10
EA11
EA12
EA13
EA14
EA15
EA16
EA17
EA18
EA19
EA20

              No  Yes
No
 
               No  Yes
No

               No  Yes
No
 
               No Yes
No
 
               No  Yes
No
 
                
               No  Yes
No
 
               Pay the Calculated amountPay the Calculated amount
Pay Min Value, balance as Cap
Pay Cumulative amount
Pay the Minimum amount
Subtract Min from Calc Amount : Round up to 0
Pay the  Allowance value
Pay only if Min amount reached

                 
               Pay the Calculated amount Pay the Calculated amount
Pay Max Value, balance as Cap
Pay Cummulative amount
Pay the Maximum amount
Pay up to Allowance value as Cap
Pay only if value does not exceed Cap

                 
        
        
             
             Edit
             Delete
             Save
             Cancel 
            
            


              222  
               test  
              test  
                Yes  Yes
No
  
               Yes  Yes
No
 
               Yes  Yes
No
 
              Yes Yes
No
 
               No  Yes
No
 
              
                     
                    Exclude from ONS reporting
                    None
Exclude from ONS reporting
This Period Hours / Cash (Base only)
Include as ONS Bonus
This Period Cash Only (No Hours)
This Period Cash (Base Only)
This Period Hours / Cash (Default)

                     
               SALARY SALARY
ABSENCE
BONUS
COMMISSION
EXPENSES
HOLIDAY PAY
OVERTIME
OTHER PAY
OCCUPATIONAL PARENTAL PAY
OCCUPATIONAL SICK PAY
SALARY EXCHANGE
SHIFT PAY
TERMINATION

               BASIC HOURS  BASIC HOURS
ADDITIONAL HOURS
HOLIDAY HOURS
OVERTIME HOURS
UNPAID HOURS

               Exclude from min pay calculation Exclude from min pay calculation
This Period Hours / Cash (Base only)
This Period Hours Only
This Period Cash Only
This Period Cash (Base Only)
Do not Include
Salary Sacrifice Offset Validation
This Period Hours / Cash

              No  Yes
No
 
               No Yes
No
 
             Yes  Yes
No
 
              Yes  Yes
No
 
              Yes  Yes
No
 
              No  Yes
No
 
               No  Yes
No
 
               No  Yes
No
 
               Permanent  Permanent
Variable
Both
  
              Cash  Cash
Units
Hours &amp; Rate
Piece Rates
Percentage this run
Percentage Cumulative
Percentage Hours
Percentage this run

                 
               EA01 EA01
HA01
EA02
HA02
EA03
HA03
EA04
HA04
EA05
HA05
EA06
HA06
EA07
HA07
EA08
HA08
EA09
HA09
EA10
HA10
EA11
EA12
EA13
EA14
EA15
EA16
EA17
EA18
EA19
EA20

              No  Yes
No
 
               No  Yes
No

               No  Yes
No
 
               No Yes
No
 
               No  Yes
No
 
                
               No  Yes
No
 
               Pay the Calculated amountPay the Calculated amount
Pay Min Value, balance as Cap
Pay Cumulative amount
Pay the Minimum amount
Subtract Min from Calc Amount : Round up to 0
Pay the  Allowance value
Pay only if Min amount reached

                 
               Pay the Calculated amount Pay the Calculated amount
Pay Max Value, balance as Cap
Pay Cummulative amount
Pay the Maximum amount
Pay up to Allowance value as Cap
Pay only if value does not exceed Cap

                 
        
        
             
             Edit
             Delete
             Save
             Cancel 
            
            


              613  
               Test 11  
              sample test2  
                Yes  Yes
No
  
               Yes  Yes
No
 
               Yes  Yes
No
 
              Yes Yes
No
 
               No  Yes
No
 
              
                     
                    Exclude from ONS reporting
                    None
Exclude from ONS reporting
This Period Hours / Cash (Base only)
Include as ONS Bonus
This Period Cash Only (No Hours)
This Period Cash (Base Only)
This Period Hours / Cash (Default)

                     
               SALARY SALARY
ABSENCE
BONUS
COMMISSION
EXPENSES
HOLIDAY PAY
OVERTIME
OTHER PAY
OCCUPATIONAL PARENTAL PAY
OCCUPATIONAL SICK PAY
SALARY EXCHANGE
SHIFT PAY
TERMINATION

               BASIC HOURS  BASIC HOURS
ADDITIONAL HOURS
HOLIDAY HOURS
OVERTIME HOURS
UNPAID HOURS

               Exclude from min pay calculation Exclude from min pay calculation
This Period Hours / Cash (Base only)
This Period Hours Only
This Period Cash Only
This Period Cash (Base Only)
Do not Include
Salary Sacrifice Offset Validation
This Period Hours / Cash

              No  Yes
No
 
               No Yes
No
 
             No  Yes
No
 
              Yes  Yes
No
 
              Yes  Yes
No
 
              No  Yes
No
 
               No  Yes
No
 
               No  Yes
No
 
               Permanent  Permanent
Variable
Both
  
              Cash  Cash
Units
Hours &amp; Rate
Piece Rates
Percentage this run
Percentage Cumulative
Percentage Hours
Percentage this run

                 
               EA01 EA01
HA01
EA02
HA02
EA03
HA03
EA04
HA04
EA05
HA05
EA06
HA06
EA07
HA07
EA08
HA08
EA09
HA09
EA10
HA10
EA11
EA12
EA13
EA14
EA15
EA16
EA17
EA18
EA19
EA20

              No  Yes
No
 
               No  Yes
No

               No  Yes
No
 
               No Yes
No
 
               No  Yes
No
 
                
               No  Yes
No
 
               Pay the Calculated amountPay the Calculated amount
Pay Min Value, balance as Cap
Pay Cumulative amount
Pay the Minimum amount
Subtract Min from Calc Amount : Round up to 0
Pay the  Allowance value
Pay only if Min amount reached

                 
               Pay the Calculated amount Pay the Calculated amount
Pay Max Value, balance as Cap
Pay Cummulative amount
Pay the Maximum amount
Pay up to Allowance value as Cap
Pay only if value does not exceed Cap

                 
        
        
             
             Edit
             Delete
             Save
             Cancel 
            
            


              712  
               Test 11  
              sample test3  
                Yes  Yes
No
  
               Yes  Yes
No
 
               Yes  Yes
No
 
              Yes Yes
No
 
               No  Yes
No
 
              
                     
                    Exclude from ONS reporting
                    None
Exclude from ONS reporting
This Period Hours / Cash (Base only)
Include as ONS Bonus
This Period Cash Only (No Hours)
This Period Cash (Base Only)
This Period Hours / Cash (Default)

                     
               SALARY SALARY
ABSENCE
BONUS
COMMISSION
EXPENSES
HOLIDAY PAY
OVERTIME
OTHER PAY
OCCUPATIONAL PARENTAL PAY
OCCUPATIONAL SICK PAY
SALARY EXCHANGE
SHIFT PAY
TERMINATION

               BASIC HOURS  BASIC HOURS
ADDITIONAL HOURS
HOLIDAY HOURS
OVERTIME HOURS
UNPAID HOURS

               Exclude from min pay calculation Exclude from min pay calculation
This Period Hours / Cash (Base only)
This Period Hours Only
This Period Cash Only
This Period Cash (Base Only)
Do not Include
Salary Sacrifice Offset Validation
This Period Hours / Cash

              No  Yes
No
 
               No Yes
No
 
             No  Yes
No
 
              Yes  Yes
No
 
              Yes  Yes
No
 
              No  Yes
No
 
               No  Yes
No
 
               No  Yes
No
 
               Permanent  Permanent
Variable
Both
  
              Cash  Cash
Units
Hours &amp; Rate
Piece Rates
Percentage this run
Percentage Cumulative
Percentage Hours
Percentage this run

                 
               EA01 EA01
HA01
EA02
HA02
EA03
HA03
EA04
HA04
EA05
HA05
EA06
HA06
EA07
HA07
EA08
HA08
EA09
HA09
EA10
HA10
EA11
EA12
EA13
EA14
EA15
EA16
EA17
EA18
EA19
EA20

              No  Yes
No
 
               No  Yes
No

               No  Yes
No
 
               No Yes
No
 
               No  Yes
No
 
                
               No  Yes
No
 
               Pay the Calculated amountPay the Calculated amount
Pay Min Value, balance as Cap
Pay Cumulative amount
Pay the Minimum amount
Subtract Min from Calc Amount : Round up to 0
Pay the  Allowance value
Pay only if Min amount reached

                 
               Pay the Calculated amount Pay the Calculated amount
Pay Max Value, balance as Cap
Pay Cummulative amount
Pay the Maximum amount
Pay up to Allowance value as Cap
Pay only if value does not exceed Cap

                 
        
        
             
             Edit
             Delete
             Save
             Cancel 
            
            


              734  
               test222  
              test456  
                Yes  Yes
No
  
               Yes  Yes
No
 
               Yes  Yes
No
 
              Yes Yes
No
 
               No  Yes
No
 
              
                     
                    Exclude from ONS reporting
                    None
Exclude from ONS reporting
This Period Hours / Cash (Base only)
Include as ONS Bonus
This Period Cash Only (No Hours)
This Period Cash (Base Only)
This Period Hours / Cash (Default)

                     
               SALARY SALARY
ABSENCE
BONUS
COMMISSION
EXPENSES
HOLIDAY PAY
OVERTIME
OTHER PAY
OCCUPATIONAL PARENTAL PAY
OCCUPATIONAL SICK PAY
SALARY EXCHANGE
SHIFT PAY
TERMINATION

               BASIC HOURS  BASIC HOURS
ADDITIONAL HOURS
HOLIDAY HOURS
OVERTIME HOURS
UNPAID HOURS

               Exclude from min pay calculation Exclude from min pay calculation
This Period Hours / Cash (Base only)
This Period Hours Only
This Period Cash Only
This Period Cash (Base Only)
Do not Include
Salary Sacrifice Offset Validation
This Period Hours / Cash

              No  Yes
No
 
               No Yes
No
 
             No  Yes
No
 
              Yes  Yes
No
 
              Yes  Yes
No
 
              No  Yes
No
 
               No  Yes
No
 
               No  Yes
No
 
               Permanent  Permanent
Variable
Both
  
              Cash  Cash
Units
Hours &amp; Rate
Piece Rates
Percentage this run
Percentage Cumulative
Percentage Hours
Percentage this run

                 
               EA01 EA01
HA01
EA02
HA02
EA03
HA03
EA04
HA04
EA05
HA05
EA06
HA06
EA07
HA07
EA08
HA08
EA09
HA09
EA10
HA10
EA11
EA12
EA13
EA14
EA15
EA16
EA17
EA18
EA19
EA20

              No  Yes
No
 
               No  Yes
No

               No  Yes
No
 
               No Yes
No
 
               No  Yes
No
 
                
               No  Yes
No
 
               Pay the Calculated amountPay the Calculated amount
Pay Min Value, balance as Cap
Pay Cumulative amount
Pay the Minimum amount
Subtract Min from Calc Amount : Round up to 0
Pay the  Allowance value
Pay only if Min amount reached

                 
               Pay the Calculated amount Pay the Calculated amount
Pay Max Value, balance as Cap
Pay Cummulative amount
Pay the Maximum amount
Pay up to Allowance value as Cap
Pay only if value does not exceed Cap

                 
        
        
             
             Edit
             Delete
             Save
             Cancel 
            
            


              212  
               wer  
              qwerty  
                Yes  Yes
No
  
               Yes  Yes
No
 
               Yes  Yes
No
 
              Yes Yes
No
 
               No  Yes
No
 
              
                     
                    Exclude from ONS reporting
                    None
Exclude from ONS reporting
This Period Hours / Cash (Base only)
Include as ONS Bonus
This Period Cash Only (No Hours)
This Period Cash (Base Only)
This Period Hours / Cash (Default)

                     
               SALARY SALARY
ABSENCE
BONUS
COMMISSION
EXPENSES
HOLIDAY PAY
OVERTIME
OTHER PAY
OCCUPATIONAL PARENTAL PAY
OCCUPATIONAL SICK PAY
SALARY EXCHANGE
SHIFT PAY
TERMINATION

               BASIC HOURS  BASIC HOURS
ADDITIONAL HOURS
HOLIDAY HOURS
OVERTIME HOURS
UNPAID HOURS

               Exclude from min pay calculation Exclude from min pay calculation
This Period Hours / Cash (Base only)
This Period Hours Only
This Period Cash Only
This Period Cash (Base Only)
Do not Include
Salary Sacrifice Offset Validation
This Period Hours / Cash

              No  Yes
No
 
               No Yes
No
 
             No  Yes
No
 
              Yes  Yes
No
 
              Yes  Yes
No
 
              No  Yes
No
 
               No  Yes
No
 
               No  Yes
No
 
               Permanent  Permanent
Variable
Both
  
              Cash  Cash
Units
Hours &amp; Rate
Piece Rates
Percentage this run
Percentage Cumulative
Percentage Hours
Percentage this run

                 
               HA01 EA01
HA01
EA02
HA02
EA03
HA03
EA04
HA04
EA05
HA05
EA06
HA06
EA07
HA07
EA08
HA08
EA09
HA09
EA10
HA10
EA11
EA12
EA13
EA14
EA15
EA16
EA17
EA18
EA19
EA20

              No  Yes
No
 
               No  Yes
No

               No  Yes
No
 
               No Yes
No
 
               No  Yes
No
 
                
               No  Yes
No
 
               Pay the Calculated amountPay the Calculated amount
Pay Min Value, balance as Cap
Pay Cumulative amount
Pay the Minimum amount
Subtract Min from Calc Amount : Round up to 0
Pay the  Allowance value
Pay only if Min amount reached

                 
               Pay the Calculated amount Pay the Calculated amount
Pay Max Value, balance as Cap
Pay Cummulative amount
Pay the Maximum amount
Pay up to Allowance value as Cap
Pay only if value does not exceed Cap

                 
        
    
    

                

            
        
    

    

    
    
    
        
            
                
                    ×
                    Remove Allowance
                
                
                    
                        
                            
                                Are you sure you want to delete the Allowance  ?
                                

                            
                        
                    
                    
                        Yes
                        No
                    
                
            
        

    



    $(document).ready(function () {
        $('.pane-hScroll').scroll(function (e) {
            $('thead').css(&quot;left&quot;, -$('.pane-hScroll').scrollLeft()); //fix the thead relative to the body scrolling
            $('thead th:nth-child(1)').css(&quot;left&quot;, $('.pane-hScroll').scrollLeft()); //fix the first cell of the header
            $('tbody td:nth-child(1)').css(&quot;left&quot;, $('.pane-hScroll').scrollLeft()); //fix the first column of tdbody
            $('thead th:nth-child(2)').css(&quot;left&quot;, $('.pane-hScroll').scrollLeft()); //fix the first cell of the header
            $('tbody td:nth-child(2)').css(&quot;left&quot;, $('.pane-hScroll').scrollLeft()); //fix the first column of tdbody
            $('thead th:nth-child(3)').css(&quot;left&quot;, $('.pane-hScroll').scrollLeft()); //fix the first cell of the header
            $('tbody td:nth-child(3)').css(&quot;left&quot;, $('.pane-hScroll').scrollLeft()); //fix the first column of tdbody

        });
    });





    tr.footer {
        position: absolute;
        margin-top: 400px;
        z-index: 1;
        width: 159px;
    }

    /*thead*/
    thead {
        display: block;
        border-collapse: separate;
        border-spacing: 0;
    }

        thead th {
            min-width: 160px;
            height: 100px;
            background: #f7f7f7;
        }

            thead th:nth-child(1) {
                position: relative;
                display: flex;
                align-items: center;
                justify-content: center;
                height: 120px;
            }
            thead th:nth-child(2) {
                position: relative;
            }

            thead th:nth-child(3) {
                position: relative;
            }

    /*tbody*/
    tbody {
        position: relative;
        display: block;
        height: 400px;
        overflow-y: scroll;
        overflow-x: hidden;
        border-collapse: separate;
        border-spacing: 0;
    }

        tbody td {
            min-width: 160px;
            background: #fff;
        }

        tbody tr td:nth-child(1) { /*the first cell in each tr*/
            position: relative;
        }

        tbody tr td:nth-child(2) { /*the first cell in each tr*/
            position: relative;
        }

        tbody tr td:nth-child(3) { /*the first cell in each tr*/
            position: relative;
        }

    tr:nth-child(odd) > td {
        border-collapse: separate;
        border-spacing: 0;
        background-color: #f9f9f9;
    }
    .table-responsive {
        min-height: .01%;
        overflow-x: hidden;
        overflow-y: hidden;
        height:440px;

    }



            
        

  
    
        

            Earnings Accumulators
            
            
                Allowances can be grouped together for more complex calculation purposes using accumulators. Twenty Earnings accumulators are available to configure, having determined what the purpose of each accumulator will be, this screen should be used to define what allowances should be included in each &amp; what type of cash value should be used:

                            C - Add cumulative value         D - Minus cumulative value    
                            P - Add Permanent value          Q - Minus Permanent value   
                            T - Add This time value            U - Minus This time value   

            

            
            
                
                     Payroll Number: 
                    
                        000001 -SD Worx Mauritius
                        000002 -SD Worx Ireland
                    
                
            
            
                   Set Status to:   Not Required
In Progress
Completed
  
            
            
         
 
 
            
                


$(document).ready(function() {
    $('#messageEarning').fadeOut(10000);
});



      
    
    


          
    
    

        

            
                
                        
    
        
            
RowNumber            
            
Allowance Number            
            
Allowance Short Description            
            
Allowance Long Description            
            
EA01            
            
EA02            
            
EA03            
            
EA04            
            
EA05            
            
EA06            
            
EA07            
            
EA08            
            
EA09            
            
EA10            
            
EA11            
            
EA12            
            
EA13            
            
EA14            
            
EA15            
            
EA16            
            
EA17            
            
EA18            
            
EA19            
            
EA20            
        
    
    
        
            1 2 3 4 Next > Last >>
        
    
    
        
            1
            
            

            test
        
            
            mobile phone package
        
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
        
        
            2
            
            

            test4
        
            
            sample test allowance
        
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
        
        
            3
            
            

            test1
        
            
            test
        
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
        
        
            4
            
            

            test
        
            
            test
        
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
        
        
            5
            
            

            test
        
            
            testlong
        
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
        
        
            6
            
            

            test
        
            
            test
        
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
        
        
            7
            
            

            Test 11
        
            
            sample test2
        
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
        
        
            8
            
            

            Test 11
        
            
            sample test3
        
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
        
        
            9
            
            

            test222
        
            
            test456
        
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
        
        
            10
            
            

            wer
        
            
            qwerty
        
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
            
        
    
    


                

            
        
    



    .pane-vScroll.Earning {
    width: 3699px;
}

 
        .nodisplay {
            display: none;
            width:150px;
        }
 
    /*
       input:focus {
            outline: 1px
        }*/

        input {
           width: 100PX;
           
        } 
        .edithours {
            /* outline: none; */
            border: none;
            background-color: transparent;
        }

        .edithours_all {
            /*outline: none; */
          border: none;
        background-color : transparent;
            font-weight: bold;
        }


     


        td {
            max-width: 10px;
        }









 
             
          

 

        

            Hours Accumulators
            
            
                Allowances can be grouped together for more complex calculation purposes using accumulators. Ten Hours accumulators are available to configure, having determined what the purpose of each accumulator will be, this screen should be used to define what allowances should be included in each &amp; what type of Hours value should be used:
                             1 - Worked + premium hours This Time value   
                             2 - Worked hours only This Time value   
                             3 - Premium hours only This Time value  

            
            
            
                
                     Payroll Number: 
                    
                        000001 -SD Worx Mauritius
                        000002 -SD Worx Ireland
                    
                
            
            
                 Set Status to:   Not Required
In Progress
Completed
 
            
            

            
                 


    .pane-vScroll.Hours {
    width: 2099px;
    }

    .nodisplay {
        display: none;
        width:150px;
    }

    /*input:focus {
        outline: none;
    }*/

    input {
        width:100PX;
         
    }
.edithours {

      /* outline: none; */
        border: none;
        background-color : transparent;
}

.edithours_all {

        /*outline: none; */
        border: none;
        background-color : transparent;
        font-weight : bold;

}


 td {
    max-width: 10px;
}


$(document).ready(function() {
    $('#messageHours').fadeOut(10000);
});




      
    
    


          
    
            
                
                    
                        
                                
    
        
            
RowNumber            
            
Allowance Number            
            
Allowance Short Description            
            
Allowance Long Description            
            
HA01            
            
HA02            
            
HA03            
            
HA04            
            
HA05            
            
HA06            
            
HA07            
            
HA08            
            
HA09            
            
HA10            
        
    
    
        
            1 2 3 4 Next > Last >>
        
    
    
        
            1
            
            
                test 
            
            mobile phone package 
            
            
            
            
            
            
            
            
            
            
        
        
            2
            
            
                test4 
            
            sample test allowance 
            
            
            
            
            
            
            
            
            
            
        
        
            3
            
            
                test1 
            
            test 
            
            
            
            
            
            
            
            
            
            
        
        
            4
            
            
                test 
            
            test 
            
            
            
            
            
            
            
            
            
            
        
        
            5
            
            
                test 
            
            testlong 
            
            
            
            
            
            
            
            
            
            
        
        
            6
            
            
                test 
            
            test 
            
            
            
            
            
            
            
            
            
            
        
        
            7
            
            
                Test 11 
            
            sample test2 
            
            
            
            
            
            
            
            
            
            
        
        
            8
            
            
                Test 11 
            
            sample test3 
            
            
            
            
            
            
            
            
            
            
        
        
            9
            
            
                test222 
            
            test456 
            
            
            
            
            
            
            
            
            
            
        
        
            10
            
            
                wer 
            
            qwerty 
            
            
            
            
            
            
            
            
            
            
        
    
    


                        

                    
                
            
            
 
            
          

    
</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;js flexbox flexboxlegacy canvas canvastext webgl no-touch geolocation postmessage websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms no-csstransforms3d csstransitions fontface no-generatedcontent video audio localstorage sessionstorage webworkers applicationcache svg inlinesvg smil svgclippaths&quot;]/body[1]/div[@class=&quot;container body-content&quot;]/div[@class=&quot;container&quot;]</value>
   </webElementProperties>
</WebElementEntity>
