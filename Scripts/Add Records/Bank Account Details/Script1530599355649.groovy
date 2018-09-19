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

WebUI.setText(findTestObject('Screens/Bank Account Details/input_Username'), 'dev@sdworx.com')

WebUI.setText(findTestObject('Screens/Bank Account Details/input_Password'), '1234')

WebUI.click(findTestObject('Screens/Bank Account Details/input_Submit'))

WebUI.maximizeWindow()

WebUI.click(findTestObject('Screens/Bank Account Details/h4_YOUR SETUP'))

WebUI.click(findTestObject('Screens/Bank Account Details/button_Your Setup'))

WebUI.click(findTestObject('Screens/Bank Account Details/a_Your Payroll'))

WebUI.click(findTestObject('Screens/Bank Account Details/a_Payroll Basic Details'))

WebUI.selectOptionByValue(findTestObject('Screens/Bank Account Details/select_testDev Payroll 1WebTes'), 'e865745f-ca2b-4af8-a580-50285c6b5440', 
    true)

WebUI.click(findTestObject('Screens/Bank Account Details/button_Next'))

//WebUI.click(findTestObject('Screens/Bank Account Details/label_Bank Account Name'))

WebUI.setText(findTestObject('Screens/Bank Account Details/input_PayrollCompany.BankAccou'), findTestData('Bank Account Details').getValue(1, 1))

WebUI.setText(findTestObject('Screens/Bank Account Details/input_PayrollCompany.SortCodeO'), findTestData('Bank Account Details').getValue(2, 1))

WebUI.setText(findTestObject('Screens/Bank Account Details/input_PayrollCompany.SortCodeT'), findTestData('Bank Account Details').getValue(3, 1))

WebUI.setText(findTestObject('Screens/Bank Account Details/input_PayrollCompany.SortCodeT_1'), findTestData('Bank Account Details').getValue(4, 1))

//WebUI.click(findTestObject('Screens/Bank Account Details/label_Bank Account Number'))

WebUI.setText(findTestObject('Screens/Bank Account Details/input_PayrollCompany.BankAccou_1'), findTestData('Bank Account Details').getValue(5, 1))

WebUI.setText(findTestObject('Screens/Bank Account Details/input_PayrollCompany.BACSUserN'), findTestData('Bank Account Details').getValue(6, 1))

WebUI.setText(findTestObject('Screens/Bank Account Details/input_PayrollCompany.BACSLimit'), findTestData('Bank Account Details').getValue(7, 1))

WebUI.click(findTestObject('Screens/Bank Account Details/button_Next_1'))

//WebUI.closeBrowser()

