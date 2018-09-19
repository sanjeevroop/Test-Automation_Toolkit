<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Action</name>
   <tag></tag>
   <elementGuidId>4a8666a8-01d0-4747-adbd-fde035624aea</elementGuidId>
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
      <value>pane-hScroll</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
            
                
                    
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
        
    
     Save  Cancel  YesNo  YesNo  Yes No  YesNo No  Yes Exclude from ONS reporting This Period Hours / Cash (Base only)Include as ONS Bonus This Period Cash Only (No Hours) This Period Cash (Base Only) This Period Hours / Cash (Default)SALARY ABSENCEBONUS COMMISSIONEXPENSES HOLIDAY PAYOVERTIME OTHER PAYOCCUPATIONAL PARENTAL PAY OCCUPATIONAL SICK PAY SALARY EXCHANGESHIFT PAY TERMINATION BASIC HOURS ADDITIONAL HOURS HOLIDAY HOURS OVERTIME HOURS UNPAID HOURS Exclude from min pay calculation This Period Hours / Cash (Base only) This Period Hours Only This Period Cash Only This Period Cash (Base Only) Do not Include Salary Sacrifice Offset Validation  This Period Hours / CashNoYesNo  YesNo YesYesNo  YesNo No YesNoYesNo YesPlease select Permanent Variable BothCashUnits Hours &amp; Rate Piece Rates Percentage this run Percentage Cumulative Percentage Hours Salary Sacrifice  Please selectEA01HA01 EA02HA02EA03HA03EA04HA04EA05HA05EA06HA06EA07HA07EA08HA08EA09HA09EA10HA10EA11EA12EA13EA14EA15EA16EA17EA18EA19EA20 No YesNo  Yes No  Yes No YesNo  YesNoYes Pay the Calculated amount Pay Min Value, balance as CapPay Cummulative amount Pay the Minimum amount Subtract Min from Calc Amount : Round up to 0Pay the  Allowance value Pay only if Min amount reached Pay the Calculated amount Pay Max Value, balance as CapPay Cummulative amount Pay the Maximum amount Pay up to Allowance value as Cap Pay only if value does not exceed Cap 
        
             
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
            
            


              111  
               we  
              ew  
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

                 
        
    
    

                

            
        </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;divPartialView&quot;)/div[1]/div[@class=&quot;pane pane--table1&quot;]/div[@class=&quot;pane-hScroll&quot;]</value>
   </webElementProperties>
</WebElementEntity>
