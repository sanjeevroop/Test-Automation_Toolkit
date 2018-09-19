import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

//WebUI.openBrowser('https://implementationnp.sdworx.co.uk/')

//WebUI.navigateToUrl('https://implementationnp.sdworx.co.uk/')

//WebUI.setText(findTestObject('Archives/Navigation - Allowances/input_loginfmt'), 'Sanjeev@sdworx.com')

//WebUI.click(findTestObject('Archives/Navigation - Allowances/input_idSIButton9'))

//WebUI.maximizeWindow()

//WebUI.delay(5)

//WebUI.mouseOver(findTestObject('null'))

WebUI.delay(3)

//WebUI.click(findTestObject('null'))

//WebUI.click(findTestObject('null'))

//WebUI.click(findTestObject('null'))


for (def row = 1; row <= findTestData('Allowances').getRowNumbers(); row++) {
  

	WebUI.click(findTestObject('Add Records/Add Records - Allowances/Add Allowance'))

	
	WebUI.setText(findTestObject('Add Records/Add Records - Allowances/AllowanceNumber'), findTestData('Allowances').getValue(1, row))
	
    //WebUI.setText(findTestObject('Temp/input_ShortDescription0'), findTestData('Allowances').getValue(2, row))
	
	WebUI.setText( findTestObject('Add Records/Add Records - Allowances/ShortDescription'), findTestData('Allowances').getValue(2, row))
	
    //WebUI.setText(findTestObject('Temp/input_LongDescription0'), findTestData('Allowances').getValue(3, row))
	
	WebUI.setText(findTestObject('Add Records/Add Records - Allowances/LongDescription'), findTestData('Allowances').getValue(3, row))
	
    //WebUI.selectOptionByIndex(findTestObject('Temp/select_YesNo'), findTestData('Allowances').getValue(4, row))
		
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Taxable'), findTestData('Allowances').getValue(4, row))
		
    //WebUI.selectOptionByIndex(findTestObject('Temp/select_YesNo_1'), findTestData('Allowances').getValue(5, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Pensionable'), findTestData('Allowances').getValue(5, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_Yes No (1)'), findTestData('Allowances').getValue(6, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Niable'), findTestData('Allowances').getValue(6, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_YesNo_2'), findTestData('Allowances').getValue(7, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Pension Reform Earnings'), findTestData('Allowances').getValue(7, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_No  Yes (1)'), findTestData('Allowances').getValue(8, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Benefit in kind'), findTestData('Allowances').getValue(8, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_Exclude from ONS report'), findTestData('Allowances').getValue(
           // 9, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Part of ONS reporting'), findTestData('Allowances').getValue(
           9, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_SALARY ABSENCEBONUS COM'), findTestData('Allowances').getValue(
           // 10, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Cash Category'), findTestData('Allowances').getValue(
           10, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_BASIC HOURS ADDITIONAL'), findTestData('Allowances').getValue(
            //11, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Hours Category'), findTestData('Allowances').getValue(
            11, row))
	
    //WebUI.selectOptionByIndex(findTestObject('Temp/select_Exclude from min pay ca'), findTestData('Allowances').getValue(
            //12, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Minimum pay option'), findTestData('Allowances').getValue(
            12, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_NoYes'), findTestData('Allowances').getValue(13, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Calc Allow after Net to Gross'), findTestData('Allowances').getValue(13, row))
	
    //WebUI.selectOptionByIndex(findTestObject('Temp/select_No  Yes_1'), findTestData('Allowances').getValue(14, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Salary Sacrifice'), findTestData('Allowances').getValue(14, row))
	
    //WebUI.selectOptionByIndex(findTestObject('Temp/select_No Yes (1)'), findTestData('Allowances').getValue(15, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Pay requirement'), findTestData('Allowances').getValue(15, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_YesNo_3'), findTestData('Allowances').getValue(16, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Print requirement'), findTestData('Allowances').getValue(16, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_YesNo_4'), findTestData('Allowances').getValue(17, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Print balance'), findTestData('Allowances').getValue(17, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_No Yes_1 (1)'), findTestData('Allowances').getValue(18, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Auto Proporttion Starter'), findTestData('Allowances').getValue(18, row))
	
    //WebUI.selectOptionByIndex(findTestObject('Temp/select_NoYes_1'), findTestData('Allowances').getValue(19, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Auto Proportion Leaver'), findTestData('Allowances').getValue(19, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_No Yes_2'), findTestData('Allowances').getValue(20, row))
	
	WebUI.selectOptionByIndex(findTestObject('Object Repository/Add Records/Add Records - Allowances/Permanent Variable Both'), findTestData('Allowances').getValue(20, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_Please select Permanent (1)'), findTestData('Allowances').getValue(
            //21, row))
	
	WebUI.selectOptionByIndex(findTestObject('Object Repository/Add Records/Add Records - Allowances/Input Type'), findTestData('Allowances').getValue(
            21, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_Cash Hours Units Hours'), findTestData('Allowances').getValue(
           //22, row))
	
	WebUI.setText(findTestObject('Add Records/Add Records - Allowances/PercentageValue'), findTestData('Allowances').getValue(
           22, row))

    //WebUI.setText(findTestObject('Temp/input_PercentageValue0'), findTestData('Allowances').getValue(23, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Percentage of Acc'), findTestData('Allowances').getValue(
           23, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_Please selectEA01HA01 E'), findTestData('Allowances').getValue(
            //24, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Pay Basic Hours'), findTestData('Allowances').getValue(
            24, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_No Yes_3'), findTestData('Allowances').getValue(25, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Court Order Eligibility'), findTestData('Allowances').getValue(25, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_No  Yes_2'), findTestData('Allowances').getValue(26, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Financial summary exemption'), findTestData('Allowances').getValue(26, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_No  Yes_3'), findTestData('Allowances').getValue(27, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Pay during holidays'), findTestData('Allowances').getValue(27, row))

    //WebUI.selectOptionByIndex(findTestObject('Object Repository/Allow/select_No Yes'), findTestData('Allowances').getValue(
            //28, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Pay if suspended'), findTestData('Allowances').getValue(
            28, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_No  Yes_4'), findTestData('Allowances').getValue(29, row))
	
	WebUI.setText(findTestObject('Add Records/Add Records - Allowances/Clear Down Tax Period'), findTestData('Allowances').getValue(29, row))

    //WebUI.setText(findTestObject('Temp/input_ClearDownTaxPeriod0 (1)'), findTestData('Allowances').getValue(30, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Suppress Warning'), findTestData('Allowances').getValue(30, row))
	
    //WebUI.selectOptionByIndex(findTestObject('Temp/select_NoYes_2'), findTestData('Allowances').getValue(31, row))
	
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Lower Limit Option'), findTestData('Allowances').getValue(31, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_Pay the Calculated amou'), findTestData('Allowances').getValue(
            //32, row))
	
	WebUI.setText(findTestObject('Add Records/Add Records - Allowances/Lower Limit Value'), findTestData('Allowances').getValue(
            32, row))

    //WebUI.setText(findTestObject('Temp/input_LowerLimitValue0'), findTestData('Allowances').getValue(33, row))
	 
	WebUI.selectOptionByIndex(findTestObject('Add Records/Add Records - Allowances/Upper Limit Option'), findTestData('Allowances').getValue(33, row))

    //WebUI.selectOptionByIndex(findTestObject('Temp/select_Pay the Calculated amou_1'), findTestData('Allowances').getValue(
            //34, row))
	 
	WebUI.setText(findTestObject('Add Records/Add Records - Allowances/Upper Limit Value'), findTestData('Allowances').getValue(
            34, row))

    //WebUI.setText(findTestObject('Temp/input_UpperLimitValue0'), findTestData('Allowances').getValue(35, row))
	 
	WebUI.setText(findTestObject('Add Records/Add Records - Allowances/Reference Code'), findTestData('Allowances').getValue(35, row))

	WebUI.delay(3)
			
	WebUI.click(findTestObject('Object Repository/Add Records/Add Records - Allowances/Save'))
	
    WebUI.delay(7) 
	
}

