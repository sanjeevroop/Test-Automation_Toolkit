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

WebUI.openBrowser('')

WebUI.navigateToUrl('https://implementationtoolkit.azurewebsites.net/')
WebUI.setText(findTestObject('Screens/Pension Rules/input_loginfmt'), 'Sanjeev@sdworx.com')

WebUI.click(findTestObject('Screens/Pension Rules/input_idSIButton9'))

WebUI.delay(5)

WebUI.mouseOver(findTestObject('null'))

WebUI.delay(5)

WebUI.click(findTestObject('null'))

WebUI.delay(5)

WebUI.mouseOver(findTestObject('null'))

WebUI.click(findTestObject('Screens/Pension Rules/a_Pension Rules'))

WebUI.delay(5)

WebUI.selectOptionByValue(findTestObject('Screens/Pension Rules/select_000002 - Nadia Payroll0'), '3893c9a7-81e1-420f-994f-cdf21535ad59', 
    true)

for (def row = 1; row <= findTestData('Pension Details').getRowNumbers(); row++) {

WebUI.selectOptionByIndex(findTestObject('Screens/Pension Rules/select_Please SelectEA01EA02EA'), findTestData('Pension Rules').getValue(1, row))

WebUI.selectOptionByValue(findTestObject('Screens/Pension Rules/select_Please Select01'), 'D9A32A60-597C-4C63-A7E1-69D20D90239A', 
    true)

WebUI.selectOptionByValue(findTestObject('Screens/Pension Rules/select_Please Select01_1'), 'D9A32A60-597C-4C63-A7E1-69D20D90239A', 
    true)

WebUI.selectOptionByValue(findTestObject('Screens/Pension Rules/select_Please Select32 - TEST'), 'ba70f23f-1a0a-4d8a-a382-09bbdfccc0b5', 
    true)

WebUI.selectOptionByValue(findTestObject('Screens/Pension Rules/select_Please Select32 - TEST_1'), 'ba70f23f-1a0a-4d8a-a382-09bbdfccc0b5', 
    true)

WebUI.selectOptionByValue(findTestObject('Screens/Pension Rules/select_Please Select32 - TEST_2'), 'ba70f23f-1a0a-4d8a-a382-09bbdfccc0b5', 
    true)

WebUI.selectOptionByValue(findTestObject('Screens/Pension Rules/select_Please Select32 - TEST_3'), 'ba70f23f-1a0a-4d8a-a382-09bbdfccc0b5', 
    true)

WebUI.selectOptionByValue(findTestObject('Screens/Pension Rules/select_Please Select88 - test2'), '1aab9055-5e81-4deb-9e79-b3786f1f0562', 
    true)

WebUI.selectOptionByValue(findTestObject('Screens/Pension Rules/select_Please Select88 - test2_1'), 'bd7e9f37-3689-42b3-b883-d91a91fe5350', 
    true)

WebUI.selectOptionByValue(findTestObject('Screens/Pension Rules/select_Please Select88 - test2_2'), '1aab9055-5e81-4deb-9e79-b3786f1f0562', 
    true)

WebUI.selectOptionByValue(findTestObject('Screens/Pension Rules/select_Please Select88 - test2_3'), '1aab9055-5e81-4deb-9e79-b3786f1f0562', 
    true)

WebUI.setText(findTestObject('Screens/Pension Rules/input_StagingDate'), findTestData('Pension Rules').getValue(12, row))

WebUI.setText(findTestObject('Screens/Pension Rules/input_ReEnrolmentDate'), findTestData('Pension Rules').getValue(13, row))

WebUI.selectOptionByIndex(findTestObject('Screens/Pension Rules/select_NoNot in a SchemeYes'), findTestData('Pension Rules').getValue(14, row))

WebUI.selectOptionByIndex(findTestObject('Screens/Pension Rules/select_Please Select0123456789'), findTestData('Pension Rules').getValue(15, row))

WebUI.selectOptionByIndex(findTestObject('Screens/Pension Rules/select_NoYes'), findTestData('Pension Rules').getValue(16, row))

WebUI.selectOptionByIndex(findTestObject('Screens/Pension Rules/select_Please Select0123456789_1'), findTestData('Pension Rules').getValue(17, row))

WebUI.selectOptionByIndex(findTestObject('Screens/Pension Rules/select_YesNo'), findTestData('Pension Rules').getValue(18, row))

WebUI.selectOptionByIndex(findTestObject('Screens/Pension Rules/select_0123456789101112'), findTestData('Pension Rules').getValue(19, row))

WebUI.click(findTestObject('Screens/Pension Rules/button_Save'))

}

//WebUI.closeBrowser()

