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

WebUI.navigateToUrl('http://mauw2k8sdworx:81/')

WebUI.setText(findTestObject('Temp/input_Username (18)'), 'dev@sdworx.com')

WebUI.setText(findTestObject('Temp/input_Password (19)'), '1234')

WebUI.click(findTestObject('Temp/input_Submit (19)'))

WebUI.click(findTestObject('Temp/h4_YOUR SETUP (12)'))

WebUI.click(findTestObject('Temp/button_Your Setup (8)'))

WebUI.click(findTestObject('Temp/a_Your Pay  Benefits (4)'))

WebUI.click(findTestObject('Temp/a_Deduction'))

WebUI.maximizeWindow()

WebUI.waitForPageLoad(3)

for (def row = 1; row <= findTestData('AddDeduction').getRowNumbers(); row++) {

WebUI.click(findTestObject('Temp/a_Add Deduction'))

WebUI.setText(findTestObject('Temp/input_DeductionNumber0'), findTestData('AddDeduction').getValue(1, row))

WebUI.setText(findTestObject('Temp/input_DeductionShortDesc0'), findTestData('AddDeduction').getValue(2, row))

WebUI.setText(findTestObject('Temp/input_DeductionLongDesc0'), findTestData('AddDeduction').getValue(3, row))

WebUI.selectOptionByIndex(findTestObject('Temp/select_Please Select Permanent'), findTestData('AddDeduction').getValue(4, row))

WebUI.selectOptionByIndex(findTestObject('Temp/select_CashHours - Eligibility'), findTestData('AddDeduction').getValue(5, row))

WebUI.selectOptionByIndex(findTestObject('Temp/select_Yes No'), findTestData('AddDeduction').getValue(6, row))

WebUI.setText(findTestObject('Temp/input_DeductionPercentage0'),findTestData('AddDeduction').getValue(7, row))

WebUI.selectOptionByIndex(findTestObject('Temp/select_Please Select  None EA0'), findTestData('AddDeduction').getValue(8, row))

WebUI.selectOptionByIndex(findTestObject('Temp/select_From Net Pre Tax  NI Pr'), findTestData('AddDeduction').getValue(9, row))

WebUI.setText(findTestObject('Temp/input_CharityAidRef0'), findTestData('AddDeduction').getValue(10, row))

WebUI.selectOptionByIndex(findTestObject('Temp/select_None Increasing  Reduci'), findTestData('AddDeduction').getValue(11, row))

WebUI.selectOptionByIndex(findTestObject('Temp/select_No Yes'), findTestData('AddDeduction').getValue(12, row))

WebUI.selectOptionByIndex(findTestObject('Temp/select_No Yes_1'), findTestData('AddDeduction').getValue(13, row))

WebUI.selectOptionByIndex(findTestObject('Temp/select_Yes No_1'), findTestData('AddDeduction').getValue(14, row))

WebUI.setText(findTestObject('Temp/input_ClearDownTaxPeriod0'), findTestData('AddDeduction').getValue(15, row))

WebUI.click(findTestObject('Temp/button_Save (6)'))

}

//WebUI.closeBrowser()

