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

WebUI.openBrowser('https://implementationtoolkit.azurewebsites.net/')

WebUI.navigateToUrl('https://implementationtoolkit.azurewebsites.net/')

WebUI.setText(findTestObject('null'), 'Sanjeev@sdworx.com')

WebUI.click(findTestObject('null'))

WebUI.delay(5)

WebUI.mouseOver(findTestObject('null'))

WebUI.delay(5)

WebUI.click(findTestObject('null'))

WebUI.delay(5)

//WebUI.mouseOver(findTestObject('null'))

//WebUI.delay(5)

WebUI.mouseOver(findTestObject('null'))

//WebUI.click(findTestObject('Non Prod/a_Your Pay  Benefits'))

WebUI.delay(5)


WebUI.click(findTestObject('null'))
WebUI.selectOptionByValue(findTestObject('Screens/Pension Details/select_000002 - Nadia Payroll0'), 'c4393963-2aa8-4342-93fd-08ed9e25aa4f', 
    true)

WebUI.delay(2)

WebUI.click(findTestObject('Screens/Pension Details/a_Add Pension'))

WebUI.delay(2)

for (def row = 1; row <= findTestData('Pension Details').getRowNumbers(); row++) {

WebUI.setText(findTestObject('Screens/Pension Details/input_SchemeNumber0'), findTestData('Pension Details').getValue(1, row))

WebUI.setText(findTestObject('Screens/Pension Details/input_ShortDescription0'), findTestData('Pension Details').getValue(2, row))

WebUI.setText(findTestObject('Screens/Pension Details/input_LongDescription0'), findTestData('Pension Details').getValue(3, row))

WebUI.selectOptionByIndex(findTestObject('Screens/Pension Details/select_Please SelectCashPercen'), findTestData('Pension Details').getValue(4, row))

WebUI.setText(findTestObject('Screens/Pension Details/input_SchemeYTDClearDownPeriod'), findTestData('Pension Details').getValue(5, row))

//WebUI.click(findTestObject('Screens/Pension Details/div_ActionHideScheme Number Sh'))

WebUI.selectOptionByIndex(findTestObject('Screens/Pension Details/select_Please SelectNon-tax Ap'), findTestData('Pension Details').getValue(6, row))

WebUI.selectOptionByIndex(findTestObject('Screens/Pension Details/select_No  Yes'), findTestData('Pension Details').getValue(7, row))


//WebUI.click(findTestObject('Screens/Pension Details/div_ActionHideScheme Number Sh'))

WebUI.selectOptionByValue(findTestObject('Screens/Pension Details/select_Please select001-BASIC'), '52bd1ff0-1c2b-47c1-a974-d2c7c5ae04ae', 
    true)

WebUI.selectOptionByValue(findTestObject('Screens/Pension Details/select_Please select001-BASIC _1'), '52bd1ff0-1c2b-47c1-a974-d2c7c5ae04ae', 
    true)

//WebUI.click(findTestObject('Screens/Pension Details/div_ActionHideScheme Number Sh'))

WebUI.selectOptionByValue(findTestObject('Screens/Pension Details/select_Please select001-BASIC _2'), '52bd1ff0-1c2b-47c1-a974-d2c7c5ae04ae', 
    true)

WebUI.selectOptionByIndex(findTestObject('Screens/Pension Details/select_Please SelectEntitled J'), findTestData('Pension Details').getValue(11, row))


//WebUI.click(findTestObject('Screens/Pension Details/div_ActionHideScheme Number Sh'))

WebUI.setText(findTestObject('Screens/Pension Details/input_StandardEEValue0'), findTestData('Pension Details').getValue(12, row))


WebUI.setText(findTestObject('Screens/Pension Details/input_StandardERValue0'), findTestData('Pension Details').getValue(13, row))

//WebUI.setText(findTestObject('Screens/Pension Details/input_StandardEEValue0'), findTestData('Pension Details').getValue(14, row))

//WebUI.setText(findTestObject('Screens/Pension Details/input_StandardERValue0'), findTestData('Pension Details').getValue(15, row))

//WebUI.click(findTestObject('Screens/Pension Details/div_ActionHideScheme Number Sh'))

//WebUI.click(findTestObject('Screens/Pension Details/div_ActionHideScheme Number Sh'))

WebUI.selectOptionByIndex(findTestObject('Screens/Pension Details/select_Please SelectPensionabl'), findTestData('Pension Details').getValue(14, row))

WebUI.selectOptionByIndex(findTestObject('Screens/Pension Details/select_Please SelectEA01EA02EA'), findTestData('Pension Details').getValue(15, row))

WebUI.selectOptionByIndex(findTestObject('Screens/Pension Details/select_Please SelectAnnual Dif'), findTestData('Pension Details').getValue(16, row))

//WebUI.click(findTestObject('Screens/Pension Details/div_ActionHideScheme Number Sh'))

WebUI.selectOptionByIndex(findTestObject('Screens/Pension Details/select_Please SelectAnnual Dif_1'), findTestData('Pension Details').getValue(17, row))

//WebUI.click(findTestObject('Screens/Pension Details/div_ActionHideScheme Number Sh'))


WebUI.click(findTestObject('Screens/Pension Details/button_Save'))

}

//WebUI.click(findTestObject('Screens/Pension Details/td_dataTables_empty'))

//WebUI.click(findTestObject('Screens/Pension Details/i_fa fa-floppy-o'))

//WebUI.click(findTestObject('Screens/Pension Details/div_media (min-width 1200px)'))

//WebUI.click(findTestObject('Screens/Pension Details/div_ActionHideScheme Number Sh_1'))

