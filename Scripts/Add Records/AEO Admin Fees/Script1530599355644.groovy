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

WebUI.setText(findTestObject('Screens/AEO Admin Fees/input_Username'), 'dev@sdworx.com')

WebUI.setText(findTestObject('Screens/AEO Admin Fees/input_Password'), '1234')

WebUI.click(findTestObject('Screens/AEO Admin Fees/input_Submit (1)'))

WebUI.click(findTestObject('Screens/AEO Admin Fees/h4_YOUR SETUP'))

WebUI.click(findTestObject('Screens/AEO Admin Fees/button_Your Setup'))

WebUI.click(findTestObject('Screens/AEO Admin Fees/a_Your Payroll'))

WebUI.click(findTestObject('Screens/AEO Admin Fees/a_Payroll Basic Details'))

WebUI.selectOptionByValue(findTestObject('Screens/AEO Admin Fees/select_testDev Payroll 1WebTes'), 'e865745f-ca2b-4af8-a580-50285c6b5440', 
    true)

WebUI.click(findTestObject('Screens/AEO Admin Fees/button_Next'))

WebUI.click(findTestObject('Screens/AEO Admin Fees/button_Next_1'))

WebUI.click(findTestObject('Screens/AEO Admin Fees/button_Next_2'))

WebUI.selectOptionByIndex(findTestObject('Screens/AEO Admin Fees/select_Please selectNo admin c'), findTestData('AEO Admin Fees').getValue(1, 1))

WebUI.click(findTestObject('Screens/AEO Admin Fees/div_col-md-8 inputGroupContain'))

WebUI.setText(findTestObject('Screens/AEO Admin Fees/input_PayrollCompany.CSA'), findTestData('AEO Admin Fees').getValue(2, 1))

WebUI.click(findTestObject('Screens/AEO Admin Fees/div_col-md-8 inputGroupContain'))

WebUI.setText(findTestObject('Screens/AEO Admin Fees/input_PayrollCompany.Maintenan'), findTestData('AEO Admin Fees').getValue(3, 1))

WebUI.click(findTestObject('Screens/AEO Admin Fees/label_Scottish Arrestment fee'))

WebUI.setText(findTestObject('Screens/AEO Admin Fees/input_PayrollCompany.Judgement'), findTestData('AEO Admin Fees').getValue(4, 1))

WebUI.click(findTestObject('Screens/AEO Admin Fees/div_col-md-8 inputGroupContain'))

WebUI.setText(findTestObject('Screens/AEO Admin Fees/input_PayrollCompany.Fines'), findTestData('AEO Admin Fees').getValue(5, 1))

WebUI.click(findTestObject('Screens/AEO Admin Fees/div_col-md-8 inputGroupContain'))

WebUI.setText(findTestObject('Screens/AEO Admin Fees/input_PayrollCompany.DWP'), findTestData('AEO Admin Fees').getValue(6, 1))

WebUI.click(findTestObject('Screens/AEO Admin Fees/label_Deductions from Earnings'))

WebUI.setText(findTestObject('Screens/AEO Admin Fees/input_PayrollCompany.ScottishA'), findTestData('AEO Admin Fees').getValue(7, 1))

WebUI.click(findTestObject('Screens/AEO Admin Fees/div_col-md-8 inputGroupContain'))

WebUI.setText(findTestObject('Screens/AEO Admin Fees/input_PayrollCompany.ScottishA_1'), findTestData('AEO Admin Fees').getValue(8, 1))

WebUI.click(findTestObject('Screens/AEO Admin Fees/span_glyphicon glyphicon-send'))

//WebUI.closeBrowser()

